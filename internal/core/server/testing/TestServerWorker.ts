import {Server, ServerRequest, WorkerBridge} from "@internal/core";
import {ErrorFrame} from "@internal/errors";
import {
	InspectorClient,
	InspectorClientCloseError,
	urlToFilename,
} from "@internal/v8";
import {createWebSocketClient} from "@internal/codec-websocket";
import TestServer from "@internal/core/server/testing/TestServer";
import {
	DIAGNOSTIC_CATEGORIES,
	createSingleDiagnosticsError,
	deriveDiagnosticFromErrorStructure,
} from "@internal/diagnostics";
import {markup} from "@internal/markup";
import {ReporterProgress} from "@internal/cli-reporter";
import {
	AbsoluteFilePathMap,
	AbsoluteFilePathSet,
	createPath,
} from "@internal/path";
import {ansiEscapes} from "@internal/cli-layout";
import {PathLocker} from "@internal/async/lockers";
import TestServerFile from "@internal/core/server/testing/TestServerFile";
import {BridgeServer} from "@internal/events";
import {Duration} from "@internal/numbers";
import {ThreadWorkerContainer} from "@internal/core/worker/types";
import {createResourceFromTimeout} from "@internal/resources";

export default class TestServerWorker {
	constructor(
		{server, request, runner, container}: {
			server: Server;
			container: ThreadWorkerContainer;
			runner: TestServer;
			request: ServerRequest;
		},
	) {
		this.server = server;
		this.runner = runner;
		this.request = request;

		this.thread = container.thread;
		this.bridge = container.bridge;

		this.inspector = undefined;

		this.prepareLock = new PathLocker();
		this.preparedPaths = new AbsoluteFilePathSet();
		this.transferredCompiled = new AbsoluteFilePathSet();
	}

	private server: Server;
	private request: ServerRequest;
	private runner: TestServer;
	private transferredCompiled: AbsoluteFilePathSet;
	private preparedPaths: AbsoluteFilePathSet;
	private prepareLock: PathLocker;

	public bridge: BridgeServer<typeof WorkerBridge>;
	public thread: ThreadWorkerContainer["thread"];
	public inspector: undefined | InspectorClient;

	public async init() {
		const {bridge, runner} = this;

		bridge.startHeartbeatMonitor(
			Duration.fromSeconds(10),
			() => {
				this.server.fatalErrorHandler.wrapPromise(
					this.handleTimeout("10 seconds"),
				);
			},
		);

		// Start debugger
		const {inspectorUrl} = await bridge.events.inspectorDetails.call();
		if (inspectorUrl !== undefined) {
			const client = new InspectorClient(
				await createWebSocketClient(inspectorUrl),
			);
			this.inspector = client;
			this.thread.resources.add(client);

			await client.call("Debugger.enable");

			// When a debugger is attached there's always a "Debugger attached" log emitted
			// This is written to stderr from native and there's no way for us to intercept it, and no way to disable it
			// https://github.com/nodejs/node/issues/34799
			// Until we have a way to disable it we need to resort to grossness like this...
			process.stderr.write(ansiEscapes.cursorUp() + ansiEscapes.eraseLine);
		}

		bridge.events.testDiskSnapshotDiscovered.subscribe((
			{testPath, snapshotPath},
		) => {
			this.runner.files.assert(testPath).discoveredDiskSnapshot(
				snapshotPath,
				this,
			);
		});

		bridge.events.testSnapshotEntry.subscribe(({testPath, snapshotPath, entry}) => {
			this.runner.files.assert(testPath).addSnapshotEntry(snapshotPath, entry);
		});

		bridge.events.testInlineSnapshotUpdate.subscribe(({testPath, update}) => {
			this.runner.files.assert(testPath).addInlineSnapshotUpdate(update);
		});

		bridge.events.testDiagnostic.subscribe(({ref, diagnostic}) => {
			if (ref !== undefined) {
				this.runner.files.assert(ref.path).onDiagnostics();
			}

			runner.addDiagnostic(diagnostic, ref);
		});
	}

	public async handleTimeout(duration: string): Promise<void> {
		return new Promise((resolve, reject) => {
			const timeout = createResourceFromTimeout(
				"TimeoutResolver",
				setTimeout(
					() => {
						resolve(
							this.bridge.end(
								`Test worker was unresponsive for ${duration}. We tried to collect some additional metadata but we timed out again trying to fetch it...`,
								false,
							),
						);
					},
					3_000,
				),
			);

			if (this.inspector === undefined) {
				timeout.release();
			} else {
				this.inspector.resources.add(timeout);
			}

			this._handleTimeout(duration).then(() => {
				timeout.release();
				resolve();
			}).catch((err) => {
				timeout.release();
				if (err instanceof InspectorClientCloseError) {
					this.bridge.end(
						`Test worker was unresponsive for ${duration}. We tried to collect some additional metadata but the inspector connection closed abruptly`,
						false,
					);
				} else {
					reject(err);
				}
			});
		});
	}

