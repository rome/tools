/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {UnknownObject} from "@romejs/typescript-helpers";
import {
	Diagnostic,
	DiagnosticAdvice,
	DiagnosticLocation,
	DiagnosticLogCategory,
	DiagnosticOrigin,
	INTERNAL_ERROR_LOG_ADVICE,
	catchDiagnostics,
	createBlessedDiagnosticMessage,
	createSingleDiagnosticError,
	deriveDiagnosticFromErrorStructure,
	descriptions,
	getErrorStackAdvice,
} from "@romejs/diagnostics";
import {
	GlobalTestOptions,
	TestCallback,
	TestOptions,
} from "@romejs-runtime/rome/test";
import {
	TestRef,
	default as TestWorkerBridge,
	TestWorkerPrepareTestOptions,
	TestWorkerPrepareTestResult,
	TestWorkerRunTestOptions,
} from "../common/bridges/TestWorkerBridge";
import {TestMasterRunnerOptions} from "../master/testing/types";
import SnapshotManager, {
	InlineSnapshotUpdate,
	SnapshotCounts,
} from "./SnapshotManager";
import TestAPI, {OnTimeout} from "./TestAPI";
import executeMain from "../common/utils/executeMain";
import {
	FileReference,
	convertTransportFileReference,
} from "../common/types/files";
import {AbsoluteFilePath, createAbsoluteFilePath} from "@romejs/path";
import {escapeMarkup, markup} from "@romejs/string-markup";
import {
	ErrorFrames,
	StructuredError,
	getErrorStructure,
	getSourceLocationFromErrorFrame,
} from "@romejs/v8";
import prettyFormat from "@romejs/pretty-format";

const MAX_RUNNING_TESTS = 20;

function cleanFrames(frames: ErrorFrames): ErrorFrames {
	// TODO we should actually get the frames before module init and do it that way
	// Remove everything before the original module factory
	let latestTestWorkerFrame = frames.find((frame, i) => {
		if (
			frame.typeName === "global" &&
			frame.methodName === undefined &&
			frame.functionName === undefined
		) {
			// We are the global.<anonymous> frame
			// Now check for Script.runInContext
			const nextFrame = frames[i + 1];
			if (
				nextFrame !== undefined &&
				nextFrame.typeName === "Script" &&
				nextFrame.methodName === "runInContext"
			) {
				// Yes!
				// TODO also check for ___$romejs$core$common$utils$executeMain_ts$default (packages/romejs/core/common/utils/executeMain.ts:69:17)
				return true;
			}
		}

		return false;
	});

	// And if there was no module factory frame, then we must be inside of a test
	if (latestTestWorkerFrame === undefined) {
		latestTestWorkerFrame = frames.find((frame) => {
			return (
				frame.typeName !== undefined &&
				frame.typeName.includes("$TestWorkerRunner")
			);
		});
	}

	if (latestTestWorkerFrame === undefined) {
		return frames;
	}

	return frames.slice(0, frames.indexOf(latestTestWorkerFrame));
}

export type TestWorkerFileResult = {
	snapshotCounts: SnapshotCounts;
	inlineSnapshotUpdates: Array<InlineSnapshotUpdate>;
};

type FoundTest = {
	options: TestOptions;
	callback: TestCallback;
};

export type FocusedTest = {
	testName: string;
	location: DiagnosticLocation;
};

export default class TestWorkerRunner {
	constructor(opts: TestWorkerPrepareTestOptions, bridge: TestWorkerBridge) {
		this.opts = opts;
		this.locked = false;
		this.file = convertTransportFileReference(opts.file);
		this.options = opts.options;
		this.bridge = bridge;
		this.projectFolder = createAbsoluteFilePath(opts.projectFolder);

		this.snapshotManager = new SnapshotManager(
			this,
			createAbsoluteFilePath(opts.file.real),
		);

		this.hasDiagnostics = false;
		this.consoleAdvice = [];
		this.hasFocusedTests = false;
		this.focusedTests = [];
		this.pendingDiagnostics = [];
		this.foundTests = new Map();
	}

