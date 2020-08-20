/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {UnknownObject} from "@internal/typescript-helpers";
import {
	Diagnostic,
	DiagnosticAdvice,
	DiagnosticLocation,
	DiagnosticLogCategory,
	catchDiagnostics,
	createSingleDiagnosticError,
	deriveDiagnosticFromErrorStructure,
	descriptions,
	diagnosticLocationToMarkupFilelink,
	getErrorStackAdvice,
} from "@internal/diagnostics";
import {
	GlobalTestOptions,
	TestCallback,
	TestOptions,
} from "@internal/virtual-rome/test";
import {
	TestRef,
	default as TestWorkerBridge,
	TestWorkerPrepareTestOptions,
	TestWorkerPrepareTestResult,
	TestWorkerRunTestOptions,
} from "../common/bridges/TestWorkerBridge";
import {TestServerRunnerOptions} from "../server/testing/types";
import SnapshotManager, {
	InlineSnapshotUpdate,
	Snapshot,
} from "./SnapshotManager";
import TestAPI, {OnTimeout} from "./TestAPI";
import executeMain from "../common/utils/executeMain";
import {
	AbsoluteFilePath,
	AbsoluteFilePathMap,
	createAbsoluteFilePath,
} from "@internal/path";
import {
	concatMarkup,
	isEmptyMarkup,
	markup,
	readMarkup,
	serializeLazyMarkup,
} from "@internal/markup";
import {
	ErrorFrames,
	StructuredError,
	getDiagnosticLocationFromErrorFrame,
	getErrorStructure,
} from "@internal/v8";
import prettyFormat from "@internal/pretty-format";
import {TestWorker} from "@internal/core";
import {ExtendedMap} from "@internal/collections";

export function cleanFrames(frames: ErrorFrames): ErrorFrames {
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
				// TODO also check for ___$romefrontend$core$common$utils$executeMain_ts$default (internal/romefrontend/core/common/utils/executeMain.ts:69:17)
				return true;
			}
		}

		return false;
	});

	// And if there was no module factory frame, then we must be inside of a test
	if (latestTestWorkerFrame === undefined) {
		latestTestWorkerFrame = frames.find((frame) => {
			return (
				frame.filename !== undefined &&
				frame.filename.includes("TestWorkerFile")
			);
		});
	}

	if (latestTestWorkerFrame === undefined) {
		return frames;
	}

	return frames.slice(0, frames.indexOf(latestTestWorkerFrame));
}

export type TestWorkerFileResult = {
	snapshots: AbsoluteFilePathMap<Snapshot>;
	inlineSnapshotUpdates: Array<InlineSnapshotUpdate>;
};

type FoundTest = {
	name: string;
	options: TestOptions;
	callback: TestCallback;
	callsiteLocation: DiagnosticLocation;
};

export type FocusedTest = {
	ref: TestRef;
	location: DiagnosticLocation;
};

export default class TestWorkerFile {
	constructor(
		worker: TestWorker,
		bridge: TestWorkerBridge,
		opts: TestWorkerPrepareTestOptions,
	) {
		this.opts = opts;
		this.locked = false;
		this.path = opts.path;
		this.options = opts;
		this.globalOptions = opts.globalOptions;
		this.bridge = bridge;
		this.worker = worker;
		this.projectDirectory = createAbsoluteFilePath(opts.projectDirectory);

		this.snapshotManager = new SnapshotManager(
			this,
			createAbsoluteFilePath(opts.path),
		);

		this.onlyFocusedTests = false;
		this.hasDiagnostics = false;
		this.consoleAdvice = [];
		this.focusedTests = [];
		this.pendingDiagnostics = [];
		this.foundTests = new ExtendedMap("foundTests");
	}

	public hasDiagnostics: boolean;
	public onlyFocusedTests: boolean;
	public path: AbsoluteFilePath;
	public projectDirectory: AbsoluteFilePath;
	public globalOptions: TestServerRunnerOptions;
	public options: TestWorkerPrepareTestOptions;

	private worker: TestWorker;
	private foundTests: ExtendedMap<string, FoundTest>;
	private focusedTests: Array<FocusedTest>;
	private bridge: TestWorkerBridge;
	private snapshotManager: SnapshotManager;
	private opts: TestWorkerPrepareTestOptions;
	private locked: boolean;
	private consoleAdvice: Array<() => DiagnosticAdvice>;

	// Diagnostics that shouldn't result in console logs being output
	private pendingDiagnostics: Array<Diagnostic>;

	private createTestRef(test: FoundTest): TestRef {
		return {
			testName: test.name,
			path: this.path,
		};
	}

