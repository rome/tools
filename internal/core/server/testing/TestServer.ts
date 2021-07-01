/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {Reporter, ReporterNamespace} from "@internal/cli-reporter";
import {
	DIAGNOSTIC_CATEGORIES,
	Diagnostic,
	DiagnosticLocation,
	appendAdviceToDiagnostic,
	deriveDiagnosticFromError,
	descriptions,
	diagnosticLocationToMarkupFilelink,
	equalCategoryNames,
	getDiagnosticsFromError,
} from "@internal/diagnostics";
import {Server, ServerRequest, TestRef} from "@internal/core";
import {DiagnosticsPrinter} from "@internal/cli-diagnostics";
import {humanizeNumber} from "@internal/numbers";
import {AnyBridge, isBridgeEndDiagnosticsError} from "@internal/events";
import {CoverageCollector} from "@internal/v8";
import {ManifestDefinition} from "@internal/codec-js-manifest";
import {
	CoverageDirectory,
	TestServerRunnerConstructorOptions,
	TestServerRunnerOptions,
} from "./types";
import {
	formatPercent,
	percentInsideCoverageDirectory,
	sortMapKeys,
} from "./utils";
import {Markup, StaticMarkup, joinMarkup, markup} from "@internal/markup";
import {MAX_WORKER_COUNT} from "@internal/core/common/constants";
import net = require("net");
import {FocusedTest} from "@internal/core/worker/test/TestWorkerFile";
import {SourceMapConsumerCollection} from "@internal/codec-source-map";
import {VoidCallback} from "@internal/typescript-helpers";
import Bundler from "@internal/core/server/bundler/Bundler";
import {
	AbsoluteFilePath,
	AbsoluteFilePathMap,
	AbsoluteFilePathSet,
	createPath,
} from "@internal/path";
import TestServerWorker from "@internal/core/server/testing/TestServerWorker";
import TestServerFile from "@internal/core/server/testing/TestServerFile";
import {ExtendedMap} from "@internal/collections";
import {promiseAllFrom} from "@internal/async";
import {TestFileRef} from "@internal/core/worker/types";
import {getExecuteMainFilename} from "@internal/core/worker/utils/executeMain";

function grammarNumberTests(num: number): StaticMarkup {
	return markup`<grammarNumber plural="tests" singular="test">${String(num)}</grammarNumber>`;
}

function getProgressTestRefText(ref: TestRef) {
	return markup`${ref.path}: ${ref.testName}`;
}

function findAvailablePort(): Promise<number> {
	return new Promise((resolve, reject) => {
		// When you create a server without specifying a port then the OS will choose a port number for you!
		const server = net.createServer();
		server.unref();
		server.on("error", reject);
		server.listen(
			undefined,
			() => {
				const address = server.address();
				if (address == null || typeof address === "string") {
					throw new Error("Invalid address value");
				}

				server.close(() => {
					resolve(address.port);
				});
			},
		);
	});
}

type TestProgress = {
	teardown: VoidCallback;
};

function refToKey(ref: TestRef): string {
	return `${ref.path.join()}: ${ref.testName}`;
}

export default class TestServer {
	constructor(opts: TestServerRunnerConstructorOptions) {
		this.paths = opts.paths;
		this.reporter = opts.request.reporter;
		this.server = opts.request.server;
		this.logger = this.server.logger.namespace(markup`TestServerRunner`);
		this.request = opts.request;
		this.options = opts.options;

		this.ignoreBridgeEndError = new Set();

		this.coverageCollector = new CoverageCollector();

		this.progress = {
			totalTests: 0,
			failedTests: 0,
			passedTests: 0,
			startedTests: 0,
			finishedTests: 0,
			updatedSnapshots: 0,
			deletedSnapshots: 0,
			createdSnapshots: 0,
			updatedInlineSnapshots: 0,
		};

		this.files = new AbsoluteFilePathMap();
		this.focusedTests = [];
		this.testFilesStack = [];
		this.runningTests = new ExtendedMap("runningTests");
		this.needsSnapshotUpdate = false;

		this.sourceMaps = new SourceMapConsumerCollection();
		this.printer = opts.request.createDiagnosticsPrinter({
			processor: this.request.createDiagnosticsProcessor({
				origin: {
					entity: "TestServer",
				},
				sourceMaps: this.sourceMaps,
				normalizeOptions: {
					defaultTags: {
						// All test diagnostics are important and cannot risk being deduped
						unique: true,
					},
				},
			}),
		});

		this.printer.processor.guaranteedTruncationEvent.subscribe(() => {
			// TODO: Notify all test workers that they should no longer send us diagnostics
			// We will however still need to receive an event that a diagnostic was created so we can increment
			// our own truncated count. We should also send over the filter to mark those correctly too.
		});
	}

