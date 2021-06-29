/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import Server from "../Server";
import ServerClient from "../ServerClient";
import {
	AbsoluteFilePath,
	AbsoluteFilePathMap,
	AbsoluteFilePathSet,
} from "@internal/path";
import {
	DIAGNOSTIC_CATEGORIES,
	Diagnostic,
	DiagnosticsProcessor,
	formatCategoryDescription,
} from "@internal/diagnostics";
import {
	PartialServerQueryRequest,
	ServerQueryResponse,
} from "@internal/core/common/bridges/ServerBridge";
import Checker from "../checker/Checker";
import ServerRequest, {EMPTY_SUCCESS_RESPONSE} from "../ServerRequest";
import {DEFAULT_CLIENT_REQUEST_FLAGS} from "@internal/core/common/types/client";
import {
	ReporterProgress,
	ReporterProgressOptions,
} from "@internal/cli-reporter";
import {LSPTransport} from "./protocol";
import LSPProgress from "./LSPProgress";
import {convertDiagnosticsToLSP} from "./utils";
import {markup, readMarkup} from "@internal/markup";
import {Event} from "@internal/events";
import {CommandName} from "@internal/core/common/commands";
import {notificationHandlers, requestHandlers} from "./messages";

type LSPProjectSession = {
	request: ServerRequest;
	status: "PENDING" | "WATCHING";
};

export default class LSPServer {
	constructor(request: ServerRequest) {
		this.request = request;
		this.server = request.server;
		this.client = request.client;

		this.lintSessionsPending = new AbsoluteFilePathSet();
		this.projectSessions = new AbsoluteFilePathMap();
		this.fileBuffers = new AbsoluteFilePathSet();
		this.fileVersions = new AbsoluteFilePathMap();

		this.watchProjectEvent = new Event("LSPServer.watchProject");

		request.endEvent.subscribe(async () => {
			await this.shutdown();
		});

		const transport = new LSPTransport(this.server.logger);
		this.transport = transport;

		transport.notificationEvent.subscribe(({method, params}) => {
			return notificationHandlers.get(method)?.(this, params, method);
		});

		transport.requestEvent.subscribe(({method, params}) => {
			return requestHandlers.get(method)?.(this, params, method);
		});

		transport.errorEvent.subscribe((err) => {
			request.server.fatalErrorHandler.handle(err);
		});

		this.diagnosticsProcessor = this.createDiagnosticsProcessor();
	}

	public transport: LSPTransport;
	public request: ServerRequest;
	public client: ServerClient;
	public server: Server;
	public diagnosticsProcessor: DiagnosticsProcessor;

	public fileBuffers: AbsoluteFilePathSet;
	public fileVersions: AbsoluteFilePathMap<number>;
	private lintSessionsPending: AbsoluteFilePathSet;
	private projectSessions: AbsoluteFilePathMap<LSPProjectSession>;

	public watchProjectEvent: Event<AbsoluteFilePath, void>;

	private createDiagnosticsProcessor(): DiagnosticsProcessor {
		// We want to filter pendingFixes because we'll autoformat the file on save if necessary and it's just noise
		const processor = this.request.createDiagnosticsProcessor({
			mutable: true,
			filter: undefined,
		});
		processor.addEliminationFilter({
			category: DIAGNOSTIC_CATEGORIES["lint/pendingFixes"],
		});
		return processor;
	}

	public logMessage(path: AbsoluteFilePath, message: string) {
		this.server.logger.info(markup`[LSPServer] ${message}`);
		this.transport.write({
			method: "window/logMessage",
			params: {
				type: 4,
				uri: `file://${path.join()}`,
				message,
			},
		});
	}

	public logDiagnostics(path: AbsoluteFilePath, diagnostics: Diagnostic[] = []) {
		if (diagnostics.length === 0) {
			return;
		}

		const lines: string[] = [];
		const date = new Date();

		lines.push(`[Diagnostics - ${date.toTimeString()}] ${path.join()}`);
		for (const diag of diagnostics) {
			lines.push(
				`  (${formatCategoryDescription(diag.description)}) ${readMarkup(
					diag.description.message,
				)}`,
			);
		}
		this.logMessage(path, lines.join("\n"));
	}