	foundTests: Map<string, FoundTest>;
	hasFocusedTests: boolean;
	focusedTests: Array<FocusedTest>;
	bridge: TestWorkerBridge;
	projectFolder: AbsoluteFilePath;
	file: FileReference;
	options: TestMasterRunnerOptions;
	snapshotManager: SnapshotManager;
	opts: TestWorkerPrepareTestOptions;
	locked: boolean;
	consoleAdvice: DiagnosticAdvice;
	hasDiagnostics: boolean;
	pendingDiagnostics: Array<Diagnostic>;

	createConsole(): Partial<Console> {
		const addDiagnostic = (
			category: DiagnosticLogCategory,
			args: Array<unknown>,
		) => {
			let textParts: Array<string> = [];
			if (args.length === 1 && typeof args[0] === "string") {
				textParts.push(escapeMarkup(args[0]));
			} else {
				textParts = args.map((arg) =>
					prettyFormat(arg, {allowCustom: false, markup: true})
				);
			}
			const text = textParts.join(" ");

			const err = new Error();

			// Remove the first two frames to get to the actual source
			const frames = cleanFrames(getErrorStructure(err).frames.slice(2));

			this.consoleAdvice.push({
				type: "log",
				category,
				text,
			});
			this.consoleAdvice = this.consoleAdvice.concat(
				getErrorStackAdvice(
					getErrorStructure({
						...err,
						frames,
					}),
				),
			);
		};

		function log(...args: Array<unknown>): void {
			addDiagnostic("none", args);
		}

		return {
			assert(expression: unknown, ...args: Array<unknown>): void {
				if (!expression) {
					args[0] = `Assertion failed${args.length === 0 ? "" : `: ${args[0]}`}`;
					addDiagnostic("warn", args);
				}
			},
			dir(obj: unknown): void {
				addDiagnostic("info", [obj]);
			},
			error: (...args: Array<unknown>): void => {
				addDiagnostic("error", args);
			},
			warn: (...args: Array<unknown>): void => {
				addDiagnostic("warn", args);
			},
			dirxml: log,
			debug: log,
			info: (...args: Array<unknown>): void => {
				addDiagnostic("info", args);
			},
			log,
			trace: log,
			// Noop
			count(): void {},
			countReset(): void {},
			table(): void {},
			time(): void {},
			timeEnd(): void {},
			timeLog(): void {},
			clear(): void {},
			group(): void {},
			groupCollapsed(): void {},
			groupEnd(): void {},
			profile(): void {},
			profileEnd(): void {},
			timeStamp(): void {},
		};
	}

	//  Global variables to expose to tests
	getEnvironment(): UnknownObject {
		const testOptions: GlobalTestOptions = {
			dirname: this.file.real.getParent().join(),
			register: (
				callsiteError: Error,
				opts: TestOptions,
				callback: TestCallback,
			) => {
				this.registerTest(callsiteError, opts, callback);
			},
		};

		return {
			__ROME__TEST_OPTIONS__: testOptions,
			console: this.createConsole(),
		};
	}

	// execute the test file and discover tests
	async discoverTests() {
		const {code} = this.opts;

		try {
			const res = await executeMain({
				path: this.file.real,
				code,
				globals: this.getEnvironment(),
			});

			if (res.syntaxError !== undefined) {
				const message = `A bundle was generated that contained a syntax error: ${res.syntaxError.description.message.value}`;

				throw createSingleDiagnosticError({
					...res.syntaxError,
					description: {
						...res.syntaxError.description,
						message: createBlessedDiagnosticMessage(message),
						advice: [INTERNAL_ERROR_LOG_ADVICE],
					},
					location: {
						...res.syntaxError.location,
						filename: this.file.uid,
					},
				});
			}
		} catch (err) {
			await this.onError(
				undefined,
				{
					error: err,
					firstAdvice: [],
					lastAdvice: [
						{
							type: "log",
							category: "info",
							text: markup`Error occured while executing test file <filelink emphasis target="${this.file.uid}" />`,
						},
					],
				},
			);
		}
	}

	lockTests() {
		this.locked = true;
	}

