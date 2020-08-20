/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {
	ClientRequestFlags,
	DEFAULT_CLIENT_FLAGS,
	DEFAULT_CLIENT_REQUEST_FLAGS,
} from "../common/types/client";
import {BundlerConfig, FileReference, LAG_INTERVAL} from "@internal/core";
import {
	Diagnostic,
	DiagnosticAdvice,
	DiagnosticCategory,
	DiagnosticDescription,
	DiagnosticLocation,
	DiagnosticSuppressions,
	Diagnostics,
	DiagnosticsError,
	DiagnosticsProcessor,
	createSingleDiagnosticError,
	deriveDiagnosticFromError,
	descriptions,
	diagnosticLocationToMarkupFilelink,
	getDiagnosticsFromError,
	getOrDeriveDiagnosticsFromError,
} from "@internal/diagnostics";
import {
	DiagnosticsPrinter,
	DiagnosticsPrinterFlags,
	DiagnosticsPrinterOptions,
	printDiagnostics,
} from "@internal/cli-diagnostics";
import {ProjectDefinition} from "@internal/project";
import {ResolverOptions} from "./fs/Resolver";

import ServerBridge, {
	ServerQueryRequest,
	ServerQueryResponse,
	ServerQueryResponseSuccess,
} from "../common/bridges/ServerBridge";
import Server, {
	ServerClient,
	ServerMarker,
	ServerUnfinishedMarker,
} from "./Server";
import {Reporter, ReporterNamespace} from "@internal/cli-reporter";
import {Event} from "@internal/events";
import {
	FlagValue,
	SerializeCLITarget,
	serializeCLIFlags,
} from "@internal/cli-flags";
import {AnyRoot} from "@internal/ast";
import {TransformStageName} from "@internal/compiler";
import WorkerBridge, {
	PrefetchedModuleSignatures,
	WorkerAnalyzeDependencyResult,
	WorkerBufferPatch,
	WorkerCompileResult,
	WorkerCompilerOptions,
	WorkerFormatResult,
	WorkerLintOptions,
	WorkerLintResult,
	WorkerParseOptions,
	WorkerUpdateInlineSnapshotResult,
} from "../common/bridges/WorkerBridge";
import {ModuleSignature} from "@internal/js-analysis";
import {
	AbsoluteFilePath,
	AbsoluteFilePathMap,
	AbsoluteFilePathSet,
	AnyFilePath,
	UnknownPath,
	createAbsoluteFilePath,
	createUnknownPath,
} from "@internal/path";
import {Dict, RequiredProps, mergeObjects} from "@internal/typescript-helpers";
import {ob1Coerce0, ob1Number0, ob1Number1} from "@internal/ob1";
import {markup, readMarkup} from "@internal/markup";
import {DiagnosticsProcessorOptions} from "@internal/diagnostics/DiagnosticsProcessor";
import {JSONObject} from "@internal/codec-json";
import {VCSClient} from "@internal/vcs";
import {InlineSnapshotUpdates} from "../test-worker/SnapshotManager";
import {CacheEntry} from "./Cache";
import {FormatterOptions} from "@internal/formatter";
import {RecoverySaveFile} from "./fs/RecoveryStore";
import crypto = require("crypto");
import {GlobOptions, Globber} from "./fs/glob";

type ServerRequestOptions = {
	server: Server;
	client: ServerClient;
	query: ServerQueryRequest;
};

let requestIdCounter = 0;

type NormalizedCommandFlags = {
	flags: undefined | Dict<unknown>;
	defaultFlags: Dict<unknown>;
};

export const EMPTY_SUCCESS_RESPONSE: ServerQueryResponseSuccess = {
	type: "SUCCESS",
	hasData: false,
	data: undefined,
	markers: [],
	files: {},
};

type WrapRequestDiagnosticOpts = {
	noRetry?: boolean;
};

type ServerRequestGlobOptions = Omit<GlobOptions, "args" | "relativeDirectory"> & {
	args?: Array<UnknownPath | string>;
	tryAlternateArg?: (path: UnknownPath) => undefined | UnknownPath;
	ignoreArgumentMisses?: boolean;
	ignoreProjectIgnore?: boolean;
	disabledDiagnosticCategory?: DiagnosticCategory;
	advice?: DiagnosticAdvice;
	verb?: string;
	noun?: string;
};