	public sourceMaps: SourceMapConsumerCollection;
	public printer: DiagnosticsPrinter;
	public coverageCollector: CoverageCollector;
	public focusedTests: FocusedTest[];
	public options: TestServerRunnerOptions;
	public files: AbsoluteFilePathMap<TestServerFile>;

	private request: ServerRequest;
	private reporter: Reporter;
	private logger: ReporterNamespace;
	private paths: AbsoluteFilePathSet;
	private server: Server;
	private ignoreBridgeEndError: Set<AnyBridge>;

	public testFilesStack: AbsoluteFilePath[];

	private needsSnapshotUpdate: boolean;
	private runningTests: ExtendedMap<
		string,
		{
			ref: TestRef;
			timeout: undefined | NodeJS.Timeout;
		}
	>;

	public progress: {
		passedTests: number;
		failedTests: number;
		totalTests: number;
		startedTests: number;
		finishedTests: number;
		updatedInlineSnapshots: number;
		updatedSnapshots: number;
		deletedSnapshots: number;
		createdSnapshots: number;
	};

	public handlePossibleBridgeError(err: Error, bridge: AnyBridge) {
		const diagnostics = getDiagnosticsFromError(err);
		if (diagnostics === undefined) {
			throw err;
		}

		if (
			isBridgeEndDiagnosticsError(err) &&
			this.ignoreBridgeEndError.has(bridge)
		) {
			return;
		}

		this.printer.processor.addDiagnostics(diagnostics);
	}

	public addDiagnostic(diagnostic: Diagnostic, ref?: TestFileRef) {
		if (
			diagnostic.label === undefined &&
			ref !== undefined &&
			ref.testName !== undefined
		) {
			diagnostic = {
				...diagnostic,
				label: markup`${ref.testName}`,
			};
		}

		if (
			equalCategoryNames(
				diagnostic.description.category,
				DIAGNOSTIC_CATEGORIES["tests/snapshots/incorrect"],
			)
		) {
			this.needsSnapshotUpdate = true;
		}

		// For test diagnostics that don't explicitly refer to the test file it's a part of, push on some clarifying advice to make it obvious
		// how to find the originating test
		if (ref !== undefined) {
			// Normalize diagnostic to resolve source maps for stacktrace advice
			diagnostic = this.printer.processor.normalizer.normalizeDiagnostic(
				diagnostic,
			);

			let includeTestDeclaration = true;

			// If there's a stacktrace and the first frame points to the test file itself then don't show the
			// test declaration or else it's just noise
			const {advice} = diagnostic.description;
			if (
				advice.length > 0 &&
				advice[0].type === "stacktrace" &&
				advice[0].frames?.[0].path?.equal(ref.path)
			) {
				includeTestDeclaration = false;
			}

			if (includeTestDeclaration) {
				let callsiteLocation: undefined | DiagnosticLocation;

				if (ref.testName !== undefined) {
					callsiteLocation = this.files.assert(ref.path).getTestCallsiteLocation(
						ref.testName,
					);

					// Resolve source maps
					callsiteLocation = this.printer.processor.normalizer.normalizeLocation(
						callsiteLocation,
					);
				}

				if (callsiteLocation === undefined) {
					diagnostic = appendAdviceToDiagnostic(
						diagnostic,
						[
							{
								type: "log",
								category: "info",
								text: markup`Originated from test file <emphasis>${ref.path}</emphasis>`,
							},
						],
					);
				} else {
					let text = markup`Test declared at <emphasis>${diagnosticLocationToMarkupFilelink(
						callsiteLocation,
					)}</emphasis>`;

					if (!callsiteLocation.path.equal(ref.path)) {
						// Add original test path if the actual test() declaration callsite wasn't in the test file
						// This will occur when using helpers
						text = markup`${text} from test file <emphasis>${ref.path}</emphasis>`;
					}

					diagnostic = appendAdviceToDiagnostic(
						diagnostic,
						[
							{
								type: "log",
								category: "info",
								text,
							},
							// TODO: Decide if we might want a frame? It's pretty noisy
							/*{
							type: "frame",
							location: callsiteLocation,
						},*/
						],
					);
				}
			}
		}

		this.printer.processor.addDiagnostic(diagnostic);
	}