	registerTest(
		callsiteError: Error,
		options: TestOptions,
		callback: TestCallback,
	) {
		if (this.locked) {
			throw new Error("Test can't be added outside of init");
		}

		let testName = options.name;
		if (Array.isArray(testName)) {
			testName = testName.join(" > ");
		}

		if (this.foundTests.has(testName)) {
			throw new Error(`Test ${testName} has already been defined`);
		}

		this.foundTests.set(
			testName,
			{
				callback,
				options,
			},
		);

		if (options.only === true) {
			const callsiteStruct = getErrorStructure(callsiteError, 1);

			this.focusedTests.push({
				testName,
				location: getSourceLocationFromErrorFrame(callsiteStruct.frames[0]),
			});

			this.hasFocusedTests = true;

			if (!this.options.focusAllowed) {
				const diag = this.deriveDiagnosticFromErrorStructure(callsiteStruct);

				this.pendingDiagnostics.push({
					...diag,
					description: {
						...diag.description,
						message: createBlessedDiagnosticMessage(
							"Focused tests are not allowed due to a set flag",
						),
					},
				});
			}
		}
	}

	async emitDiagnostic(
		diag: Diagnostic,
		ref?: TestRef,
		advice?: DiagnosticAdvice,
	) {
		let origin: DiagnosticOrigin = {
			category: "test/error",
			message: "Generated from a test worker without being attached to a test",
		};

		if (ref !== undefined) {
			origin.message = markup`Generated from the file <filelink target="${this.file.uid}" /> and test name "${ref.testName}"`;
		}

		let label = diag.label;
		if (label !== undefined && ref !== undefined) {
			label = escapeMarkup(ref.testName);
		}

		diag = {
			...diag,
			label,
			description: {
				...diag.description,
				advice: [...diag.description.advice, ...(advice || [])],
			},
		};

		this.hasDiagnostics = true;
		await this.bridge.testDiagnostic.call({diagnostic: diag, origin});
	}

	deriveDiagnosticFromErrorStructure(struct: StructuredError): Diagnostic {
		return deriveDiagnosticFromErrorStructure(
			struct,
			{
				description: {
					category: "tests/failure",
				},
				filename: this.file.real.join(),
				cleanFrames,
			},
		);
	}

	async onError(
		testName: undefined | string,
		opts: {
			error: Error;
			firstAdvice?: DiagnosticAdvice;
			lastAdvice?: DiagnosticAdvice;
		},
	): Promise<void> {
		let diagnostic = this.deriveDiagnosticFromErrorStructure(
			getErrorStructure(opts.error),
		);

		diagnostic = {
			...diagnostic,
			unique: true,
			description: {
				...diagnostic.description,
				advice: [
					...(opts.firstAdvice || []),
					...diagnostic.description.advice,
					...(opts.lastAdvice || []),
				],
			},
		};

		await this.emitDiagnostic(
			diagnostic,
			testName === undefined ? undefined : this.createTestRef(testName),
		);
	}

	async teardownTest(testName: string, api: TestAPI): Promise<boolean> {
		api.clearTimeout();

		try {
			await api.teardownEvent.callOptional();
			return true;
		} catch (err) {
			await this.onError(
				testName,
				{
					error: err,
					firstAdvice: [],
					lastAdvice: [
						{
							type: "log",
							category: "info",
							text: `Error occured while running <emphasis>teardown</emphasis> for test <emphasis>${testName}</emphasis>`,
						},
						...api.advice,
					],
				},
			);
			return false;
		}
	}

	createTestRef(testName: string): TestRef {
		return {
			testName,
			filename: this.file.real.join(),
		};
	}

