import {IS_ROME_DEV_ENV} from "@internal/cli-environment";
import {Reporter} from "@internal/cli-reporter";
import {BridgeServer} from "@internal/events";
import {markup} from "@internal/markup";
import {Resource, createResourceFromCallback} from "@internal/resources";
import ServerRequest, {EMPTY_SUCCESS_RESPONSE} from "./ServerRequest";
import ServerBridge, {
	PartialServerQueryRequest,
	ServerProfileWorker,
	ServerQueryRequest,
	ServerQueryResponse,
} from "../common/bridges/ServerBridge";
import Server, {partialServerQueryRequestToFull} from "./Server";
import {
	ClientFlags,
	ClientRequestFlags,
	DEFAULT_CLIENT_FLAGS,
} from "../common/types/client";
import {VERSION} from "../common/constants";
import {DurationMeasurer} from "@internal/numbers";
import {DIAGNOSTIC_CATEGORIES, descriptions} from "@internal/diagnostics";
import {ConsumePath, consume} from "@internal/consume";
import {createUIDPath} from "@internal/path";
import {Dict} from "@internal/typescript-helpers";
import {ServerCommand, serverCommands} from "./commands";
import {toKebabCase} from "@internal/string-utils";

export default class ServerClient {
	constructor(
		server: Server,
		id: number,
		bridge: BridgeServer<typeof ServerBridge>,
	) {
		this.server = server;
		this.id = id;
		this.bridge = bridge;

		this.requestsInFlight = new Set();

		this.reporter = new Reporter(
			"ServerClient",
			{
				wrapperFactory: server.fatalErrorHandler.wrapBound,
				markupOptions: server.logger.markupOptions,
			},
		);

		this.resources = server.resources.createContainer(
			`ServerClient<${id}>`,
			[bridge, this.reporter],
		);

		this.flags = DEFAULT_CLIENT_FLAGS;
	}

	private server: Server;

	public id: number;
	public bridge: BridgeServer<typeof ServerBridge>;
	public resources: Resource;
	public requestsInFlight: Set<ServerRequest>;
	public flags: ClientFlags;
	public reporter: Reporter;

	public async init(): Promise<void> {
		const {bridge, server} = this;

		bridge.events.profilingGetWorkers.subscribe(async () => {
			const workers: ServerProfileWorker[] = [];
			for (const {id, displayName} of server.workerManager.getExternalWorkers()) {
				workers.push({id, displayName});
			}
			return workers;
		});

		bridge.events.profilingStopWorker.subscribe(async (id) => {
			const worker = server.workerManager.getWorkerAssert(id);
			return await worker.bridge.events.profilingStop.call();
		});

		bridge.resources.add(
			createResourceFromCallback(
				"BridgeEndServerRequestCancellationHandler",
				async () => {
					for (const req of this.requestsInFlight) {
						await req.cancel("client disconnected");
					}
				},
			),
		);

		await bridge.handshake();

		const client = await this.setup();

		bridge.events.query.subscribe(async (request) => {
			return await this.handleRequest(request);
		});

		bridge.events.cancelQuery.subscribe(async (token) => {
			for (const req of this.requestsInFlight) {
				if (req.query.cancelToken === token) {
					await req.cancel("user requested");
				}
			}
		});

		bridge.events.endServer.subscribe(() => server.end());

		return client;
	}

	private async setup(): Promise<void> {
		const {bridge, server, reporter} = this;

		const {
			flags,
			streamState,
			outputFormat,
			outputSupport,
			version,
		} = await bridge.events.getClientInfo.call();

		const stream = reporter.addStream(
			{
				format: outputFormat,
				features: outputSupport,
				write(chunk: string, error: boolean) {
					if (flags.silent && !error) {
						return;
					}

					bridge.events.write.send([chunk, error]);
				},
			},
			streamState,
		);

		if (version !== VERSION) {
			reporter.error(
				markup`Client version ${version} does not match server version ${VERSION}. Goodbye lol.`,
			);
			await this.bridge.end();
			return;
		}

		bridge.events.updateFeatures.subscribe((features) => {
			return stream.updateFeatures(features);
		});

		// Update ServerClient props
		this.flags = flags;
		reporter.updateMarkupOptions({
			cwd: flags.cwd,
		});

		// Add reporter to connected set, important logs may be output to these
		server.connectedReporters.addAttachedStream(stream);

		// Warn about disabled disk caching. Don't bother if it's only been set due to ROME_DEV. We don't care to see it in development.
		if (server.cache.writeDisabled && !IS_ROME_DEV_ENV) {
			reporter.warn(
				markup`Disk caching has been disabled due to the <emphasis>ROME_CACHE=0</emphasis> environment variable`,
			);
		}

		bridge.endEvent.subscribe(() => {
			// Cancel any requests still in flight
			for (const req of this.requestsInFlight) {
				req.cancel("bridge died");
			}
		});
	}

	public async handleRequest(
		partialQuery: PartialServerQueryRequest,
	): Promise<ServerQueryResponse> {
		const {server} = this;

		const query: ServerQueryRequest = partialServerQueryRequestToFull(
			partialQuery,
		);

		const req = new ServerRequest({
			client: this,
			query,
			server,
		});
		await req.init();

		try {
			let res: ServerQueryResponse = await this.dispatchRequest(req, []);
			res = await req.teardown(res);
			return res;
		} catch (err) {
			await server.fatalErrorHandler.handle(err);
			throw new Error("Process should have quit already");
		}
	}