async function globUnmatched(
	req: ServerRequest,
	opts: ServerRequestGlobOptions,
	path: AbsoluteFilePath,
	location: DiagnosticLocation,
) {
	const {server} = req;
	const {configCategory, ignoreProjectIgnore} = opts;

	let category: DiagnosticCategory = "args/fileNotFound";

	let advice: DiagnosticAdvice = [...(opts.advice || [])];

	// Hint if all files were ignored
	if (configCategory !== undefined && !ignoreProjectIgnore) {
		const globber = await req.glob({
			...opts,
			ignoreProjectIgnore: true,
		});
		const withoutIgnore = await globber.get(false);

		if (withoutIgnore.size > 0) {
			advice.push({
				type: "log",
				category: "info",
				text: markup`The following files were ignored`,
			});

			advice.push({
				type: "list",
				list: Array.from(withoutIgnore, (path) => markup`${path}`),
				truncate: true,
			});

			const ignoreSource = server.projectManager.findProjectConfigConsumer(
				await req.server.projectManager.assertProject(path, location),
				(consumer) =>
					consumer.has(configCategory) &&
					consumer.get(configCategory).get("ignore")
				,
			);

			if (ignoreSource.value !== undefined) {
				const ignorePointer = ignoreSource.value.getDiagnosticLocation("value");

				advice.push({
					type: "log",
					category: "info",
					text: markup`Ignore patterns were defined here`,
				});

				advice.push({
					type: "frame",
					location: ignorePointer,
				});
			}
		}
	}

	throw createSingleDiagnosticError({
		location: {
			...location,
			marker: markup`${path}`,
		},
		description: {
			...descriptions.FLAGS.NO_FILES_FOUND(opts.noun),
			category,
			advice,
		},
	});
}

export class ServerRequestInvalid extends DiagnosticsError {
	constructor(message: string, diagnostics: Diagnostics, showHelp: boolean) {
		super(message, diagnostics);
		this.showHelp = showHelp;
	}

	public showHelp: boolean;
}

function hash(val: JSONObject): string {
	return val === undefined || Object.keys(val).length === 0
		? "none"
		: crypto.createHash("sha256").update(JSON.stringify(val)).digest("hex");
}

export class ServerRequestCancelled extends Error {
	constructor(reason: string) {
		super(
			"ServerRequest has been cancelled. This error is meant to be seen by Server",
		);

		this.reason = reason;
	}

	public reason: string;
}

export default class ServerRequest {
	constructor({query, server, client}: ServerRequestOptions) {
		this.query = query;
		this.server = server;
		this.bridge = client.bridge;
		this.reporter = query.silent ? new Reporter() : client.reporter.fork();
		this.client = client;

		this.start = Date.now();
		this.id = requestIdCounter++;
		this.cancelledReason = undefined;
		this.toredown = false;
		this.markers = [];
		this.normalizedCommandFlags = {
			flags: {},
			defaultFlags: {},
		};
		this.files = new AbsoluteFilePathMap();

		this.logger = server.logger.namespace(
			markup`[ServerRequest] Request #${this.id}:`,
		);

		this.markerEvent = new Event({
			name: "ServerRequest.marker",
		});
		this.cancelEvent = new Event({
			name: "ServerRequest.cancel",
		});
		this.endEvent = new Event({
			name: "ServerRequest.end",
			serial: true,
		});

		this.client.requestsInFlight.add(this);
	}

	public id: number;
	public query: ServerQueryRequest;
	public bridge: ServerBridge;
	public client: ServerClient;
	public logger: ReporterNamespace;
	public server: Server;
	public reporter: Reporter;
	public endEvent: Event<ServerQueryResponse, void>;
	public cancelEvent: Event<void, void>;
	public markerEvent: Event<ServerMarker, void>;

	private start: number;
	private normalizedCommandFlags: NormalizedCommandFlags;
	private markers: Array<ServerMarker>;
	private cancelledReason: undefined | string;
	private toredown: boolean;
	private files: AbsoluteFilePathMap<RecoverySaveFile>;

	public queueSaveFile(path: AbsoluteFilePath, opts: RecoverySaveFile) {
		this.files.set(path, opts);
	}

	public async flushFiles(): Promise<number> {
		const {files} = this;
		const {server} = this;
		const {logger} = server;

		if (files.size === 0) {
			this.logger.info(markup`No files to write`);
			return 0;
		} else if (this.query.noFileWrites) {
			this.logger.info(
				markup`Writing no files due to noFileWrites flag being set`,
			);
			return 0;
		}

		this.files = new AbsoluteFilePathMap();

		this.logger.info(markup`Flushing files`);
		logger.list(Array.from(files.keys(), (path) => markup`${path}`));

		// Need to capture this before as it will be modified by server.writeFiles
		const totalFiles = files.size;

		await this.server.recoveryStore.writeFiles(
			files,
			{
				unsafeWrites: this.query.requestFlags.unsafeWrites,
			},
			{
				onFileDone: () => {
					// Maybe a progress bar later?
				},
				beforeFileWrite: async (path, fd) => {
					const content = await fd.readFile();
					await this.server.recoveryStore.save(this, path, content);
				},
				unexpectedModified: (path, expectedMtime, actualMtime) => {
					this.logger.info(
						markup`Skipped writing file ${path} as the mtime ${actualMtime} of the file on disk was newer than when we read it at ${expectedMtime}`,
					);
					this.reporter.warn(
						markup`File <emphasis>${path}</emphasis> was not updated as it was changed since we read it`,
					);
				},
				expectedExists: (path) => {
					this.logger.info(
						markup`Skipped writing file ${path} as it does not exist when we expected it to`,
					);
					this.reporter.warn(
						markup`File <emphasis>${path}</emphasis> was not updated as it does not exist when we expected it to`,
					);
				},
				unexpectedExists: (path) => {
					this.logger.info(
						markup`Skipped writing file ${path} as it exists when we didn't expect it`,
					);
					this.reporter.warn(
						markup`File <emphasis>${path}</emphasis> was not written as it exists when we didn't expect it`,
					);
				},
			},
		);

		await this.server.recoveryStore.commit(this);
		this.logger.info(markup`Flushed ${totalFiles} files`);

		return totalFiles;
	}