	private createConsole(): Partial<Console> {
		const addDiagnostic = (
			category: DiagnosticLogCategory,
			args: Array<unknown>,
		) => {
			const err = new Error();

			// Remove the first two frames to get to the actual source
			const struct = getErrorStructure(err);
			const frames = cleanFrames(struct.frames.slice(2));

			this.consoleAdvice.push(() => {
				let text;
				if (args.length === 1 && typeof args[0] === "string") {
					text = markup`${args[0]}`;
				} else {
					text = concatMarkup(
						args.map((arg) =>
							serializeLazyMarkup(prettyFormat(arg, {allowCustom: false}))
						),
						markup` `,
					);
				}

				if (isEmptyMarkup(text)) {
					text = markup`<dim>empty log</dim>`;
				}

				return [
					{
						type: "log",
						category,
						text,
					},
					...getErrorStackAdvice({...struct, frames}),
				];
			});
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
	private getEnvironment(): UnknownObject {
		const testOptions: GlobalTestOptions = {
			dirname: this.path.getParent().join(),
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
	private async discoverTests() {
		const code = this.worker.serializeAssembled(this.opts.assembled);

		try {
			const res = await executeMain({
				path: this.path,
				code,
				globals: this.getEnvironment(),
			});

			if (res.syntaxError !== undefined) {
				const message = markup`A bundle was generated that contained a syntax error: ${readMarkup(
					res.syntaxError.description.message,
				)}`;

				throw createSingleDiagnosticError({
					...res.syntaxError,
					description: {
						...res.syntaxError.description,
						message,
					},
					location: {
						...res.syntaxError.location,
						filename: this.path.join(),
					},
					tags: {
						...res.syntaxError,
						internal: true,
					},
				});
			}
		} catch (err) {
			await this.emitError({
				origin: {type: "EXECUTING"},
				error: err,
			});
		}

		// Emit error about no found tests. If we already have diagnostics then there was an issue
		// during initialization.
		if (this.foundTests.size === 0 && !this.hasDiagnostics) {
			await this.emitDiagnostic({
				location: {
					filename: this.path.join(),
				},
				description: descriptions.TESTS.UNDECLARED,
			});
		}
	}

	private lockTests() {
		this.locked = true;
	}

	private isFocusedTest({name, only}: TestOptions): boolean {
		const {filter} = this.globalOptions;
		if (filter === undefined) {
			return only === true;
		} else {
			return name.includes(filter);
		}
	}

	private registerTest(
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

		// Get the frame where this test was declared. We pop 1 off as the error is created inside the test function.
		const callsiteStruct = getErrorStructure(callsiteError, 1);
		const callsiteLocation = getDiagnosticLocationFromErrorFrame(
			callsiteStruct.frames[0],
		);

		const foundTest: FoundTest = {
			callsiteLocation,
			name: testName,
			callback,
			options,
		};

		this.foundTests.set(testName, foundTest);

		if (this.isFocusedTest(options)) {
			this.focusedTests.push({
				ref: this.createTestRef(foundTest),
				location: callsiteLocation,
			});

			if (!this.globalOptions.focusAllowed) {
				const diag = this.deriveDiagnosticFromErrorStructure(callsiteStruct);

				this.pendingDiagnostics.push({
					...diag,
					description: {
						...diag.description,
						message: markup`Focused tests are not allowed due to a set flag`,
					},
				});
			}
		}
	}

	public async emitDiagnostic(diag: Diagnostic, test?: FoundTest) {
		let label = diag.label;
		if (label === undefined && test !== undefined) {
			label = markup`${test.name}`;
		}

		diag = {
			...diag,
			label,
			tags: {
				...diag.tags,
				unique: true,
			},
		};

		this.hasDiagnostics = true;
		await this.bridge.testDiagnostic.call({
			diagnostic: diag,
			origin: undefined,
			testPath: this.path,
		});
	}

	private deriveDiagnosticFromErrorStructure(
		struct: StructuredError,
	): Diagnostic {
		return deriveDiagnosticFromErrorStructure(
			struct,
			{
				description: {
					category: "tests/failure",
				},
				filename: this.path.join(),
				cleanFrames,
				stackAdviceOptions: {
					importantFilenames: [this.path.join()],
				},
			},
		);
	}

	private async emitError(
		opts: {
			error: Error;
			origin:
				| {
						type: "INTERNAL";
					}
				| {
						type: "EXECUTING";
					}
				| {
						type: "TEST";
						test: FoundTest;
					}
				| {
						type: "TEARDOWN";
						test: FoundTest;
					};
			trailingAdvice?: DiagnosticAdvice;
		},
	): Promise<void> {
		const {origin, error, trailingAdvice = []} = opts;

		let diagnostic = this.deriveDiagnosticFromErrorStructure(
			getErrorStructure(error),
		);
		let {location} = diagnostic;
		let {advice} = diagnostic.description;
		let test: undefined | FoundTest;

		switch (origin.type) {
			case "EXECUTING": {
				advice.push({
					type: "log",
					category: "info",
					text: markup`Error occured while executing test file <filelink emphasis target="${this.path.join()}" />`,
				});
				break;
			}

			case "TEST": {
				advice.push({
					type: "log",
					category: "info",
					text: markup`Test declared at <emphasis>${diagnosticLocationToMarkupFilelink(
						origin.test.callsiteLocation,
					)}:</emphasis>`,
				});
				advice.push({
					type: "frame",
					location: origin.test.callsiteLocation,
				});
				test = origin.test;
				break;
			}

			case "TEARDOWN": {
				advice.push({
					type: "log",
					category: "info",
					text: markup`Error occured while running <emphasis>teardown</emphasis> for test declared at <emphasis>${diagnosticLocationToMarkupFilelink(
						origin.test.callsiteLocation,
					)}:</emphasis>`,
				});
				advice.push({
					type: "frame",
					location: origin.test.callsiteLocation,
				});
				test = origin.test;
				break;
			}
		}

		advice = [...advice, ...trailingAdvice];

		diagnostic = {
			...diagnostic,
			location,
			tags: {
				...diagnostic.tags,
				unique: true,
			},
			description: {
				...diagnostic.description,
				advice,
			},
		};

		if (origin.type === "INTERNAL") {
			diagnostic = {
				...diagnostic,
				tags: {
					...diagnostic.tags,
					internal: true,
				},
			};
		}

		await this.emitDiagnostic(diagnostic, test);
	}

	private async teardownTest(test: FoundTest, api: TestAPI): Promise<boolean> {
		api.clearTimeout();

		try {
			await api.teardownEvent.callOptional();
			return true;
		} catch (err) {
			await this.emitError({
				origin: {type: "TEARDOWN", test},
				error: err,
				trailingAdvice: api.getAdvice(),
			});
			return false;
		}
	}

	private async runTest(test: FoundTest) {
		const {callback, name: testName} = test;

		let onTimeout: OnTimeout = () => {
			throw new Error("Promise wasn't created. Should be impossible.");
		};

		const timeoutPromise = new Promise((resolve, reject) => {
			onTimeout = (time: number) => {
				reject(new Error(`Test timeout - exceeded ${String(time)}ms`));
			};
		});

		const emitDiagnostic = (diag: Diagnostic): Promise<void> => {
			return this.emitDiagnostic(
				{
					...diag,
					description: {
						...diag.description,
						advice: api.getAdvice(diag.description.advice),
					},
				},
				test,
			);
		};

		const api = new TestAPI({
			path: this.path,
			testName,
			onTimeout,
			snapshotManager: this.snapshotManager,
			options: this.globalOptions,
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
			await this.emitError({
				origin: {type: "TEST", test},
				error: err,
				trailingAdvice: api.getAdvice(),
			});
		} finally {
			const teardownSuccess = await this.teardownTest(test, api);
			await this.bridge.testFinish.call({
				success: testSuccess && teardownSuccess,
				ref: this.createTestRef(test),
			});
		}
	}

	public async run(runOptions: TestWorkerRunTestOptions): Promise<void> {
		for (const testName of runOptions.testNames) {
			const test = this.foundTests.assert(testName);
			const {options} = test;

			this.bridge.testStart.send({
				ref: this.createTestRef(test),
				timeout: options.timeout,
			});

			await this.runTest(test);
		}
	}

	public async teardown(): Promise<TestWorkerFileResult> {
		if (this.hasDiagnostics && this.consoleAdvice.length > 0) {
			let advice: DiagnosticAdvice = [];
			for (const factory of this.consoleAdvice) {
				advice = advice.concat(factory());
			}
			await this.emitDiagnostic({
				description: descriptions.TESTS.LOGS(advice),
				location: {
					filename: this.path.join(),
				},
			});
		}

		for (const diag of this.pendingDiagnostics) {
			await this.emitDiagnostic(diag);
		}

		return {
			snapshots: this.snapshotManager.getModifiedSnapshots(),
			inlineSnapshotUpdates: this.snapshotManager.inlineSnapshotsUpdates,
		};
	}

	private async wrap(callback: () => Promise<void>): Promise<void> {
		try {
			const {diagnostics} = await catchDiagnostics(callback);

			if (diagnostics !== undefined) {
				for (const diagnostic of diagnostics) {
					await this.emitDiagnostic(diagnostic);
				}
			}
		} catch (err) {
			await this.emitError({
				origin: {type: "INTERNAL"},
				error: err,
			});
		}
	}

	public async prepare(): Promise<TestWorkerPrepareTestResult> {
		await this.wrap(async () => {
			await this.snapshotManager.init();
			await this.discoverTests();
			this.lockTests();
		});

		return {
			foundTests: Array.from(this.foundTests.keys()),
			focusedTests: this.focusedTests,
		};
	}
}