	private async setupWorkers(): Promise<TestServerWorker[]> {
		const workers: Promise<TestServerWorker>[] = [];
		for (let i = 0; i < MAX_WORKER_COUNT; i++) {
			const inspectorPort = await findAvailablePort();

			const container = await this.server.workerManager.spawnWorkerUnsafe({
				type: "test-runner",
				ghost: true,
				inspectorPort,
				env: this.request.client.env,
			});

			const worker = new TestServerWorker({
				runner: this,
				container,
				server: this.server,
				request: this.request,
			});
			workers.push(worker.init().then(() => worker));
		}
		return Promise.all(workers);
	}

	public async init() {
		const workers = await this.setupWorkers();
		const fileQueue = await this.bundleTests();
		await this.prepareTests(workers, fileQueue);
		await this.runTests(workers);
		this.throwPrinter();
	}

	private async bundleTests(): Promise<TestServerFile[]> {
		const fileQueue: TestServerFile[] = [];

		const bundler = new Bundler(
			this.request,
			this.request.getBundlerConfigFromFlags({
				mocks: true,
			}),
		);
		for (const [path, bundle] of await bundler.bundleMultiple(
			Array.from(this.paths),
			{
				deferredSourceMaps: true,
			},
		)) {
			if (this.options.sourceMaps) {
				const sourceMap = bundle.entry.sourceMap.map;
				const consumer = sourceMap.toConsumer();
				//this.coverageCollector.addSourceMap(path.join(), code, consumer);
				this.sourceMaps.add(createPath(getExecuteMainFilename(path)), consumer);
			}

			const ref = this.server.projectManager.getFileReference(path);
			const file = new TestServerFile({
				ref,
				bundle,
				runner: this,
				request: this.request,
			});
			this.files.set(path, file);
			fileQueue.push(file);
			this.testFilesStack.push(path);
		}

		return fileQueue;
	}

	private async prepareTests(
		workers: TestServerWorker[],
		fileQueue: TestServerFile[],
	) {
		const progress = this.reporter.progress({
			title: markup`Preparing test files`,
		});
		progress.setTotal(this.paths.size);
		await promiseAllFrom(
			workers,
			(worker) => worker.prepareAll(progress, fileQueue),
		);
		progress.end();

		// If we have focused tests, clear the pending queues and populate it with only ours
		if (this.hasFocusedTests()) {
			for (const file of this.files.values()) {
				file.clearPendingTests();
			}

			for (const {ref} of this.focusedTests) {
				this.files.assert(ref.path).addPendingTest(ref.testName);
			}
		}
	}

	private async runTests(workers: TestServerWorker[]) {
		const runProgress = this.setupRunProgress(workers);
		await promiseAllFrom(workers, (worker) => worker.run());
		await promiseAllFrom(workers, (worker) => worker.thread.worker.terminate());
		runProgress.teardown();
	}

	public hasFocusedTests(): boolean {
		return this.focusedTests.length > 0;
	}

	private getTotalTests(): number {
		if (this.hasFocusedTests()) {
			return this.focusedTests.length;
		} else {
			return this.progress.totalTests;
		}
	}

