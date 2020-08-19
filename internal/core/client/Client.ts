/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {
	ClientFlags,
	ClientTerminalFeatures,
	DEFAULT_CLIENT_FLAGS,
} from "../common/types/client";
import ClientRequest, {ClientRequestType} from "./ClientRequest";
import Server, {ServerClient, ServerOptions} from "../server/Server";
import {
	CLI_SOCKET_PATH,
	SERVER_SOCKET_PATH,
	ServerBridge,
	ServerQueryResponse,
	VERSION,
} from "@internal/core";
import {forkProcess} from "../common/utils/fork";
import {
	BridgeError,
	Event,
	EventSubscription,
	createBridgeFromLocal,
	createBridgeFromSocket,
	createEmptySubscription,
	createSubscriptionHelper,
} from "@internal/events";
import {Reporter, ReporterDerivedStreams} from "@internal/cli-reporter";
import prettyFormat from "@internal/pretty-format";

import {TarWriter} from "@internal/codec-tar";
import {Profile, Profiler, Trace, TraceEvent} from "@internal/v8";
import {PartialServerQueryRequest} from "../common/bridges/ServerBridge";
import {UserConfig, getUserConfigFile} from "../common/userConfig";
import {createWriteStream, removeFile} from "@internal/fs";
import {stringifyJSON} from "@internal/codec-json";
import stream = require("stream");
import net = require("net");
import zlib = require("zlib");
import os = require("os");
import child = require("child_process");
import {Dict, mergeObjects} from "@internal/typescript-helpers";
import {
	AnyMarkup,
	concatMarkup,
	convertToMarkupFromRandomString,
	joinMarkupLines,
	markup,
} from "@internal/markup";

import {
	markupToHtml,
	markupToJoinedPlainText,
	markupToPlainText,
} from "@internal/cli-layout";
import {AbsoluteFilePath} from "@internal/path";
import {NodeSystemError} from "@internal/node";

export function getFilenameTimestamp(): string {
	return new Date().toISOString().replace(/[^0-9a-zA-Z]/g, "");
}

const NEW_SERVER_INIT_TIMEOUT = 10_000;

type ClientOptions = {
	terminalFeatures: ClientTerminalFeatures;
	globalErrorHandlers: boolean;
	stdout: stream.Writable;
	stderr: stream.Writable;
	stdin: NodeJS.ReadStream;
	userConfig: UserConfig;
	flags: Partial<Omit<ClientFlags, "clientName">>;
};

export type ClientProfileOptions = {
	samplingInterval: number;
	timeoutInterval: undefined | number;
	includeWorkers: boolean;
};

type ProfileCallback = (profile: Array<TraceEvent>) => Promise<void>;

type BridgeStatus = BridgeStatusDedicated | BridgeStatusLocal;

type BridgeStatusDedicated = {
	bridge: ServerBridge;
	dedicated: true;
	socket: net.Socket;
};

type BridgeStatusLocal = {
	bridge: ServerBridge;
	server: Server;
	dedicated: false;
};

type ClientRequestResponseResult = {
	request: PartialServerQueryRequest;
	response: ServerQueryResponse;
};

export default class Client {
	constructor(opts: ClientOptions) {
		this.options = opts;
		this.userConfig = opts.userConfig;
		this.queryCounter = 0;
		this.flags = mergeObjects<ClientFlags>(DEFAULT_CLIENT_FLAGS, opts.flags);

		this.requestResponseEvent = new Event({
			name: "Client.requestResponseEvent",
		});
		this.endEvent = new Event({name: "Client.endEvent", serial: true});
		this.bridgeStatus = undefined;

		this.bridgeAttachedEvent = new Event({
			name: "Client.bridgeAttached",
		});

		this.reporter = new Reporter({
			stdin: opts.stdin,
			markupOptions: {
				userConfig: this.userConfig,
				cwd: this.flags.cwd,
			},
		});
		this.reporter.redirectOutToErr(true);

		this.derivedReporterStreams = this.reporter.attachStdoutStreams(
			// Suppress stdout when silent is set
			this.flags.silent ? undefined : opts.stdout,
			opts.stderr,
			this.options.terminalFeatures,
		);
	}

