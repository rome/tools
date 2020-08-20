/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {Reporter, ReporterNamespace} from "@internal/cli-reporter";
import {
	Diagnostic,
	DiagnosticsError,
	deriveDiagnosticFromError,
	descriptions,
	diagnosticLocationToMarkupFilelink,
	getDiagnosticsFromError,
} from "@internal/diagnostics";
import {TestRef} from "../../common/bridges/TestWorkerBridge";
import {Server, ServerRequest} from "@internal/core";
import {DiagnosticsPrinter} from "@internal/cli-diagnostics";
import {humanizeNumber} from "@internal/string-utils";
import {Bridge, BridgeError} from "@internal/events";
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
import {
	AnyMarkups,
	StaticMarkup,
	concatMarkup,
	markup,
	readMarkup,
} from "@internal/markup";
import {MAX_WORKER_COUNT} from "@internal/core/common/constants";
import net = require("net");
import {FocusedTest} from "@internal/core/test-worker/TestWorkerFile";
import {SourceMapConsumerCollection} from "@internal/codec-source-map";
import {VoidCallback} from "@internal/typescript-helpers";
import Bundler from "@internal/core/server/bundler/Bundler";
import {
	AbsoluteFilePath,
	AbsoluteFilePathMap,
	AbsoluteFilePathSet,
} from "@internal/path";
import TestServerWorker from "@internal/core/server/testing/TestServerWorker";
import TestServerFile from "@internal/core/server/testing/TestServerFile";
import {ExtendedMap} from "@internal/collections";

export class BridgeDiagnosticsError extends DiagnosticsError {
	constructor(diag: Diagnostic, bridge: Bridge) {
		super(readMarkup(diag.description.message), [diag]);
		this.bridge = bridge;
	}

	public bridge: Bridge;
}

function grammarNumberTests(num: number): StaticMarkup {
	return markup`<grammarNumber plural="tests" singular="test">${String(num)}</grammarNumber>`;
}