	private createFakeServerRequest(
		commandName: CommandName,
		args: string[] = [],
	): ServerRequest {
		return new ServerRequest({
			client: this.client,
			server: this.server,
			query: {
				requestFlags: DEFAULT_CLIENT_REQUEST_FLAGS,
				commandFlags: {},
				args,
				commandName,
				silent: true,
				noData: false,
				noFileWrites: false,
			},
		});
	}

	public unwatchProject(path: AbsoluteFilePath) {
		// TODO maybe unset all buffers?
		const session = this.projectSessions.get(path);
		if (session !== undefined) {
			this.server.fatalErrorHandler.wrapPromise(
				session.request.teardown(EMPTY_SUCCESS_RESPONSE),
			);
			this.projectSessions.delete(path);
		}
		this.logMessage(path, `Unwatched: ${path.join()}`);
	}

	public createProgress(opts?: ReporterProgressOptions): ReporterProgress {
		return new LSPProgress(this.transport, this.request.reporter, opts);
	}

	public async initProject(path: AbsoluteFilePath) {
		if (this.projectSessions.has(path) || this.lintSessionsPending.has(path)) {
			return;
		}

		this.lintSessionsPending.add(path);

		const project = await this.server.projectManager.findProject(path);

		if (project === undefined) {
			// Not a Rome project
			this.lintSessionsPending.delete(path);
			return;
		}

		const req = this.createFakeServerRequest("noop", [path.join()]);
		await req.init();

		this.projectSessions.set(
			path,
			{
				request: req,
				status: "PENDING",
			},
		);
	}

	public async watchPendingProjects() {
		const promises = [];
		for (const [path, session] of this.projectSessions) {
			if (session.status === "PENDING") {
				promises.push(this.watchProject(path, session.request));
			}
		}
		await Promise.all(promises);
	}

	private async watchProject(path: AbsoluteFilePath, req: ServerRequest) {
		const checker = new Checker(
			req,
			{
				apply: false,
				hasDecisions: false,
				formatOnly: false,
				suppressionExplanation: "suppressed via editor",
			},
		);

		const runner = await checker.createRunner({
			onRunStart: () => {},
			createProgress: () => {
				return this.createProgress();
			},
			onChange: ({path, diagnostics}) => {
				const absolutePath = this.server.projectManager.getFilePathFromUIDOrAbsolute(
					path,
				);
				if (!absolutePath) {
					// Can only display absolute path diagnostics
					return;
				}

				this.diagnosticsProcessor.removePath(absolutePath);
				this.diagnosticsProcessor.addDiagnostics(diagnostics);

				this.transport.write({
					method: "textDocument/publishDiagnostics",
					params: {
						uri: `file://${absolutePath.join()}`,
						diagnostics: convertDiagnosticsToLSP(
							this.diagnosticsProcessor.getDiagnosticsForPath(absolutePath),
							this.server,
						),
					},
				});
			},
			onRunEnd: ({}) => {},
		});

		req.resources.add(await checker.watch(runner));

		this.projectSessions.set(
			path,
			{
				request: req,
				status: "WATCHING",
			},
		);
		this.lintSessionsPending.delete(path);

		const date = new Date();
		this.logMessage(path, `Watching ${path.join()} at ${date.toTimeString()}`);
		this.watchProjectEvent.send(path);
	}

	public async shutdown() {
		// Unwatch projects
		for (const path of this.projectSessions.keys()) {
			this.unwatchProject(path);
		}
		this.projectSessions.clear();

		// Clear set buffers
		for (const path of this.fileBuffers) {
			await this.request.requestWorkerClearBuffer(path);
		}
		this.fileBuffers.clear();
	}

	public async sendClientRequest(
		req: PartialServerQueryRequest,
	): Promise<ServerQueryResponse> {
		return this.client.handleRequest({
			silent: true,
			...req,
		});
	}
}