	public updateRequestFlags(flags: Partial<ClientRequestFlags>) {
		this.query = {
			...this.query,
			requestFlags: {
				...this.query.requestFlags,
				...flags,
			},
		};
	}

	public async init() {
		if (this.query.requestFlags.collectMarkers) {
			this.markerEvent.subscribe((marker) => {
				this.markers.push(marker);
			});
		}

		await this.server.handleRequestStart(this);
	}

	public checkCancelled() {
		if (this.cancelledReason !== undefined) {
			throw new ServerRequestCancelled(this.cancelledReason);
		}
	}

	public async cancel(reason: string): Promise<void> {
		await this.cancelEvent.callOptional();
		this.cancelledReason = reason;
		await this.teardown({
			type: "CANCELLED",
			reason,
			markers: [],
		});
	}

	public async teardown(
		res: ServerQueryResponse,
	): Promise<undefined | ServerQueryResponse> {
		if (this.toredown) {
			return;
		}

		this.toredown = true;
		this.client.requestsInFlight.delete(this);

		this.logger.info(markup`Response type: ${String(res?.type)}`);
		if (res.type === "DIAGNOSTICS") {
			this.logDiagnostics(res.diagnostics);
		}

		// Output timing information
		if (this.query.requestFlags.timing) {
			const end = Date.now();
			this.reporter.info(
				markup`Request took <duration emphasis>${String(end - this.start)}</duration>`,
			);
		}

		// If the query asked for no data then strip all diagnostics and data values
		if (this.query.noData) {
			switch (res.type) {
				case "SUCCESS": {
					res = {
						...EMPTY_SUCCESS_RESPONSE,
						hasData: res.data !== undefined,
					};
					break;
				}

				case "DIAGNOSTICS": {
					res = {
						type: "DIAGNOSTICS",
						hasDiagnostics: res.hasDiagnostics,
						diagnostics: [],
						markers: [],
						files: {},
					};
					break;
				}

				case "INVALID_REQUEST": {
					res = {
						type: "INVALID_REQUEST",
						diagnostics: [],
						markers: [],
						showHelp: res.showHelp,
					};
					break;
				}
			}
		} else {
			switch (res.type) {
				case "SUCCESS":
				case "DIAGNOSTICS": {
					const files: Dict<RecoverySaveFile> = {};
					for (const [path, opts] of this.files) {
						files[path.join()] = opts;
					}
					res = {
						...res,
						files,
					};
					break;
				}
			}
		}

		if (res.type === "DIAGNOSTICS" || res.type === "SUCCESS") {
			await this.flushFiles();
		}

		// Add on markers
		res = {
			...res,
			markers: this.markers,
		};

		await this.endEvent.callOptional(res);
		await this.server.handleRequestEnd(this);
		return res;
	}

	public setNormalizedCommandFlags(normalized: NormalizedCommandFlags) {
		this.normalizedCommandFlags = normalized;
	}

	public async resolveEntryAssertPathArg(
		index: number,
		max: boolean = true,
	): Promise<AbsoluteFilePath> {
		this.expectArgumentLength(index + 1, max ? undefined : Infinity);
		const arg = this.query.args[index];

		return await this.server.resolver.resolveEntryAssertPath(
			{
				...this.getResolverOptionsFromFlags(),
				source: createUnknownPath(arg),
			},
			{location: this.getDiagnosticLocationFromFlags({type: "arg", key: index})},
		);
	}

	public async assertClientCwdProject(): Promise<ProjectDefinition> {
		const location = this.getDiagnosticLocationForClientCwd();
		return this.server.projectManager.assertProject(
			this.client.flags.cwd,
			location,
		);
	}

	public async getVCSClient(): Promise<VCSClient> {
		return this.server.projectManager.getVCSClient(
			await this.assertClientCwdProject(),
		);
	}

	public async maybeGetVCSClient(): Promise<undefined | VCSClient> {
		return this.server.projectManager.maybeGetVCSClient(
			await this.assertClientCwdProject(),
		);
	}