	private onTestStart(
		worker: TestServerWorker,
		ref: TestRef,
		timeoutMs: undefined | number,
	) {
		this.progress.startedTests++;

		let timeout = undefined;
		if (timeoutMs !== undefined) {
			timeout = setTimeout(
				() => {
					// TODO This will kill the whole worker, maybe it's possible to just terminate the current test? Throw an error, see if the next test was ran, or else terminate completely
					this.server.fatalErrorHandler.wrapPromise(
						worker.handleTimeout(`${String(timeoutMs)}ms`),
					);
				},
				timeoutMs,
			);
		}

		const key = refToKey(ref);
		this.logger.info(markup`Running test ${key}`);
		this.runningTests.set(
			key,
			{
				ref,
				timeout,
			},
		);
	}

	public onTestFound(ref: TestRef, callsiteLocation: DiagnosticLocation) {
		const file = this.files.assert(ref.path);
		file.addFoundTest(ref.testName, callsiteLocation);
		file.addPendingTest(ref.testName);
		this.progress.totalTests++;
	}

	private onTestFinished(ref: TestRef, success: boolean) {
		const key = refToKey(ref);
		const running = this.runningTests.assert(key);

		this.logger.info(markup`Finished test ${key}`);
		if (running.timeout !== undefined) {
			clearTimeout(running.timeout);
		}
		this.runningTests.delete(key);
		this.progress.finishedTests++;

		if (success) {
			this.progress.passedTests++;
		} else {
			this.progress.failedTests++;
		}
	}

	private setupRunProgress(workers: TestServerWorker[]): TestProgress {
		const progress = this.request.reporter.progress({
			persistent: true,
			title: markup`Running tests`,
		});
		progress.setTotal(this.getTotalTests());

		for (const worker of workers) {
			const {bridge} = worker;
			const ourRunningTests: Set<string> = new Set();

			bridge.endEvent.subscribe((error) => {
				// Cancel all currently running tests
				const cancelTests: TestRef[] = [];

				for (const key of ourRunningTests) {
					const test = this.runningTests.get(key);
					if (test !== undefined) {
						cancelTests.push(test.ref);
					}
				}

				for (const ref of cancelTests) {
					this.onTestFinished(ref, false);

					if (cancelTests.length === 1) {
						// If we only have one test to cancel then let's only point the bridge error to this test
						this.ignoreBridgeEndError.add(bridge);

						const errDiag = deriveDiagnosticFromError(
							error,
							{
								label: markup`${ref.testName}`,
								path: ref.path,
								description: {
									category: DIAGNOSTIC_CATEGORIES["tests/failure"],
								},
								internal: false,
							},
						);

						this.printer.processor.addDiagnostic({
							...errDiag,
							description: {
								...errDiag.description,
								// We don't care about the advice
								advice: [
									{
										type: "log",
										category: "info",
										text: markup`Was executing test file <emphasis>${ref.path}</emphasis>`,
									},
								],
							},
						});
					} else {
						this.printer.processor.addDiagnostic({
							label: markup`${ref.testName}`,
							description: descriptions.TESTS.CANCELLED,
							location: {
								path: ref.path,
							},
						});
					}
				}
			});

			bridge.events.testStart.subscribe((data) => {
				const key = refToKey(data.ref);
				ourRunningTests.add(key);
				this.onTestStart(worker, data.ref, data.timeout);
				progress.pushText(getProgressTestRefText(data.ref), key);
			});

			bridge.events.testFinish.subscribe((data) => {
				this.onTestFinished(data.ref, data.success);
				progress.popText(refToKey(data.ref));
				progress.tick();
			});
		}

		return {
			teardown() {
				progress.end();
			},
		};
	}