	public reporter: Reporter;
	public derivedReporterStreams: ReporterDerivedStreams;

	private queryCounter: number;
	private userConfig: UserConfig;
	private options: ClientOptions;
	private flags: ClientFlags;
	private bridgeStatus: undefined | BridgeStatus;

	public bridgeAttachedEvent: Event<BridgeStatus, void>;
	private requestResponseEvent: Event<ClientRequestResponseResult, void>;
	public endEvent: Event<void, void>;

	private assertBridgeStatus(): BridgeStatus {
		const {bridgeStatus} = this;
		if (bridgeStatus === undefined) {
			throw new Error("Expected a connected bridge but found none");
		}
		return bridgeStatus;
	}

	private async onBridge(
		callback: (
			bridgeStatus: BridgeStatus,
		) => Promise<EventSubscription | undefined>,
	): Promise<EventSubscription> {
		if (this.bridgeStatus === undefined) {
			const helper = createSubscriptionHelper();

			helper.add(
				this.bridgeAttachedEvent.subscribe(async (bridgeStatus) => {
					const subscription = await callback(bridgeStatus);
					if (subscription !== undefined) {
						helper.add(subscription);
					}
				}),
			);

			return helper;
		} else {
			const subscription = await callback(this.bridgeStatus);
			return subscription ?? createEmptySubscription();
		}
	}

	private getBridgeStatus(): undefined | BridgeStatus {
		return this.bridgeStatus;
	}

	public setFlags(flags: Partial<ClientFlags>) {
		if (this.bridgeStatus !== undefined) {
			throw new Error(
				"Already connected to bridge. Cannot change client flags.",
			);
		}

		this.flags = {
			...this.flags,
			...flags,
		};
	}

	public async profile(opts: ClientProfileOptions, callback: ProfileCallback) {
		this.reporter.info(markup`Starting CPU profile...`);
		return this._profile(opts, callback);
	}

	private async _profile(opts: ClientProfileOptions, callback: ProfileCallback) {
		const {samplingInterval, timeoutInterval, includeWorkers} = opts;

		// Start server and start profiling
		const bridge = await this.findOrStartServer();
		await bridge.profilingStart.call({
			samplingInterval,
		});

		// Start cli profiling
		let cliProfiler: undefined | Profiler;
		const bridgeStatus = this.getBridgeStatus();
		if (bridgeStatus === undefined || bridgeStatus.dedicated) {
			cliProfiler = new Profiler();
			await cliProfiler.startProfiling(samplingInterval);
		}

		// Start a profile timer if one was specified
		let hasProfiled: undefined | Promise<void>;
		let timeout: undefined | NodeJS.Timeout;
		if (timeoutInterval !== undefined) {
			timeout = setTimeout(
				() => {
					hasProfiled = stopProfile(true);
				},
				timeoutInterval,
			);
		}

		const stopProfile = async (isTimeout: boolean) => {
			// This is to prevent stopping the profile multiple times via the timeout and then at the end
			// It's a promise so that the final stopProfile call will block until the first has finished
			if (hasProfiled) {
				return hasProfiled;
			}

			// Stop the timeout if it hasn't been triggered
			if (timeout !== undefined) {
				clearTimeout(timeout);
			}

			//
			const trace = new Trace();
			const fetchers: Array<[string, () => Promise<Profile>]> = [];

			// CLI
			if (cliProfiler !== undefined) {
				const cliProfilerAssert = cliProfiler;
				fetchers.push([
					"CLI",
					async () => {
						return cliProfilerAssert.stopProfiling();
					},
				]);
			}

			// Server
			fetchers.push([
				cliProfiler === undefined ? "Server/CLI" : "Server",
				async () => {
					return await bridge.profilingStop.call(
						undefined,
						{
							priority: true,
						},
					);
				},
			]);

			// Workers
			if (includeWorkers) {
				const workerIds = await bridge.profilingGetWorkers.call();
				for (const id of workerIds) {
					fetchers.push([
						`Worker ${id}`,
						async () => {
							return await bridge.profilingStopWorker.call(
								id,
								{
									priority: true,
								},
							);
						},
					]);
				}
			}

			// Fetch profiles
			const progress = this.reporter.progress({title: markup`Fetching profiles`});
			progress.setTotal(fetchers.length);
			for (const [text, callback] of fetchers) {
				progress.setText(markup`${text}`);
				const profile = await callback();
				trace.addProfile(text, profile);
				progress.tick();
			}
			progress.end();

			const events = trace.build();
			await callback(events);

			// If we're a timeout than separate these logs from the
			if (isTimeout) {
				this.reporter.hr();
			}
		};

		this.endEvent.subscribe(() => {
			return stopProfile(false);
		});
	}