	private logDiagnostics(diagnostics: Diagnostics) {
		for (const diag of diagnostics) {
			this.logger.error(
				markup`Encountered diagnostic: ${diag.description.message}. Category: ${diag.description.category}. Location: ${diagnosticLocationToMarkupFilelink(
					diag.location,
				)}`,
			);
		}
	}

	public async printDiagnostics(
		{diagnostics, suppressions = [], printerOptions, excludeFooter}: {
			diagnostics: Diagnostics;
			suppressions?: DiagnosticSuppressions;
			printerOptions?: DiagnosticsPrinterOptions;
			excludeFooter?: boolean;
		},
	) {
		this.logDiagnostics(diagnostics);

		await printDiagnostics({
			diagnostics,
			suppressions,
			excludeFooter: excludeFooter !== false,
			printerOptions: {
				reporter: this.reporter,
				processor: this.createDiagnosticsProcessor(),
				wrapErrors: true,
				...printerOptions,
			},
		});
	}

	public createDiagnosticsProcessor(
		opts: DiagnosticsProcessorOptions = {},
	): DiagnosticsProcessor {
		return new DiagnosticsProcessor({
			markupOptions: this.reporter.markupOptions,
			...opts,
		});
	}

	public createDiagnosticsPrinter(
		processor: DiagnosticsProcessor = this.createDiagnosticsProcessor(),
	): DiagnosticsPrinter {
		processor.unshiftOrigin({
			category: "server",
			message: `${this.query.commandName} command was dispatched`,
		});

		return new DiagnosticsPrinter({
			processor,
			reporter: this.reporter,
			cwd: this.client.flags.cwd,
			wrapErrors: true,
			flags: this.getDiagnosticsPrinterFlags(),
			fileReaders: this.server.createDiagnosticsPrinterFileReaders(),
		});
	}

	private getDiagnosticsPrinterFlags(): DiagnosticsPrinterFlags {
		const {requestFlags} = this.query;
		return {
			auxiliaryDiagnosticFormat: requestFlags.auxiliaryDiagnosticFormat,
			grep: requestFlags.grep,
			inverseGrep: requestFlags.inverseGrep,
			showAllDiagnostics: requestFlags.showAllDiagnostics,
			verboseDiagnostics: requestFlags.verboseDiagnostics,
			maxDiagnostics: requestFlags.maxDiagnostics,
			fieri: requestFlags.fieri,
		};
	}

	public expectArgumentLength(
		min: number,
		max: number = min,
		advice: DiagnosticAdvice = [],
	) {
		const {args} = this.query;
		let message;

		let excessive = false;

		if (min === max) {
			if (args.length !== min) {
				if (min === 0) {
					message = markup`Expected no arguments`;
				} else {
					message = markup`Expected exactly <number emphasis>${String(min)}</number> arguments`;
				}
			}
		} else {
			if (args.length < min) {
				message = markup`Expected at least <number emphasis>${String(min)}</number> arguments`;
			}

			if (args.length > max) {
				excessive = true;
				message = markup`Expected no more than <number emphasis>${String(min)}</number> arguments`;
			}
		}

		if (message !== undefined) {
			const description = descriptions.FLAGS.INCORRECT_ARG_COUNT(
				excessive,
				message,
			);
			this.throwDiagnosticFlagError({
				target: {
					type: "arg-range",
					from: min,
					to: max,
				},
				description: {
					...description,
					advice: [...description.advice, ...advice],
				},
			});
		}
	}

	public throwDiagnosticFlagError(
		{
			description,
			target = "none",
			showHelp = true,
		}: {
			description: RequiredProps<Partial<DiagnosticDescription>, "message">;
			target?: SerializeCLITarget;
			showHelp?: boolean;
		},
	) {
		const location = this.getDiagnosticLocationFromFlags(target);

		let {category} = description;
		if (category === undefined) {
			category =
				typeof target !== "string" &&
				(target.type === "arg" || target.type === "arg-range")
					? "args/invalid"
					: "flags/invalid";
		}

		const diag: Diagnostic = {
			description: {
				advice: [],
				...description,
				category,
			},
			location,
		};

		throw new ServerRequestInvalid(
			readMarkup(description.message),
			[diag],
			showHelp,
		);
	}

	public getDiagnosticLocationForClientCwd(): DiagnosticLocation {
		const cwd = this.client.flags.cwd.join();
		return {
			sourceText: cwd,
			start: {
				line: ob1Number1,
				column: ob1Number0,
			},
			end: {
				line: ob1Number1,
				column: ob1Coerce0(cwd.length),
			},
			filename: "cwd",
		};
	}