	private printCoverageReport(isError: boolean) {
		const {reporter, server, coverageCollector} = this;

		if (isError && this.options.showAllCoverage) {
			// Only show coverage for errors when --show-all-coverage has been specified
			return;
		}

		if (!this.options.coverage) {
			return;
		}

		reporter.info(markup`Generating coverage`);

		// Fetch coverage entries
		const files = coverageCollector.generate();
		if (files.length === 0) {
			return;
		}

		reporter.heading(markup`Code coverage`);

		// Get the packages associated with all the ran tests, we will filter code coverage to those packages only
		const testedPackages: Set<undefined | ManifestDefinition> = new Set();
		for (const path of this.paths) {
			testedPackages.add(server.memoryFs.getOwnedManifest(path));
		}

		let root: CoverageDirectory = {
			name: undefined,
			directories: new Map(),
			files: new Map(),
		};

		let totalFiles = 0;

		// Turn the flat list of filenames into a directory tree
		for (const file of files) {
			const {path} = file;

			// Get the absolute filename
			const absolute = server.projectManager.maybeGetFilePathFromUID(path);
			if (absolute === undefined) {
				continue;
			}

			// Filter out untested packages
			const pkg = server.memoryFs.getOwnedManifest(absolute);
			if (testedPackages.has(pkg) === false) {
				continue;
			}

			// TODO maybe filter out test files too?

			// Track unfiltered files
			totalFiles++;

			const basename = path.getBasename();
			if (basename === undefined) {
				throw new Error("Should always be at least one element from a split()");
			}

			let target: CoverageDirectory = root;

			for (const part of path.getParentSegments()) {
				const existingDirectory = target.directories.get(part);
				if (existingDirectory === undefined) {
					const newDirectory = {
						name: part,
						directories: new Map(),
						files: new Map(),
					};
					target.directories.set(part, newDirectory);
					target = newDirectory;
				} else {
					target = existingDirectory;
				}
			}

			target.files.set(basename, file);
		}

		// Continuously merge all entries with only a single directory from the root
		while (root.directories.size === 1 && root.files.size === 0) {
			// Awkward way to get the first value out of the directories map...
			const newRoot = root.directories.values().next().value;
			root = {
				...newRoot,
				name: root.name !== undefined && newRoot.name !== undefined
					? `${root.name}/${newRoot.name}`
					: newRoot.name,
			};
		}

		const rows: Markup[][] = [];

		// If there's more than 15 files to show, and we don't have the explicit showAllCoverage flag
		// then truncate the output
		const showAllCoverage = this.options.showAllCoverage || totalFiles < 15;

		function buildRows(directory: CoverageDirectory, depth: number) {
			const name =
				directory.name === undefined ? "All files" : `${directory.name}/`;
			const directoryPercent = percentInsideCoverageDirectory(directory);

			rows.push([
				markup`${" ".repeat(depth)}<emphasis>${name}</emphasis>`,
				formatPercent(directoryPercent.functions),
				formatPercent(directoryPercent.branches),
				formatPercent(directoryPercent.lines),
			]);

			// Don't ever show anything deeper than a single level when showAllCoverage is off
			if (!showAllCoverage && depth > 0) {
				return;
			}

			const fileIndent = " ".repeat(depth + 1);
			for (const [name, file] of sortMapKeys(directory.files)) {
				let absolute = file.path;

				// Exchange any UIDs
				const absolutePath = server.projectManager.maybeGetFilePathFromUID(
					file.path,
				);
				if (absolutePath !== undefined) {
					absolute = absolutePath;
				}

				rows.push([
					markup`${fileIndent}<filelink target="${absolute}">${name}</filelink>`,
					formatPercent(file.functions.percent),
					formatPercent(file.branches.percent),
					formatPercent(file.lines.percent),
				]);
			}

			for (const subDirectory of sortMapKeys(directory.directories).values()) {
				buildRows(subDirectory, depth + 1);
			}
		}

		buildRows(root, 0);

		reporter.table(
			[markup`File`, markup`% Functions`, markup`% Branches`, markup`% Lines`],
			rows,
		);

		if (!showAllCoverage) {
			reporter.br();
			reporter.info(
				markup`Additional coverage information available. Refine the executed tests or add the <emphasis>--show-all-coverage</emphasis> flag`,
			);
		}

		reporter.hr();
	}

