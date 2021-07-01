/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {UnknownObject} from "@internal/typescript-helpers";
import {
	DIAGNOSTIC_CATEGORIES,
	DeriveErrorDiagnosticOptions,
	Diagnostic,
	DiagnosticAdvice,
	DiagnosticLocation,
	DiagnosticLogCategory,
	catchDiagnostics,
	createSingleDiagnosticsError,
	deriveDiagnosticFromErrorStructure,
	descriptions,
	getErrorStackAdvice,
} from "@internal/diagnostics";
import {
	GlobalTestOptions,
	TestCallback,
	TestOptions,
} from "@internal/virtual-rome/test";
import {TestServerRunnerOptions} from "../../server/testing/types";
import SnapshotManager, {
	InlineSnapshotUpdate,
	Snapshot,
	SnapshotEntry,
} from "./SnapshotManager";
import TestAPI, {OnTimeout} from "./TestAPI";
import executeMain from "../utils/executeMain";
import {
	AbsoluteFilePath,
	AbsoluteFilePathMap,
	MixedPathSet,
	createAbsoluteFilePath,
} from "@internal/path";
import {
	isEmptyMarkup,
	joinMarkup,
	markup,
	readMarkup,
	serializeLazyMarkup,
} from "@internal/markup";
import {
	ErrorFrame,
	StructuredError,
	getDiagnosticLocationFromErrorFrame,
	getErrorStructure,
} from "@internal/errors";
import prettyFormat from "@internal/pretty-format";
import {
	TestRef,
	TestWorkerPrepareTestOptions,
	TestWorkerPrepareTestResult,
	TestWorkerRunTestOptions,
	Worker,
	WorkerBridge,
} from "@internal/core";
import {ExtendedMap} from "@internal/collections";
import {BridgeClient} from "@internal/events";
import TestWorker from "./TestWorker";

export function cleanFrames(frames: ErrorFrame[]): ErrorFrame[] {
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
				nextFrame?.typeName === "Script" &&
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
				frame.path !== undefined && frame.path.join().includes("TestWorkerFile")
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
};

export type TestDetails = {
	name: string;
	options: TestOptions;
	callback: TestCallback;
	testPath: AbsoluteFilePath;
	callsiteLocation: DiagnosticLocation;
};

export type FocusedTest = {
	ref: TestRef;
	location: DiagnosticLocation;
};

// This is just using milliseconds which probably isn't precise enough, but could lead to items appearing out of order when merged
export type TestConsoleAdvice = [DiagnosticAdvice[], number][];

export default class TestWorkerFile {
	constructor(
		worker: Worker,
		tests: TestWorker,
		opts: TestWorkerPrepareTestOptions,
	) {
		this.opts = opts;
		this.worker = worker;
		this.locked = false;
		this.contextDirectory = opts.contextDirectory;
		this.path = opts.path;
		this.options = opts;
		this.globalOptions = opts.globalOptions;
		this.bridge = worker.bridge;
		this.tests = tests;
		this.projectDirectory = createAbsoluteFilePath(opts.projectDirectory);

		this.snapshotManager = new SnapshotManager(this, opts.path);

		this.onlyFocusedTests = false;
		this.hasDiagnostics = false;
		this.consoleAdvice = [];
		this.focusedTests = [];
		this.foundTests = new ExtendedMap("foundTests");
		this.failedTests = new Set();
	}

	public hasDiagnostics: boolean;
	public onlyFocusedTests: boolean;
	public path: AbsoluteFilePath;
	public contextDirectory: AbsoluteFilePath;
	public projectDirectory: AbsoluteFilePath;
	public globalOptions: TestServerRunnerOptions;
	public options: TestWorkerPrepareTestOptions;
	public snapshotManager: SnapshotManager;
	public runningSyncTest: undefined | TestAPI;