	async runTest(testName: string, callback: TestCallback) {
		let onTimeout: OnTimeout = () => {
			throw new Error("Promise wasn't created. Should be impossible.");
		};

		const timeoutPromise = new Promise((resolve, reject) => {
			onTimeout = (time: number) => {
				reject(new Error(`Test timeout - exceeded ${String(time)}ms`));
			};
		});

		const ref = this.createTestRef(testName);

		const emitDiagnostic = (diag: Diagnostic): Promise<void> => {
			return this.emitDiagnostic(diag, ref, api.advice);
		};

		const api = new TestAPI({
			file: this.file,
			testName,
			onTimeout,
			snapshotManager: this.snapshotManager,
			options: this.options,
			emitDiagnostic,
		});

		let testSuccess = false;

		try {
			const {diagnostics} = await catchDiagnostics(async () => {
				const res = callback(api);

				// Ducktyping this to detect a cross-realm Promise
				if (res !== undefined && typeof res.then === "function") {
					await Promise.race([timeoutPromise, res]);
				}
			});

			if (diagnostics !== undefined) {
				for (const diag of diagnostics) {
					await emitDiagnostic(diag);
				}
			}

			testSuccess = true;
		} catch (err) {
			await this.onError(
				testName,
				{
					error: err,
					firstAdvice: [],
					lastAdvice: api.advice,
				},
			);
		} finally {
			const teardownSuccess = await this.teardownTest(testName, api);
			await this.bridge.testFinish.call({
				success: testSuccess && teardownSuccess,
				ref,
			});
		}
	}

	async run(opts: TestWorkerRunTestOptions): Promise<TestWorkerFileResult> {
		const promises: Set<Promise<void>> = new Set();

		const {foundTests} = this;

		// Emit error about no found tests. If we already have diagnostics then there was an issue
		// during initialization.
		if (foundTests.size === 0 && !this.hasDiagnostics) {
			this.emitDiagnostic({
				location: {
					filename: this.file.uid,
				},
				description: descriptions.TESTS.UNDECLARED,
			});
		}

		// We could be pretending we have focused tests here but at least one file was execueted with
		// focused tests
		if (opts.onlyFocusedTests) {
			this.hasFocusedTests = true;
		}

		// Execute all the tests
		for (const [testName, test] of foundTests) {
			const {options, callback} = test;
			if (this.hasFocusedTests && !test.options.only) {
				continue;
			}

			this.bridge.testStart.send({
				ref: {
					filename: this.file.real.join(),
					testName,
				},
				timeout: options.timeout,
			});

			const promise = this.runTest(testName, callback);

			if (this.options.syncTests) {
				await promise;
			} else {
				promise.then(() => {
					promises.delete(promise);
				});
				promises.add(promise);

				// if there's 5 promises, then wait for one of them to finish
				if (promises.size > MAX_RUNNING_TESTS) {
					await Promise.race(Array.from(promises));
				}
			}
		}

		// Execute the remaining tests
		await Promise.all(Array.from(promises));

		// Save the snapshot
		await this.snapshotManager.save();

		if (this.hasDiagnostics && this.consoleAdvice.length > 0) {
			await this.emitDiagnostic({
				description: descriptions.TESTS.LOGS(this.consoleAdvice),
				location: {
					filename: this.file.uid,
				},
			});
		}

		for (const diag of this.pendingDiagnostics) {
			await this.emitDiagnostic(diag);
		}

		return {
			inlineSnapshotUpdates: this.snapshotManager.inlineSnapshotsUpdates,
			snapshotCounts: this.snapshotManager.snapshotCounts,
		};
	}

	async emitFoundTests() {
		const tests: Array<TestRef> = [];

		for (const testName of this.foundTests.keys()) {
			tests.push({
				filename: this.file.real.join(),
				testName,
			});
		}

		await this.bridge.testsFound.call(tests);
	}

	async wrap(callback: () => Promise<void>): Promise<void> {
		try {
			const {diagnostics} = await catchDiagnostics(callback);

			if (diagnostics !== undefined) {
				for (const diagnostic of diagnostics) {
					await this.emitDiagnostic(diagnostic);
				}
			}
		} catch (err) {
			await this.onError(
				undefined,
				{
					error: err,
					firstAdvice: [],
					lastAdvice: [
						{
							type: "log",
							category: "info",
							text: markup`Error occured while executing test file <filelink emphasis target="${this.file.uid}" />`,
						},
						INTERNAL_ERROR_LOG_ADVICE,
					],
				},
			);
		}
	}

	async prepare(): Promise<TestWorkerPrepareTestResult> {
		await this.wrap(async () => {
			await this.snapshotManager.init();
			await this.discoverTests();
			await this.emitFoundTests();
			this.lockTests();
		});
		return {focusedTests: this.focusedTests};
	}
}