	private async dispatchBenchmarkRequest(
		req: ServerRequest,
	): Promise<ServerQueryResponse> {
		const {client} = req;
		const {reporter} = client;
		const {benchmarkIterations} = req.query.requestFlags;

		// Warmup
		const warmupStart = new DurationMeasurer();
		const result = await this.dispatchRequest(req, ["benchmark"]);
		const warmupTook = warmupStart.since();

		// Benchmark
		const progress = req.reporter.progress({title: markup`Running benchmark`});
		progress.setTotal(benchmarkIterations);
		const benchmarkStart = new DurationMeasurer();
		for (let i = 0; i < benchmarkIterations; i++) {
			await this.dispatchRequest(req, ["benchmark"]);
			progress.tick();
		}
		progress.end();
		const benchmarkTook = benchmarkStart.since();

		await reporter.section(
			markup`Benchmark results`,
			() => {
				reporter.info(
					markup`Request artifacts may have been cached after the first run, artificially decreasing subsequent run time`,
				);
				reporter.heading(markup`Query`);
				reporter.inspect(req.query);
				reporter.heading(markup`Stats`);
				reporter.list([
					markup`Warmup took <emphasis>${warmupTook}</emphasis>`,
					markup`<number emphasis>${String(benchmarkIterations)}</number> runs`,
					markup`<emphasis>${benchmarkTook}</emphasis> total`,
					markup`<emphasis>${benchmarkTook.divide(benchmarkIterations, true)}</emphasis> per run`,
				]);
			},
		);

		return result;
	}

	private async dispatchRequest(
		req: ServerRequest,
		origins: string[],
	): Promise<ServerQueryResponse> {
		const {query} = req;
		const {requestFlags} = query;

		if (requestFlags.benchmark && !origins.includes("benchmark")) {
			return this.dispatchBenchmarkRequest(req);
		}

		try {
			const defaultCommandFlags: Dict<unknown> = {};

			// A type-safe wrapper for retrieving command flags
			// TODO perhaps present this as JSON or something if this isn't a request from the CLI?
			const flagsConsumer = consume({
				path: createUIDPath("argv"),
				parent: undefined,
				value: query.commandFlags,
				onDefinition(def) {
					// objectPath should only have a depth of 1
					defaultCommandFlags[def.objectPath[0]] = def.default;
				},
				objectPath: [],
				context: {
					category: DIAGNOSTIC_CATEGORIES["flags/invalid"],
					getOriginalValue: () => {
						return undefined;
					},
					normalizeKey: (key) => {
						return toKebabCase(key);
					},
					getDiagnosticLocation: (keys: ConsumePath) => {
						return req.getDiagnosticLocationFromFlags({
							type: "flag",
							key: String(keys[0]),
							target: "value",
						});
					},
				},
			});

			// Get command
			const serverCommand: undefined | ServerCommand<Dict<unknown>> = serverCommands.get(
				query.commandName,
			);
			if (serverCommand) {
				await validateRequestFlags(req, serverCommand);

				let commandFlags;
				if (serverCommand.defineFlags !== undefined) {
					commandFlags = serverCommand.defineFlags(flagsConsumer);
				}

				req.setNormalizedCommandFlags({
					flags: commandFlags,
					defaultFlags: defaultCommandFlags,
				});

				// @ts-ignore
				const data = await serverCommand.callback(req, commandFlags);
				return {
					...EMPTY_SUCCESS_RESPONSE,
					hasData: data !== undefined,
					data,
				};
			} else {
				req.throwDiagnosticFlagError({
					target: {
						type: "arg",
						key: 0,
					},
					description: descriptions.FLAGS.UNKNOWN_COMMAND({
						programName: "rome",
						commandName: query.commandName,
					}),
				});
			}
		} catch (err) {
			return await req.buildResponseFromError(err);
		}
	}
}

const disallowedFlagsWhenReviewing: Array<keyof ClientRequestFlags> = ["watch"];

async function validateRequestFlags(
	req: ServerRequest,
	serverCommand: ServerCommand<Dict<unknown>>,
) {
	const {requestFlags} = req.query;

	// Commands need to explicitly allow these flags
	validateAllowedRequestFlag(req, "watch", serverCommand);
	validateAllowedRequestFlag(req, "review", serverCommand);

	// Don't allow review in combination with other flags
	if (requestFlags.review) {
		for (const key of disallowedFlagsWhenReviewing) {
			if (requestFlags[key]) {
				throw req.throwDiagnosticFlagError({
					description: descriptions.FLAGS.DISALLOWED_REVIEW_FLAG(key),
					target: {type: "flag", key},
				});
			}
		}
	}
}

function validateAllowedRequestFlag(
	req: ServerRequest,
	flagKey: NonNullable<ServerCommand<Dict<unknown>>["allowRequestFlags"]>[number],
	serverCommand: ServerCommand<Dict<unknown>>,
) {
	const allowRequestFlags = serverCommand.allowRequestFlags || [];
	if (req.query.requestFlags[flagKey] && !allowRequestFlags.includes(flagKey)) {
		throw req.throwDiagnosticFlagError({
			description: descriptions.FLAGS.DISALLOWED_REQUEST_FLAG(flagKey),
			target: {type: "flag", key: flagKey},
		});
	}
}