	private worker: Worker;
	private tests: TestWorker;
	private failedTests: Set<string>;
	private foundTests: ExtendedMap<string, TestDetails>;
	private focusedTests: FocusedTest[];
	private bridge: BridgeClient<typeof WorkerBridge>;
	private opts: TestWorkerPrepareTestOptions;
	private locked: boolean;
	private consoleAdvice: [(() => DiagnosticAdvice[]), number][];

	private createTestRef(test: TestDetails): TestRef {
		return {
			testName: test.name,
			path: this.path,
		};
	}

	private createConsole(): Partial<Console> {
		if (!this.globalOptions.suppressLogs) {
			return console;
		}

		const addDiagnostic = (category: DiagnosticLogCategory, args: unknown[]) => {
			const err = new Error();

			// Remove the first two frames to get to the actual source
			const struct = getErrorStructure(err);
			const frames = cleanFrames(struct.frames.slice(2));

			function adviceFactory(): DiagnosticAdvice[] {
				let text;
				if (args.length === 1 && typeof args[0] === "string") {
					text = markup`${args[0]}`;
				} else {
					text = joinMarkup(
						args.map((arg) =>
							serializeLazyMarkup(prettyFormat(arg, {accurate: true}))
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
			}

			const {runningSyncTest} = this;
			if (runningSyncTest === undefined) {
				this.consoleAdvice.push([adviceFactory, Date.now()]);
			} else {
				runningSyncTest.addToLogAdvice(adviceFactory);
			}
		};

		function log(...args: unknown[]): void {
			addDiagnostic("none", args);
		}

		return {
			assert(expression: unknown, ...args: unknown[]): void {
				if (!expression) {
					args[0] = `Assertion failed${args.length === 0 ? "" : `: ${args[0]}`}`;
					addDiagnostic("warn", args);
				}
			},
			dir(obj: unknown): void {
				addDiagnostic("info", [obj]);
			},
			error: (...args: unknown[]): void => {
				addDiagnostic("error", args);
			},
			warn: (...args: unknown[]): void => {
				addDiagnostic("warn", args);
			},
			dirxml: log,
			debug: log,
			info: (...args: unknown[]): void => {
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
			process: {
				// Suppress process events
				on() {},
			},
		};
	}

	// execute the test file and discover tests
	private async discoverTests() {
		const code = this.tests.serializeAssembled(this.opts.assembled);

		try {
			const res = await executeMain(
				this.worker,
				{
					contextDirectory: this.contextDirectory,
					commandName: "test",
					path: this.path,
					args: [],
					cwd: this.path.getParent(),
					code,
					globals: this.getEnvironment(),
				},
			);

			if (res.syntaxError !== undefined) {
				const message = markup`A bundle was generated that contained a syntax error: ${readMarkup(
					res.syntaxError.description.message,
				)}`;

				throw createSingleDiagnosticsError({
					...res.syntaxError,
					description: {
						...res.syntaxError.description,
						message,
					},
					location: {
						...res.syntaxError.location,
						path: this.path,
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
					path: this.path,
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

		const foundTest: TestDetails = {
			testPath: this.path,
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

				this.emitDiagnostic({
					...diag,
					description: {
						...diag.description,
						message: markup`Focused tests are not allowed due to a set flag`,
					},
				});
			}
		}
	}

	public async emitDiagnostic(diag: Diagnostic, test?: TestDetails) {
		this.hasDiagnostics = true;

		if (test !== undefined) {
			this.failedTests.add(test.name);
		}

		await this.bridge.events.testDiagnostic.call({
			diagnostic: diag,
			ref: test === undefined ? {path: this.path} : this.createTestRef(test),
		});
	}

	public deriveDiagnosticFromErrorStructure(
		struct: StructuredError,
		opts?: DeriveErrorDiagnosticOptions,
	): Diagnostic {
		return deriveDiagnosticFromErrorStructure(
			struct,
			{
				description: {
					category: DIAGNOSTIC_CATEGORIES["tests/failure"],
				},
				removeNodeFrames: true,
				internal: false,
				path: this.path,
				cleanFrames,
				stackAdviceOptions: {
					importantPaths: new MixedPathSet([this.path]),
				},
				...opts,
			},
		);
	}

	public emitSnapshotDiscovery(snapshotPath: AbsoluteFilePath) {
		this.bridge.events.testDiskSnapshotDiscovered.send({
			testPath: this.path,
			snapshotPath,
		});
	}

	public emitSnapshotEntry(
		snapshotPath: AbsoluteFilePath,
		entry: SnapshotEntry,
	): void {
		this.bridge.events.testSnapshotEntry.send({
			testPath: this.path,
			snapshotPath,
			entry,
		});
	}

	public emitInlineSnapshotUpdate(update: InlineSnapshotUpdate) {
		this.bridge.events.testInlineSnapshotUpdate.send({
			testPath: this.path,
			update,
		});
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
						test: TestDetails;
					};
			trailingAdvice?: DiagnosticAdvice[];
		},
	): Promise<void> {
		const {origin, error, trailingAdvice = []} = opts;

		let diagnostic = this.deriveDiagnosticFromErrorStructure(
			getErrorStructure(error),
		);
		let {location} = diagnostic;
		let {advice} = diagnostic.description;

		let test: undefined | TestDetails;
		if (origin.type === "TEST") {
			test = origin.test;
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

	private async teardownTest(test: TestDetails, api: TestAPI): Promise<void> {
		api.clearTimeout();

		try {
			await api.teardownEvent.callOptional();
		} catch (err) {
			await this.emitError({
				origin: {type: "TEST", test},
				error: err,
				trailingAdvice: api.getAdvice(),
			});
		}
	}

	private async runTest(test: TestDetails) {
		const {callback} = test;

		let onTimeout: OnTimeout = () => {
			throw new Error("Promise wasn't created. Should be impossible.");
		};

		const timeoutPromise = new Promise((resolve, reject) => {
			onTimeout = (time: number) => {
				reject(new Error(`Test timeout - exceeded ${String(time)}ms`));
			};
		});

		const api = new TestAPI(this, test, onTimeout);
		const helper = api.getUserSafeHelper();

		try {
			const {diagnostics} = await catchDiagnostics(async () => {
				this.runningSyncTest = api;
				const res = callback(helper);
				this.runningSyncTest = undefined;

				if (res !== undefined && typeof res.then === "function") {
					await Promise.race([timeoutPromise, res]);
				}
			});

			if (diagnostics !== undefined) {
				for (const diag of diagnostics) {
					await api.emitDiagnostic(diag);
				}
			}
		} catch (err) {
			await this.emitError({
				origin: {type: "TEST", test},
				error: err,
				trailingAdvice: api.getAdvice(),
			});
		} finally {
			await this.teardownTest(test, api);
			await this.bridge.events.testFinish.call({
				success: !this.failedTests.has(test.name),
				ref: this.createTestRef(test),
			});
		}
	}

	public async run(runOptions: TestWorkerRunTestOptions): Promise<void> {
		for (const testName of runOptions.testNames) {
			const test = this.foundTests.assert(testName);
			const {options} = test;

			this.bridge.events.testStart.send({
				ref: this.createTestRef(test),
				timeout: options.timeout,
			});

			await this.runTest(test);
		}
	}

	public getConsoleAdvice(): TestConsoleAdvice {
		const advice: TestConsoleAdvice = [];
		for (const [factory, time] of this.consoleAdvice) {
			advice.push([factory(), time]);
		}
		return advice;
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

		const foundTests: TestWorkerPrepareTestResult["foundTests"] = new Map();

		for (const [key, {callsiteLocation}] of this.foundTests) {
			foundTests.set(key, callsiteLocation);
		}

		return {
			foundTests,
			focusedTests: this.focusedTests,
		};
	}
}