	private async _handleTimeout(duration: string): Promise<void> {
		const {inspector, bridge} = this;
		if (inspector === undefined || !inspector.alive) {
			await bridge.end(
				`Test worker was unresponsive for ${duration}. There was no inspector connected so we were unable to capture stack frames before it was terminated.`,
				false,
			);
			return undefined;
		}

		inspector.call("Debugger.pause");

		const params = await inspector.wait("Debugger.paused");

		const frames: ErrorFrame[] = [];

		const callFrames = Array.from(params.get("callFrames").asIterable()).slice(
			0,
			20,
		);
		for (const callFrame of callFrames) {
			const loc = callFrame.get("location");

			const resolved = this.runner.sourceMaps.assertApproxOriginalPositionFor(
				createPath(urlToFilename(callFrame.get("url").asString())),
				loc.get("lineNumber").asZeroIndexedNumber().toOneIndexed(),
				loc.get("columnNumber").asZeroIndexedNumber(),
			);

			const name = callFrame.get("scopeChain").getIndex(0).get("name").default(
				"",
			).asString().split("$").pop();

			frames.push({
				resolvedLocation: resolved.found,
				typeName: undefined,
				functionName: name,
				methodName: undefined,
				path: resolved.source,
				lineNumber: resolved.line,
				columnNumber: resolved.column,
				isTopLevel: false,
				isEval: false,
				isNative: false,
				isConstructor: false,
				isAsync: false,
			});
		}

		await bridge.endWithError(
			createSingleDiagnosticsError(
				deriveDiagnosticFromErrorStructure(
					{
						name: "Error",
						frames,
					},
					{
						description: {
							category: DIAGNOSTIC_CATEGORIES["tests/timeout"],
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
			),
		);
	}

	public async prepareAll(
		progress: ReporterProgress,
		fileQueue: TestServerFile[],
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
			progressId = progress.pushText(ref.uid);
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
							pending.set(path, compiled.value.compiledCode);
						}
					}
				}
			}
			if (pending.size > 0) {
				await bridge.events.testReceiveCompiledDependency.call(pending);
			}

			const {focusedTests, foundTests} = await bridge.events.testPrepare.call({
				globalOptions,
				partial,
				contextDirectory: req.server.projectManager.getRootProjectForPath(
					ref.real,
				).directory,
				projectDirectory: req.server.projectManager.assertProjectExisting(
					ref.real,
				).directory.join(),
				path,
				cwd: flags.cwd.join(),
				assembled,
				logFound: true,
			});

			if (!partial) {
				for (const [testName, callsiteLocation] of foundTests) {
					runner.onTestFound({testName, path}, callsiteLocation);
				}

				for (const test of focusedTests) {
					runner.focusedTests.push(test);
				}
			}
		} catch (err) {
			runner.handlePossibleBridgeError(err, bridge);
		}

		lock.release();

		if (progress !== undefined && progressId !== undefined) {
			progress.popText(progressId);
			progress.tick();
		}
	}

	private async runTest(): Promise<void> {
		const {bridge} = this;

		// Find a test we've already prepared
		for (const path of this.preparedPaths) {
			const file = this.runner.files.assert(path);

			for (const testName of file.getPendingTests()) {
				file.removePendingTest(testName, this);
				await this.prepareLock.waitLock(path);
				await bridge.events.testRun.call({
					path,
					testNames: [testName],
				});
				if (file.markCompletedTest()) {
					// Start this async
					await Promise.all([file.teardown(), this.runTest()]);
					return;
				} else {
					return this.runTest();
				}
			}

			// Exhausted all tests in this file, no longer consider it for future tests
			this.preparedPaths.delete(path);
		}

		// Prepare another file with pending tests
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
			return this.runTest();
		}
	}

	public async run() {
		const {bridge, inspector, runner} = this;
		const {options: opts} = runner;

		try {
			const promises: Promise<void>[] = [];
			for (let i = 0; i < 5; i++) {
				promises.push(this.runTest());
			}
			await Promise.all(promises);
		} catch (err) {
			runner.handlePossibleBridgeError(err, bridge);
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

				await inspector.resources.release();
			}
		}
	}
}