function getProgressTestRefText(ref: TestRef) {
	return markup`<filelink target="${ref.path.join()}" />: ${ref.testName}`;
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
		this.logger = this.server.logger.namespace(markup`[TestServerRunner]`);
		this.request = opts.request;
		this.options = opts.options;

		this.ignoreBridgeEndError = new Set();

		this.coverageCollector = new CoverageCollector();

		this.progress = {
			totalTests: 0,
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

		this.sourceMaps = new SourceMapConsumerCollection();
		this.printer = opts.request.createDiagnosticsPrinter(
			this.request.createDiagnosticsProcessor({
				origins: [
					{
						category: "test",
						message: "Run initiated",
					},
				],
				sourceMaps: this.sourceMaps,
			}),
		);
	}

	public sourceMaps: SourceMapConsumerCollection;
	public printer: DiagnosticsPrinter;
	public coverageCollector: CoverageCollector;
	public focusedTests: Array<FocusedTest>;
	public options: TestServerRunnerOptions;
	public files: AbsoluteFilePathMap<TestServerFile>;

	private request: ServerRequest;
	private reporter: Reporter;
	private logger: ReporterNamespace;
	private paths: AbsoluteFilePathSet;
	private server: Server;
	private ignoreBridgeEndError: Set<Bridge>;

	public testFilesStack: Array<AbsoluteFilePath>;

	private runningTests: ExtendedMap<
		string,
		{
			ref: TestRef;
			timeout: undefined | NodeJS.Timeout;
		}
	>;

	public progress: {
		totalTests: number;
		startedTests: number;
		finishedTests: number;
		updatedInlineSnapshots: number;
		updatedSnapshots: number;
		deletedSnapshots: number;
		createdSnapshots: number;
	};

	public handlePossibleBridgeError(err: Error) {
		let diagnostics = getDiagnosticsFromError(err);
		let bridge: undefined | Bridge;

		if (err instanceof BridgeDiagnosticsError) {
			bridge = err.bridge;
		}

		if (err instanceof BridgeError) {
			bridge = err.bridge;
			diagnostics = [
				deriveDiagnosticFromError(
					err,
					{
						description: {
							category: "tests/failure",
						},
					},
				),
			];
		}

		if (diagnostics === undefined || bridge === undefined) {
			throw err;
		} else {
			if (!this.ignoreBridgeEndError.has(bridge)) {
				this.printer.processor.addDiagnostics(diagnostics);
			}
		}
	}

	private async setupWorkers(): Promise<Array<TestServerWorker>> {
		// TODO some smarter logic. we may not need all these workers
		const workers: Array<Promise<TestServerWorker>> = [];
		for (let i = 0; i < MAX_WORKER_COUNT; i++) {
			const inspectorPort = await findAvailablePort();
			const worker = new TestServerWorker({
				runner: this,
				flags: {inspectorPort},
				server: this.server,
				request: this.request,
			});
			workers.push(worker.init().then(() => worker));
		}
		return Promise.all(workers);
	}

	public async init() {
		const fileQueue: Array<TestServerFile> = [];
		const workers = await this.setupWorkers();

		await this.reporter.steps([
			{
				message: markup`Bundling test files`,
				callback: async () => {
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
							this.sourceMaps.add(path.join(), consumer);
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
				},
			},
			{
				message: markup`Loading test files`,
				callback: async () => {
					const progress = this.reporter.progress({
						title: markup`Preparing`,
					});
					progress.setTotal(this.paths.size);
					await Promise.all(
						workers.map((worker) => worker.prepareAll(progress, fileQueue)),
					);
					progress.end();

					// If we have focused tests, clear the pending queues and populate it with only ours
					if (this.focusedTests.length > 0) {
						for (const file of this.files.values()) {
							file.clearPendingTests();
						}

						for (const {ref} of this.focusedTests) {
							this.files.assert(ref.path).addPendingTest(ref.testName);
						}
					}
				},
			},
			{
				message: markup`Running tests`,
				callback: async () => {
					const runProgress = this.setupRunProgress(workers);
					await Promise.all(workers.map((worker) => worker.run()));
					runProgress.teardown();
				},
			},
		]);

		if (this.focusedTests.length === 0) {
			for (const file of this.files.values()) {
				await file.finish();
			}
		}
		await this.request.flushFiles();
		this.throwPrinter();
	}

	private getTotalTests(): number {
		if (this.focusedTests.length > 0) {
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
					this.server.wrapFatalPromise(
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

	public onTestFound(ref: TestRef) {
		const file = this.files.assert(ref.path);
		file.addPendingTest(ref.testName);
		this.progress.totalTests++;
	}

	private onTestFinished(ref: TestRef) {
		const key = refToKey(ref);
		const running = this.runningTests.assert(key);

		this.logger.info(markup`Finished test ${key}`);
		if (running.timeout !== undefined) {
			clearTimeout(running.timeout);
		}
		this.runningTests.delete(key);
		this.progress.finishedTests++;
	}

	private setupRunProgress(workers: Array<TestServerWorker>): TestProgress {
		const progress = this.request.reporter.progress({
			persistent: true,
			title: markup`Running`,
		});
		progress.setTotal(this.getTotalTests());

		for (const worker of workers) {
			const {bridge} = worker;
			const ourRunningTests: Set<string> = new Set();

			bridge.endEvent.subscribe((error) => {
				// Cancel all currently running tests
				const cancelTests: Array<TestRef> = [];

				for (const key of ourRunningTests) {
					const test = this.runningTests.get(key);
					if (test !== undefined) {
						cancelTests.push(test.ref);
					}
				}

				for (const ref of cancelTests) {
					this.onTestFinished(ref);

					if (cancelTests.length === 1) {
						// If we only have one test to cancel then let's only point the bridge error to this test
						this.ignoreBridgeEndError.add(bridge);

						const errDiag = deriveDiagnosticFromError(
							error,
							{
								label: markup`${ref.testName}`,
								filename: ref.path.join(),
								description: {
									category: "tests/failure",
								},
								tags: {
									internal: true,
								},
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
								filename: ref.path.join(),
							},
						});
					}
				}
			});

			bridge.testStart.subscribe((data) => {
				const key = refToKey(data.ref);
				ourRunningTests.add(key);
				this.onTestStart(worker, data.ref, data.timeout);
				progress.pushText(getProgressTestRefText(data.ref), key);
			});

			bridge.testFinish.subscribe((data) => {
				this.onTestFinished(data.ref);
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
			const {filename} = file;

			// Get the absolute filename
			const absolute = server.projectManager.getFilePathFromUid(filename);
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

			const filenameParts = filename.split("/");
			const basename = filenameParts.pop();
			if (basename === undefined) {
				throw new Error("Should always be at least one element from a split()");
			}

			let target: CoverageDirectory = root;

			for (const part of filenameParts) {
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

		const rows: Array<AnyMarkups> = [];

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
				let absolute = file.filename;

				// Exchange any UIDs
				const absolutePath = server.projectManager.getFilePathFromUid(
					file.filename,
				);
				if (absolutePath !== undefined) {
					absolute = absolutePath.join();
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
		if (focusedTests.length === 0) {
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

		let snapshotCounts: Array<{
			inline: boolean;
			count: number;
			noun: string;
		}> = [
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
			return concatMarkup(words, markup` `);
		});
		for (const msg of formatted) {
			reporter.info(msg);
		}
	}

	private throwPrinter() {
		const {printer} = this;

		printer.onFooterPrint(async (reporter, isError) => {
			this.printCoverageReport(isError);
			this.printSnapshotCounts(reporter);
			this.printFocusedTestWarning(reporter);

			const totalCount = this.getTotalTests();
			if (totalCount > 0 || !isError) {
				reporter.success(
					markup`<emphasis>${humanizeNumber(totalCount)}</emphasis> ${grammarNumberTests(
						totalCount,
					)} passed!`,
				);
				if (!isError) {
					printer.disableDefaultFooter();
				}
			}
		});

		throw printer;
	}
}