	private printFocusedTestWarning(reporter: Reporter) {
		const {focusedTests} = this;
		if (!this.hasFocusedTests()) {
			return;
		}

		const formattedFocusedTests = focusedTests.map(({ref, location}) => {
			const loc = this.printer.processor.normalizer.normalizeLocation(location);

			return markup`<emphasis>${ref.testName}</emphasis> at <emphasis>${diagnosticLocationToMarkupFilelink(
				loc,
			)}</emphasis>`;
		});

		if (focusedTests.length === 1) {
			reporter.warn(
				markup`Only ran the focused test ${formattedFocusedTests[0]}`,
			);
		} else {
			reporter.warn(
				markup`Only ran the following <number emphasis>${String(
					focusedTests.length,
				)}</number> focused ${grammarNumberTests(focusedTests.length)}`,
			);
			reporter.list(formattedFocusedTests);
		}

		const otherTotal = this.progress.totalTests - this.focusedTests.length;
		reporter.warn(
			markup`<number emphasis>${String(otherTotal)}</number> other ${grammarNumberTests(
				otherTotal,
			)} ignored`,
		);
	}

	private printSnapshotCounts(reporter: Reporter) {
		const {
			createdSnapshots,
			deletedSnapshots,
			updatedSnapshots,
			updatedInlineSnapshots,
		} = this.progress;

		let snapshotCounts: {
			inline: boolean;
			count: number;
			noun: string;
		}[] = [
			{inline: false, count: createdSnapshots, noun: "created"},
			{inline: false, count: updatedSnapshots, noun: "updated"},
			{inline: false, count: deletedSnapshots, noun: "deleted"},
			{inline: true, count: updatedInlineSnapshots, noun: "updated"},
		];
		snapshotCounts = snapshotCounts.filter(({count}) => count > 0);
		if (snapshotCounts.length === 0) {
			return;
		}

		const formatted = snapshotCounts.map(({inline, count, noun}) => {
			const words = [markup`<number emphasis>${String(count)}</number>`];
			if (inline) {
				words.push(markup`inline`);
				words.push(
					markup`<grammarNumber plural="snapshots" singular="snapshot">${String(
						count,
					)}</grammarNumber>`,
				);
			} else {
				words.push(markup`snapshot`);
				words.push(
					markup`<grammarNumber plural="files" singular="file">${String(count)}</grammarNumber>`,
				);
			}
			words.push(markup`${noun}`);
			return joinMarkup(words, markup` `);
		});
		for (const msg of formatted) {
			reporter.info(msg);
		}
	}

	private printSnapshotSuggestion(reporter: Reporter) {
		if (this.needsSnapshotUpdate) {
			reporter.info(
				markup`Outdated snapshots found. To update these if correct, run`,
			);
			reporter.command("rome test --update-snapshots");
			reporter.br();
		}
	}

	private throwPrinter() {
		const {printer} = this;

		printer.disableDefaultFooter();

		printer.onFooterPrint(async (reporter, isError) => {
			this.printCoverageReport(isError);
			this.printSnapshotSuggestion(reporter);
			this.printSnapshotCounts(reporter);
			this.printFocusedTestWarning(reporter);

			const {passedTests, failedTests} = this.progress;
			const otherErrorCount = printer.processor.calculateVisibile().total;

			if (passedTests > 0 || !isError) {
				reporter.success(
					markup`<emphasis>${humanizeNumber(passedTests)}</emphasis> ${grammarNumberTests(
						passedTests,
					)} passed${isError ? "" : "!"}`,
				);
			}

			if (failedTests > 0) {
				let message = markup`<emphasis>${humanizeNumber(failedTests)}</emphasis> ${grammarNumberTests(
					failedTests,
				)} failed`;

				// Don't output the total error count if it's equal to the failed test count as it's redundant and doesn't provide any
				// additional information
				if (otherErrorCount > failedTests) {
					message = markup`${message} with <emphasis>${humanizeNumber(
						otherErrorCount,
					)}</emphasis> errors`;
				}

				reporter.error(message);
			}

			// Output dedicated failure log if we only had errors unassociated with a test
			if (otherErrorCount > 0 && failedTests === 0) {
				reporter.error(
					markup`<emphasis>${humanizeNumber(otherErrorCount)}</emphasis> failures`,
				);
			}
		});

		throw printer;
	}
}