	public subscribeLogs(
		includeWorkerLogs: boolean,
		callback: (chunk: string) => void,
	): Promise<EventSubscription> {
		return this.onBridge(async ({bridge}) => {
			if (includeWorkerLogs) {
				await bridge.enableWorkerLogs.call();
			}

			return bridge.log.subscribe(({origin, chunk}) => {
				if (origin === "worker" && !includeWorkerLogs) {
					// We allow multiple calls to bridge.enableWorkerLogs
					// Filter the event if necessary if it wasn't requested by this log subscription
					return;
				}

				callback(chunk);
			});
		});
	}

	public async generateRageSummary(): Promise<AnyMarkup> {
		let summary: Array<AnyMarkup> = [];

		function push(name: string, value: unknown) {
			const formatted =
				typeof value === "string"
					? markup`${value}`
					: prettyFormat(
							value,
							{
								compact: true,
							},
						);
			summary.push(
				markup`<emphasis>${name}</emphasis>\n<indent>${formatted}</indent>\n\n`,
			);
		}

		const envVars: Array<string> = [
			"ROME_CACHE",
			"LANG",
			"COLORFGBG",

			// Variables used by process.stdout.getColorDepth
			"FORCE_COLOR",
			"NODE_DISABLE_COLORS",
			"NO_COLOR",
			"TERM",
			"TMUX",
			"CI",
			"TRAVIS",
			"CIRCLECI",
			"APPVEYOR",
			"GITLAB_CI",
			"CI_NAME",
			"TEAMCITY_VERSION",
			"TERM_PROGRAM",
			"COLORTERM",
		];
		const env: Dict<string | undefined> = {};
		for (const key of envVars) {
			env[key] = process.env[key];
		}
		push("Environment Variables", env);

		const userConfig = await getUserConfigFile();
		push(
			"User Config",
			userConfig === undefined ? "unset" : userConfig.consumer.asUnknown(),
		);

		push("Rome Version", VERSION);
		push("Node Version", process.versions.node);
		push("Platform", `${process.platform} ${process.arch} ${os.release()}`);
		push("Terminal Features", this.derivedReporterStreams.features);
		push("Client Flags", this.flags);

		// Don't do this if we never connected to the server
		const bridgeStatus = this.getBridgeStatus();
		if (bridgeStatus !== undefined) {
			const status = await this.query(
				{
					silent: true,
					commandName: "status",
				},
				"server",
			);
			if (status.type === "SUCCESS") {
				push("Server Status", status.data);
			}
		}

		return concatMarkup(summary);
	}

