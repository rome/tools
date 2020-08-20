import {Server, ServerRequest, TestWorkerBridge} from "@internal/core";
import {
	ErrorFrame,
	InspectorClient,
	InspectorClientCloseError,
	urlToFilename,
} from "@internal/v8";
import workerThreads = require("worker_threads");
import {forkThread} from "@internal/core/common/utils/fork";
import {createBridgeFromWorkerThread} from "@internal/events";
import {createClient} from "@internal/codec-websocket";
import {TestWorkerFlags} from "@internal/core/test-worker/TestWorker";
import TestServer, {BridgeDiagnosticsError} from "@internal/core/server/testing/TestServer";
import {ob1Coerce0To1} from "@internal/ob1";
import {deriveDiagnosticFromErrorStructure} from "@internal/diagnostics";
import {markup} from "@internal/markup";
import {ReporterProgress} from "@internal/cli-reporter";
import {AbsoluteFilePathMap, AbsoluteFilePathSet} from "@internal/path";
import {ansiEscapes} from "@internal/cli-layout";
import {FilePathLocker} from "@internal/async/lockers";
import TestServerFile from "@internal/core/server/testing/TestServerFile";

export default class TestServerWorker {
	constructor(
		{server, request, runner, flags}: {
			server: Server;
			runner: TestServer;
			flags: TestWorkerFlags;
			request: ServerRequest;
		},
	) {
		this.server = server;
		this.runner = runner;
		this.request = request;

		this.thread = forkThread(
			"test-worker",
			{
				workerData: flags,
				stdin: true,
				stdout: true,
				stderr: true,
			},
		);

		this.bridge = createBridgeFromWorkerThread(
			TestWorkerBridge,
			this.thread,
			{
				type: "client",
			},
		);

		this.inspector = undefined;

		this.prepareLock = new FilePathLocker();
		this.preparedPaths = new AbsoluteFilePathSet();
		this.transferredCompiled = new AbsoluteFilePathSet();
	}

	private server: Server;
	private request: ServerRequest;
	private runner: TestServer;
	private transferredCompiled: AbsoluteFilePathSet;
	private preparedPaths: AbsoluteFilePathSet;
	private prepareLock: FilePathLocker;

	public bridge: TestWorkerBridge;
	public thread: workerThreads.Worker;
	public inspector: undefined | InspectorClient;

	public async init() {
		const {thread, bridge, runner} = this;
		const {stdout, stderr} = thread;

		stdout.on(
			"data",
			(chunk) => {
				process.stdout.write(chunk);
			},
		);

		// Suppress any debugger logs
		stderr.on(
			"data",
			(chunk) => {
				const str = chunk.toString();

				if (str.startsWith("Waiting for the debugger to disconnect...")) {
					if (this.inspector !== undefined) {
						this.inspector.end();
						return;
					}
				}

				process.stderr.write(chunk);
			},
		);

		await bridge.handshake();

		bridge.monitorHeartbeat(
			5_000,
			async () => {
				this.server.wrapFatalPromise(this.handleTimeout("10 seconds"));
			},
		);

		// Start debugger
		const {inspectorUrl} = await bridge.inspectorDetails.call();
		if (inspectorUrl !== undefined) {
			const client = new InspectorClient(await createClient(inspectorUrl));
			this.inspector = client;

			await client.call("Debugger.enable");

			// When a debugger is attached there's always a "Debugger attached" log emitted
			// This is written to stderr from native and there's no way for us to intercept it, and no way to disable it
			// https://github.com/nodejs/node/issues/34799
			// Until we have a way to disable it we need to resort to grossness like this...
			process.stderr.write(ansiEscapes.cursorUp());
			process.stderr.write(ansiEscapes.eraseLine);

			bridge.endEvent.subscribe(() => {
				client.end();
			});
		}

		bridge.testDiagnostic.subscribe(({testPath, diagnostic, origin}) => {
			if (testPath !== undefined) {
				this.runner.files.assert(testPath).onDiagnostics();
			}

			runner.printer.processor.addDiagnostic(diagnostic, origin);
		});
	}

	public async handleTimeout(duration: string): Promise<void> {
		return new Promise((resolve, reject) => {
			const timeout = setTimeout(
				() => {
					resolve(
						this.bridge.end(
							`Test worker was unresponsive for ${duration}. We tried to collect some additional metadata but we timed out again trying to fetch it...`,
						),
					);
				},
				3_000,
			);

			this._handleTimeout(duration).then(() => {
				clearTimeout(timeout);
				resolve();
			}).catch((err) => {
				clearTimeout(timeout);
				if (err instanceof InspectorClientCloseError) {
					return this.bridge.end(
						`Test worker was unresponsive for ${duration}. We tried to collect some additional metadata but the inspector connection closed abruptly`,
					);
				} else {
					reject(err);
				}
			});
		});
	}