	public getDiagnosticLocationFromFlags(
		target: SerializeCLITarget,
	): RequiredProps<DiagnosticLocation, "sourceText"> {
		const {query} = this;
		const clientFlags = this.client.flags;

		const flags: Dict<FlagValue> = {
			silent: clientFlags.silent,
			...this.query.requestFlags,
			...this.normalizedCommandFlags.flags,
		};

		const defaultFlags = {
			silent: DEFAULT_CLIENT_FLAGS.silent,
			...DEFAULT_CLIENT_REQUEST_FLAGS,
			...this.normalizedCommandFlags.defaultFlags,
			clientName: this.client.flags.clientName,
		};

		// Only include the cwd flag it was different from the cwd of the actual client
		if (!clientFlags.cwd.equal(clientFlags.realCwd)) {
			flags.cwd = clientFlags.cwd.join();
		}

		return serializeCLIFlags(
			{
				programName: "rome",
				commandName: query.commandName,
				flags,
				args: query.args,
				defaultFlags,
				incorrectCaseFlags: new Set(),
				shorthandFlags: new Set(),
				cwd: this.client.flags.cwd,
			},
			target,
		);
	}

	public getResolverOptionsFromFlags(): RequiredProps<ResolverOptions, "origin"> {
		const {requestFlags} = this.query;
		return {
			origin: this.client.flags.cwd,
			platform: requestFlags.resolverPlatform,
			scale: requestFlags.resolverScale,
			mocks: requestFlags.resolverMocks,
		};
	}

	public getBundlerConfigFromFlags(
		resolverOpts: Partial<ResolverOptions> = {},
	): BundlerConfig {
		return {
			inlineSourceMap: false,
			cwd: this.client.flags.cwd,
			resolver: mergeObjects(this.getResolverOptionsFromFlags(), resolverOpts),
		};
	}

	private normalizeCompileResult(res: WorkerCompileResult): WorkerCompileResult {
		const {projectManager} = this.server;

		// Turn all the cacheDependencies entries from 'absolute paths to UIDs
		return {
			...res,
			cacheDependencies: res.cacheDependencies.map((filename) => {
				return projectManager.getFileReference(createAbsoluteFilePath(filename)).uid;
			}),
		};
	}

	private startMarker(
		opts: Omit<ServerUnfinishedMarker, "start">,
	): ServerUnfinishedMarker {
		this.logger.info(markup`Started marker: ${opts.label}`);
		return {
			...opts,
			start: Date.now(),
		};
	}

	private endMarker(startMarker: ServerUnfinishedMarker): ServerMarker {
		const endMarker: ServerMarker = {
			...startMarker,
			end: Date.now(),
		};
		this.logger.info(markup`Completed marker: ${startMarker.label}`);
		this.markerEvent.send(endMarker);
		return endMarker;
	}

	private async wrapRequestDiagnostic<T>(
		method: string,
		path: AbsoluteFilePath,
		factory: (bridge: WorkerBridge, ref: FileReference) => Promise<T>,
		opts: WrapRequestDiagnosticOpts = {},
	): Promise<T> {
		await this.server.memoryFs.processingLock.wait();

		const {server} = this;
		const owner = await server.fileAllocator.getOrAssignOwner(path);
		const startMtime = server.memoryFs.maybeGetMtime(path);
		const start = Date.now();
		const lock = await server.requestFileLocker.getLock(path);
		const ref = server.projectManager.getFileReference(path);

		const interval = setInterval(
			() => {
				const took = Date.now() - start;
				this.reporter.warn(
					markup`Running <emphasis>${method}</emphasis> on <emphasis>${path}</emphasis> seems to be taking longer than expected. Have been waiting for <emphasis><duration>${String(
						took,
					)}</duration></emphasis>.`,
				);
			},
			LAG_INTERVAL,
		);

		const marker = this.startMarker({
			label: `${method}: ${ref.relative}`,
			facet: method,
			rowId: `worker ${owner.id}`,
		});

		try {
			const res: T = await factory(owner.bridge, ref);
			this.endMarker(marker);
			return res;
		} catch (err) {
			let diagnostics = getDiagnosticsFromError(err);

			if (diagnostics === undefined) {
				const diag = deriveDiagnosticFromError(
					err,
					{
						description: {
							category: "internalError/request",
						},
					},
				);

				throw createSingleDiagnosticError({
					...diag,
					description: {
						...diag.description,
						advice: [
							...diag.description.advice,
							{
								type: "log",
								category: "info",
								text: markup`Error occurred while requesting <emphasis>${method}</emphasis> for <filelink emphasis target="${ref.uid}" />`,
							},
						],
					},
				});
			} else {
				// We don't want to tamper with these
				throw err;
			}
		} finally {
			lock.release();
			clearInterval(interval);

			const endMtime = this.server.memoryFs.maybeGetMtime(path);
			if (endMtime !== startMtime && !opts.noRetry) {
				return this.wrapRequestDiagnostic(method, path, factory);
			}
		}
	}