	public async rage(
		ragePath: AbsoluteFilePath,
		profileOpts: ClientProfileOptions,
	) {
		const {bridge} = this.assertBridgeStatus();

		this.reporter.info(markup`Rage enabled \u{1f620}`);

		let logsHTML = "";
		let logsPlain = "";
		await this.subscribeLogs(
			true,
			(chunk) => {
				logsPlain += joinMarkupLines(
					markupToPlainText(convertToMarkupFromRandomString(chunk)),
				);
				logsHTML += joinMarkupLines(
					markupToHtml(convertToMarkupFromRandomString(chunk)),
				);
			},
		);

		// Collect CPU profile
		// Callback will be called later once it has been collected
		// Initial async work is just connecting to the processes and setting up handlers
		let profileEvents: Array<TraceEvent> = [];
		await this._profile(
			profileOpts,
			async (_profileEvents) => {
				profileEvents = _profileEvents;
			},
		);

		// Collect all responses
		const responses: Array<ClientRequestResponseResult> = [];
		this.requestResponseEvent.subscribe((result) => {
			responses.push(result);
		});

		// Capture terminal output
		let output = "";
		const writeEvent = bridge.write.subscribe(([chunk]) => {
			output += chunk;
		});

		this.endEvent.subscribe(async () => {
			const stream = zlib.createGzip();
			stream.pipe(createWriteStream(ragePath));

			const writer = new TarWriter(stream);

			writer.append({name: "profile.json"}, stringifyJSON(profileEvents));
			writer.append({name: "logs.txt"}, logsPlain);
			writer.append({name: "logs.html"}, `<pre><code>${logsHTML}</code></pre>`);
			writer.append({name: "output.txt"}, output);

			await writeEvent.unsubscribe();

			// Add requests
			for (let i = 0; i < responses.length; i++) {
				const {request, response} = responses[i];
				// If there are multiple responses then use a directory otherwise just dump it in the root
				const dirname =
					responses.length === 1 ? "" : `requests/${i}-${request.commandName}/`;
				writer.append({name: `${dirname}request.json`}, stringifyJSON(request));
				writer.append(
					{name: `${dirname}response.json`},
					stringifyJSON(response),
				);
			}

			writer.append(
				{name: "summary.txt"},
				markupToJoinedPlainText(await this.generateRageSummary()),
			);

			await writer.finalize();
			this.reporter.success(
				markup`Rage archive written to <emphasis>${ragePath}</emphasis>`,
			);
		});
	}

	public async query(
		query: PartialServerQueryRequest,
		type?: ClientRequestType,
	): Promise<ServerQueryResponse> {
		const request = new ClientRequest(this, type, query);
		const res = await request.init();
		this.requestResponseEvent.send({request: query, response: res});
		return res;
	}

	public cancellableQuery(
		query: PartialServerQueryRequest,
		type?: ClientRequestType,
	): {
		promise: Promise<ServerQueryResponse>;
		cancel: () => Promise<void>;
	} {
		const cancelToken = String(this.queryCounter++);

		return {
			promise: this.query(
				{
					...query,
					cancelToken,
				},
				type,
			),
			cancel: async () => {
				const status = this.getBridgeStatus();
				if (status !== undefined) {
					await status.bridge.cancelQuery.call(cancelToken);
				}
			},
		};
	}

	public async shutdownServer() {
		await this._shutdownServer();
		await this.end();
	}

	private async _shutdownServer() {
		const status = this.bridgeStatus;
		if (status !== undefined && status.bridge.alive) {
			try {
				await status.bridge.endServer.call();
			} catch (err) {
				// Swallow BridgeErrors since we expect one to be emitted as the endServer call will be an unanswered request
				// when the server ends all client sockets
				if (!(err instanceof BridgeError)) {
					throw err;
				}
			}
		}
	}

	public async end() {
		await this.endEvent.callOptional();

		const status = this.bridgeStatus;

		if (status !== undefined && status.bridge.alive) {
			if (status.dedicated) {
				status.socket.end();
			} else {
				await this._shutdownServer();
			}
		}

		this.reporter.teardown();
		this.bridgeStatus = undefined;
	}

	private async attachBridge(status: BridgeStatus) {
		const {handle, featuresUpdated, features, format} = this.derivedReporterStreams;
		const {terminalFeatures = {}} = this.options;

		if (this.bridgeStatus !== undefined) {
			throw new Error("Already attached bridge to API");
		}

		this.bridgeStatus = status;

		const {bridge} = status;

		bridge.write.subscribe(([chunk, error]) => {
			const isError = error && !terminalFeatures.redirectError;
			handle.stream.write(chunk, isError);
		});

		// Listen for resize column events if stdout is a TTY
		featuresUpdated.subscribe((features) => {
			bridge.updateFeatures.call(features);
		});

		await Promise.all([
			bridge.getClientInfo.wait({
				version: VERSION,
				outputFormat: format,
				outputSupport: features,
				streamState: {
					...handle.stream.state,
					lineSnapshots: undefined,
				},
				flags: this.flags,
			}),
			bridge.handshake(),
			bridge.serverReady.wait(),
		]);

		await this.bridgeAttachedEvent.callOptional(status);
	}