	private async _handleTimeout(duration: string): Promise<void> {
		const {inspector, bridge} = this;
		if (inspector === undefined) {
			bridge.end(
				`Test worker was unresponsive for ${duration}. There was no inspector connected so we were unable to capture stack frames before it was terminated.`,
			);
			return undefined;
		}

		inspector.call("Debugger.pause");

		const params = await inspector.wait("Debugger.paused");

		const frames: Array<ErrorFrame> = [];

		const callFrames = Array.from(params.get("callFrames").asIterable()).slice(
			0,
			20,
		);
		for (const callFrame of callFrames) {
			const loc = callFrame.get("location");

			const resolved = this.runner.sourceMaps.assertApproxOriginalPositionFor(
				urlToFilename(callFrame.get("url").asString()),
				ob1Coerce0To1(loc.get("lineNumber").asZeroIndexedNumber()),
				loc.get("columnNumber").asZeroIndexedNumber(),
			);

			const name = callFrame.get("scopeChain").getIndex(0).get("name").asString(
				"",
			).split("$").pop();

			frames.push({
				resolvedLocation: resolved.found,
				typeName: undefined,
				functionName: name,
				methodName: undefined,
				filename: resolved.source,
				lineNumber: resolved.line,
				columnNumber: resolved.column,
				isTopLevel: false,
				isEval: false,
				isNative: false,
				isConstructor: false,
				isAsync: false,
			});
		}

		bridge.endWithError(
			new BridgeDiagnosticsError(
				deriveDiagnosticFromErrorStructure(
					{
						name: "Error",
						frames,
					},
					{
						description: {
							category: "tests/timeout",
							message: markup`Test worker was unresponsive for <emphasis>${duration}</emphasis>. Possible infinite loop. Below is a stack trace before the test was terminated.`,
							advice: [
								{
									type: "log",
									category: "info",
									text: markup`You can find the specific test that caused this by running <code>rome test --sync-tests</code>`,
								},
							],
						},
					},
				),
				bridge,
			),
		);
	}

	public async prepareAll(
		progress: ReporterProgress,
		fileQueue: Array<TestServerFile>,
	): Promise<void> {
		const {inspector, runner} = this;
		const {options: opts} = runner;

		if (inspector !== undefined && opts.coverage) {
			await inspector.call("Profiler.enable");
			await inspector.call(
				"Profiler.startPreciseCoverage",
				{
					// Turning this on disables V8 optimizations https://v8.dev/blog/javascript-code-coverage#precise-coverage-(function-granularity)
					callCount: false,
					// Otherwise coverage will only have function granularity
					detailed: true,
				},
			);
		}

		while (fileQueue.length > 0) {
			const file = fileQueue.pop()!;
			await this.prepareTest({file, progress, partial: false});
		}
	}

	public async prepareTest(
		{file, progress, partial}: {
			partial: boolean;
			file: TestServerFile;
			progress?: ReporterProgress;
		},
	) {
		const {bridge, runner} = this;
		const globalOptions = runner.options;
		const {ref, bundle} = file;
		const req = this.request;
		const {flags} = req.client;

		const path = ref.real;
		const lock = this.prepareLock.getNewLock(path);

		this.preparedPaths.add(path);

		let progressId;
		if (progress !== undefined) {
			progressId = progress.pushText(markup`<filelink target="${ref.uid}" />`);
		}

		try {
			const assembled = bundle.entry.js.assembled;

			// Transfer over compiled code that this test worker needs to assemble the file but doesn't have
			const pending: AbsoluteFilePathMap<string> = new AbsoluteFilePathMap();
			for (const item of assembled) {
				if (item[0] === 1) {
					const path = item[1];
					if (!this.transferredCompiled.has(path)) {
						this.transferredCompiled.add(path);
						const compiled = bundle.bundler.compiles.get(path);
						if (compiled !== undefined) {
							pending.set(path, compiled.compiledCode);
						}
					}
				}
			}
			if (pending.size > 0) {
				await bridge.receiveCompiled.call(pending);
			}

			const {focusedTests, foundTests} = await bridge.prepareTest.call({
				globalOptions,
				partial,
				projectDirectory: req.server.projectManager.assertProjectExisting(
					ref.real,
				).directory.join(),
				path,
				cwd: flags.cwd.join(),
				assembled,
				logFound: true,
			});

			if (!partial) {
				for (const testName of foundTests) {
					runner.onTestFound({testName, path});
				}

				for (const test of focusedTests) {
					runner.focusedTests.push(test);
				}
			}
		} catch (err) {
			runner.handlePossibleBridgeError(err);
		}

		lock.release();

		if (progress !== undefined && progressId !== undefined) {
			progress.popText(progressId);
			progress.tick();
		}
	}

	private async runTest() {
		const {bridge} = this;

		// Find a test we've already prepared
		for (const path of this.preparedPaths) {
			const file = this.runner.files.assert(path);

			for (const testName of file.getPendingTests()) {
				file.removePendingTest(testName);
				await this.prepareLock.waitLock(path);
				await bridge.runTest.call({
					path,
					testNames: [testName],
				});
				await this.runTest();
			}
		}

		// Prepare another
		while (this.runner.testFilesStack.length > 0) {
			const path = this.runner.testFilesStack.shift()!;
			const file = this.runner.files.assert(path);
			const pendingTests = file.getPendingTests();

			if (pendingTests.size === 0) {
				continue;
			} else {
				this.runner.testFilesStack.push(path);
			}

			await this.prepareTest({partial: true, file, progress: undefined});
			await this.runTest();
		}
	}

	public async run() {
		const {bridge, thread, inspector, runner} = this;
		const {options: opts} = runner;

		try {
			const promises: Array<Promise<void>> = [];
			for (let i = 0; i < 10; i++) {
				promises.push(this.runTest());
			}
			await Promise.all(promises);

			for (const path of this.preparedPaths) {
				const result = await bridge.teardownTest.call(path);
				await this.runner.files.assert(path).addResult(result);
			}
		} catch (err) {
			runner.handlePossibleBridgeError(err);
		} finally {
			if (inspector !== undefined) {
				if (opts.coverage) {
					if (inspector.alive) {
						const profile = await inspector.call("Profiler.takePreciseCoverage");
						runner.coverageCollector.addCoverage(profile.get("result").asAny());

						// Not really necessary but let's clean up anyway for completeness
						await inspector.call("Profiler.stopPreciseCoverage");
						await inspector.call("Profiler.disable");
					} else {
						// TODO log that we failed to fetch some coverage
					}
				}

				inspector.end();
			}

			await thread.terminate();
		}
	}
}