	public async requestWorkerGetBuffer(
		path: AbsoluteFilePath,
	): Promise<string | undefined> {
		this.checkCancelled();

		return this.wrapRequestDiagnostic(
			"getBuffer",
			path,
			async (bridge, ref) => {
				return bridge.getBuffer.call({ref});
			},
		);
	}

	public async requestWorkerUpdateBuffer(
		path: AbsoluteFilePath,
		content: string,
	): Promise<void> {
		this.checkCancelled();

		await this.wrapRequestDiagnostic(
			"updateBuffer",
			path,
			async (bridge, ref) => {
				await bridge.updateBuffer.call({ref, content});
				this.server.memoryFs.addBuffer(path, content);
				await this.server.refreshFileEvent.push(path);
			},
			{noRetry: true},
		);
	}

	public async requestWorkerPatchBuffer(
		path: AbsoluteFilePath,
		patches: Array<WorkerBufferPatch>,
	): Promise<string> {
		this.checkCancelled();

		return this.wrapRequestDiagnostic(
			"patchBuffer",
			path,
			async (bridge, ref) => {
				const buffer = await bridge.patchBuffer.call({ref, patches});
				this.server.memoryFs.addBuffer(path, buffer);
				this.server.refreshFileEvent.push(path);
				return buffer;
			},
			{noRetry: true},
		);
	}

	public async requestWorkerClearBuffer(path: AbsoluteFilePath): Promise<void> {
		this.checkCancelled();

		await this.wrapRequestDiagnostic(
			"clearBuffer",
			path,
			async (bridge, ref) => {
				await bridge.clearBuffer.call({ref});
				this.server.memoryFs.clearBuffer(path);
				this.server.refreshFileEvent.push(path);
			},
			{noRetry: true},
		);
	}

	public async requestWorkerParse(
		path: AbsoluteFilePath,
		opts: WorkerParseOptions,
	): Promise<AnyRoot> {
		this.checkCancelled();

		return this.wrapRequestDiagnostic(
			"parse",
			path,
			(bridge, ref) => bridge.parse.call({ref, options: opts}),
		);
	}

	public async requestWorkerUpdateInlineSnapshots(
		path: AbsoluteFilePath,
		updates: InlineSnapshotUpdates,
		parseOptions: WorkerParseOptions,
	): Promise<WorkerUpdateInlineSnapshotResult> {
		this.checkCancelled();

		return this.wrapRequestDiagnostic(
			"updateInlineSnapshots",
			path,
			(bridge, ref) =>
				bridge.updateInlineSnapshots.call({ref, updates, parseOptions})
			,
		);
	}

	public async requestWorkerLint(
		path: AbsoluteFilePath,
		optionsWithoutModSigs: Omit<WorkerLintOptions, "prefetchedModuleSignatures">,
	): Promise<WorkerLintResult> {
		this.checkCancelled();

		const {cache} = this.server;
		const cacheEntry = await cache.get(path);

		const cacheKey = hash(optionsWithoutModSigs);
		const cached = cacheEntry.lint[cacheKey];
		if (cached !== undefined) {
			return cached;
		}

		const prefetchedModuleSignatures = await this.maybePrefetchModuleSignatures(
			path,
		);

		const options: WorkerLintOptions = {
			...optionsWithoutModSigs,
			prefetchedModuleSignatures,
		};

		const res = await this.wrapRequestDiagnostic(
			"lint",
			path,
			(bridge, ref) => bridge.lint.call({ref, options, parseOptions: {}}),
		);

		await cache.update(
			path,
			(cacheEntry) =>
				({
					lint: {
						...cacheEntry.lint,
						[cacheKey]: res,
					},
				} as CacheEntry)
			,
		);

		return res;
	}

	public async requestWorkerFormat(
		path: AbsoluteFilePath,
		options: FormatterOptions,
		parseOptions: WorkerParseOptions,
	): Promise<undefined | WorkerFormatResult> {
		this.checkCancelled();

		return await this.wrapRequestDiagnostic(
			"format",
			path,
			(bridge, ref) => bridge.format.call({ref, options, parseOptions}),
		);
	}

	public async requestWorkerCompile(
		path: AbsoluteFilePath,
		stage: TransformStageName,
		options: WorkerCompilerOptions,
		parseOptions: WorkerParseOptions,
	): Promise<WorkerCompileResult> {
		this.checkCancelled();

		const {cache} = this.server;

		// Create a cache key comprised of the stage and hash of the options
		const cacheKey = `${stage}:${hash(options)}`;

		// Check cache for this stage and options
		const cacheEntry = await cache.get(path);
		const cached = cacheEntry.compile[cacheKey];
		if (cached !== undefined) {
			// TODO check cacheDependencies
			return cached;
		}

		const compileRes = await this.wrapRequestDiagnostic(
			"compile",
			path,
			(bridge, ref) => {
				// We allow options to be passed in as undefined so we can compute an easy cache key
				if (options === undefined) {
					options = {};
				}

				return bridge.compile.call({ref, stage, options, parseOptions});
			},
		);

		const res = this.normalizeCompileResult({
			...compileRes,
			cached: false,
		});

		// There's a race condition here between the file being opened and then rewritten
		await cache.update(
			path,
			(cacheEntry) =>
				({
					compile: {
						...cacheEntry.compile,
						[cacheKey]: {
							...res,
							cached: true,
						},
					},
				} as CacheEntry)
			,
		);

		return res;
	}