	public async findOrStartServer(): Promise<ServerBridge> {
		// First check if we already have a bridge connection
		const connected = this.getBridgeStatus();
		if (connected !== undefined) {
			return connected.bridge;
		}

		// Then check if there's already a running daemon
		const runningDaemon = await this.tryConnectToExistingDaemon();
		if (runningDaemon) {
			return runningDaemon;
		}

		const status = await this.startInternalServer();
		return status.bridge;
	}

	public async startInternalServer(
		opts?: Partial<ServerOptions>,
	): Promise<{
		bridge: ServerBridge;
		server: Server;
		serverClient: ServerClient;
	}> {
		// Otherwise, start a server inside this process
		const server = new Server({
			userConfig: this.userConfig,
			dedicated: false,
			globalErrorHandlers: this.options.globalErrorHandlers === true,
			...opts,
		});
		await server.init();

		const bridge = createBridgeFromLocal(ServerBridge, {});
		const status: BridgeStatusLocal = {bridge, server, dedicated: false};

		const [serverClient] = await Promise.all([
			server.attachToBridge(bridge),
			this.attachBridge(status),
		]);

		this.endEvent.subscribe(async () => {
			await server.end();
		});

		return {serverClient, bridge, server};
	}

	public async forceStartDaemon(): Promise<ServerBridge> {
		const daemon = await this.startDaemon();
		if (daemon === undefined) {
			this.reporter.error(markup`Failed to start daemon`);
			throw new Error("Failed to start daemon");
		} else {
			return daemon;
		}
	}

	public async startDaemon(): Promise<undefined | ServerBridge> {
		const {reporter} = this;

		if (this.bridgeStatus !== undefined) {
			throw new Error("Already started server");
		}

		reporter.info(markup`No running daemon found. Starting one...`);

		let exited = false;
		let proc: undefined | child.ChildProcess;

		const newDaemon: undefined | ServerBridge = await new Promise((
			resolve,
			reject,
		) => {
			const timeout = setTimeout(
				() => {
					reporter.error(markup`Daemon connection timed out`);
					cleanup();
					resolve();
				},
				NEW_SERVER_INIT_TIMEOUT,
			);

			const socketServer = net.createServer(() => {
				cleanup();

				resolve(
					this.tryConnectToExistingDaemon().then((bridge) => {
						if (bridge !== undefined) {
							this.reporter.success(markup`Started daemon!`);
						}
						return bridge;
					}),
				);
			});

			socketServer.on("error", reject);

			function listen() {
				socketServer.listen(CLI_SOCKET_PATH.join());

				proc = forkProcess(
					"server",
					{
						detached: true,
					},
				);
				proc.unref();

				proc.on(
					"close",
					() => {
						exited = true;
						cleanup();
						resolve();
					},
				);
			}

			removeFile(CLI_SOCKET_PATH).finally(() => {
				listen();
			});

			function cleanup() {
				clearTimeout(timeout);
				socketServer.close();
			}
		});
		if (newDaemon) {
			return newDaemon;
		}

		// as a final precaution kill the server
		if (exited) {
			reporter.error(markup`Daemon died while initialising.`);
		} else {
			reporter.error(markup`Failed to connect. Killing daemon.`);
		}

		if (proc !== undefined) {
			proc.kill();
		}

		return undefined;
	}

	public async tryConnectToExistingDaemon(): Promise<undefined | ServerBridge> {
		if (this.bridgeStatus !== undefined) {
			return this.bridgeStatus.bridge;
		}

		const promise: Promise<undefined | net.Socket> = new Promise((
			resolve,
			reject,
		) => {
			const socket = net.createConnection(
				{
					path: SERVER_SOCKET_PATH.join(),
				},
				() => {
					resolve(socket);
				},
			);

			socket.on(
				"error",
				(err: NodeSystemError) => {
					if (
						err.code === "ENOENT" ||
						err.code === "ECONNREFUSED" ||
						err.code === "EADDRINUSE"
					) {
						resolve();
					} else {
						reject(err);
					}
				},
			);
		});

		const socket = await promise;
		if (socket === undefined) {
			return undefined;
		}

		const bridge = createBridgeFromSocket(
			ServerBridge,
			socket,
			{
				type: "server",
			},
		);
		await this.attachBridge({socket, bridge, dedicated: true});
		this.reporter.success(markup`Connected to daemon`);
		return bridge;
	}
}