	public async requestWorkerAnalyzeDependencies(
		path: AbsoluteFilePath,
		parseOptions: WorkerParseOptions,
	): Promise<WorkerAnalyzeDependencyResult> {
		this.checkCancelled();

		const {cache} = this.server;

		const cacheEntry = await cache.get(path);
		if (cacheEntry.analyzeDependencies !== undefined) {
			return cacheEntry.analyzeDependencies;
		}

		const res = await this.wrapRequestDiagnostic(
			"analyzeDependencies",
			path,
			(bridge, ref) => bridge.analyzeDependencies.call({ref, parseOptions}),
		);
		await cache.update(
			path,
			{
				analyzeDependencies: {
					...res,
					cached: true,
				},
			},
		);

		return {
			...res,
			cached: false,
		};
	}

	public async requestWorkerModuleSignature(
		path: AbsoluteFilePath,
		parseOptions: WorkerParseOptions,
	): Promise<ModuleSignature> {
		this.checkCancelled();

		const {cache} = this.server;

		const cacheEntry = await cache.get(path);
		if (cacheEntry.moduleSignature !== undefined) {
			return cacheEntry.moduleSignature;
		}

		const res = await this.wrapRequestDiagnostic(
			"moduleSignature",
			path,
			(bridge, ref) => bridge.moduleSignatureJS.call({ref, parseOptions}),
		);
		await cache.update(
			path,
			{
				moduleSignature: res,
			},
		);
		return res;
	}

	private async maybePrefetchModuleSignatures(
		path: AbsoluteFilePath,
	): Promise<PrefetchedModuleSignatures> {
		this.checkCancelled();

		const {projectManager} = this.server;

		const prefetchedModuleSignatures: PrefetchedModuleSignatures = {};
		const project = await projectManager.assertProject(path);
		if (!project.config.typeCheck.enabled) {
			return prefetchedModuleSignatures;
		}

		// get the owner of this file

		/*const rootOwner = await fileAllocator.getOrAssignOwner(filename);
    const rootOwnerId = workerManager.getIdFromBridge(rootOwner);

    // absolute filenames to redupe export graphs
    const absoluteFilenameToGraphKey: Map<string, string> = new Map();

    // TODO exclude graphs that aren't a part of the root graph
    for (const dep of await dependencyGraph.getTransitiveDependencies(
      filename,
    )) {
      const key = `${dep.origin}:${dep.relative}`;
      const absolute = dep.absoluteMocked;

      // TODO check if we have this graph by another key and point to it if necessary
      const existingEntryKey = absoluteFilenameToGraphKey.get(absolute);
      if (existingEntryKey !== undefined) {
        invariant(existingEntryKey !== key, 'duplicate transitive dependency key %s', key);
        prefetchedModuleSignatures[key] = {
          type: 'POINTER',
          key: existingEntryKey,
        };
        continue;
      }

      // set the key so we point to the value instead of reproducing the whole graph
      absoluteFilenameToGraphKey.set(absolute, key);

      // fetch the owner so we can leave out graphs owned by the worker
      const owner = await fileAllocator.getOrAssignOwner(absolute);
      if (owner === rootOwner) {
        const project = await projectManager.assertProject(absolute);
        prefetchedModuleSignatures[key] = {
          type: 'OWNED',
          filename: absolute,
          projectId: project.id,
        };
        continue;
      }

      // get mtime so we can use it for a cache
      const mtime = this.server.memoryFs.getMtime(absolute);

      // check if this worker has it cached
      // TODO figure out some way to evict this on file deletion
      const cacheKey = `moduleSignature:${absolute}`;
      const cachedMtime = workerManager.getValueFromWorkerCache(
        rootOwnerId,
        cacheKey,
      );
      if (cachedMtime === mtime) {
        prefetchedModuleSignatures[key] = {
          type: 'USE_CACHED',
          filename: absolute,
        };
        continue;
      } else {
        workerManager.setWorkerCacheValue(rootOwnerId, cacheKey, mtime);
      }

      // calculate the graph
      const graph = await this.moduleSignature(absolute);
      prefetchedModuleSignatures[key] = {
        type: 'RESOLVED',
        graph,
      };
    }*/
		return prefetchedModuleSignatures;
	}

	public async glob(opts: ServerRequestGlobOptions): Promise<Globber> {
		const {cwd} = this.client.flags;

		const argToLocation: AbsoluteFilePathMap<DiagnosticLocation> = new AbsoluteFilePathMap();
		const args: AbsoluteFilePathSet = new AbsoluteFilePathSet();

		let rawArgs: Array<string | AnyFilePath> = opts.args ?? this.query.args;
		if (rawArgs.length === 0) {
			rawArgs = [cwd];
			argToLocation.set(cwd, this.getDiagnosticLocationForClientCwd());
		}

		for (let i = 0; i < rawArgs.length; i++) {
			const path = createUnknownPath(rawArgs[i]);
			let abs: AbsoluteFilePath;

			if (path.isAbsolute()) {
				abs = path.assertAbsolute();
			} else {
				const resolved = cwd.resolve(path);
				if (await this.server.memoryFs.existsHard(resolved)) {
					abs = resolved;
				} else {
					// Will need to be resolved...
					abs = await this.resolveEntryAssertPathArg(i, false);
				}
			}

			let exists = await this.server.memoryFs.existsHard(abs);

			// If it doesn't exist then let's try finding an alternate path
			if (!exists && opts.tryAlternateArg !== undefined) {
				const alternateSource = opts.tryAlternateArg(path);
				if (alternateSource !== undefined) {
					const resolvedAlternate = await this.server.resolver.resolveEntry({
						origin: cwd,
						source: alternateSource,
						// Allow requests to stop at directories
						requestedType: "directory",
					});
					if (resolvedAlternate.type === "FOUND") {
						abs = resolvedAlternate.path;
						exists = true;
					}
				}
			}

			if (exists) {
				args.add(abs);

				if (!argToLocation.has(abs)) {
					argToLocation.set(
						abs,
						this.getDiagnosticLocationFromFlags({
							type: "arg",
							key: i,
						}),
					);
				}
			} else {
				// This should fail. Resolver produces much nicer error messages.
				await this.resolveEntryAssertPathArg(i, false);
			}
		}

		return new Globber(
			this.server,
			{
				...opts,
				args,

				onWatch: (sub) => {
					this.endEvent.subscribe(async () => {
						await sub.unsubscribe();
					});
				},

				onSearchNoMatch: async (path) => {
					if (!opts.ignoreArgumentMisses) {
						const location = argToLocation.get(path) ?? {};
						await this.server.projectManager.assertProject(path, location);
						await globUnmatched(this, opts, path, location);
					}
				},
			},
		);
	}

	public async buildResponseFromError(
		rawErr: Error,
	): Promise<ServerQueryResponse> {
		if (!this.bridge.alive) {
			// Doesn't matter
			return {
				type: "CANCELLED",
				reason: "dead",
				markers: [],
			};
		}

		let err = rawErr;
		let printer: DiagnosticsPrinter;

		if (err instanceof DiagnosticsPrinter) {
			printer = err;
		} else {
			// If we can derive diagnostics from the error then create a diagnostics printer
			const diagnostics = getOrDeriveDiagnosticsFromError(
				err,
				{
					description: {
						category: "internalError/request",
					},
					tags: {
						internal: true,
					},
				},
			);

			printer = this.createDiagnosticsPrinter(
				this.createDiagnosticsProcessor({
					origins: [
						{
							category: "internal",
							message: "Derived diagnostics from thrown error",
						},
					],
				}),
			);

			printer.processor.addDiagnostics(diagnostics);
		}

		// Only print when the bridge is alive and we aren't in review mode
		// When we're in review mode we don't expect to show any diagnostics because they'll be intercepted in the client command
		// We will always print invalid request errors
		let shouldPrint = true;
		if (this.query.requestFlags.review) {
			shouldPrint = false;
		}
		if (rawErr instanceof ServerRequestInvalid) {
			shouldPrint = true;
		}
		if (!this.bridge.alive) {
			shouldPrint = false;
		}

		if (shouldPrint) {
			await printer.print();

			// Don't output the footer if this is a notifier for an invalid request as it will be followed by a help screen
			if (!(rawErr instanceof ServerRequestInvalid)) {
				await printer.footer();
			}
		}

		const diagnostics = printer.processor.getDiagnostics();

		if (err instanceof ServerRequestCancelled) {
			return {
				type: "CANCELLED",
				reason: err.reason,
				markers: [],
			};
		} else if (err instanceof ServerRequestInvalid) {
			return {
				type: "INVALID_REQUEST",
				diagnostics,
				showHelp: err.showHelp,
				markers: [],
			};
		} else {
			return {
				type: "DIAGNOSTICS",
				files: {},
				hasDiagnostics: diagnostics.length > 0,
				diagnostics,
				markers: [],
			};
		}
	}
}
