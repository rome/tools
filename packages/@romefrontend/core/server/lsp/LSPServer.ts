/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {Consumer} from "@romefrontend/consume";
import Server, {ServerClient} from "../Server";
import {
	AbsoluteFilePath,
	AbsoluteFilePathMap,
	AbsoluteFilePathSet,
	createAbsoluteFilePath,
} from "@romefrontend/path";
import {Diagnostics, catchDiagnostics} from "@romefrontend/diagnostics";
import {
	PartialServerQueryRequest,
	ServerQueryResponse,
} from "@romefrontend/core/common/bridges/ServerBridge";
import Linter from "../linter/Linter";
import ServerRequest, {EMPTY_SUCCESS_RESPONSE} from "../ServerRequest";
import {DEFAULT_CLIENT_REQUEST_FLAGS} from "@romefrontend/core/common/types/client";
import {JSONPropertyValue} from "@romefrontend/codec-json";
import {
	ReporterProgress,
	ReporterProgressOptions,
} from "@romefrontend/cli-reporter";
import {LSPTransport} from "./protocol";
import LSPProgress from "./LSPProgress";
import {
	convertDiagnosticsToLSP,
	diffTextEdits,
	getPathFromTextDocument,
	getWorkerBufferPatches,
} from "./utils";

export default class LSPServer {
	constructor(request: ServerRequest) {
		this.request = request;
		this.server = request.server;
		this.client = request.client;

		this.lintSessionsPending = new AbsoluteFilePathSet();
		this.lintSessions = new AbsoluteFilePathMap();

		this.fileBuffers = new AbsoluteFilePathSet();

		request.endEvent.subscribe(async () => {
			await this.shutdown();
		});

		const transport = new LSPTransport(this.server.logger);
		this.transport = transport;

		transport.notificationEvent.subscribe(({method, params}) => {
			return this.handleNotification(method, params);
		});

		transport.requestEvent.subscribe(({method, params}) => {
			return this.handleRequest(method, params);
		});
	}

	request: ServerRequest;
	client: ServerClient;
	server: Server;
	transport: LSPTransport;

	fileBuffers: AbsoluteFilePathSet;
	lintSessionsPending: AbsoluteFilePathSet;
	lintSessions: AbsoluteFilePathMap<ServerRequest>;

	logMessage(path: AbsoluteFilePath, message: string) {
		this.transport.write({
			method: "window/logMessage",
			params: {
				uri: `file://${path.join()}`,
				message,
			},
		});
	}

	logDiagnostics(path: AbsoluteFilePath, diagnostics: Diagnostics = []) {
		if (diagnostics.length === 0) {
			return;
		}

		const lines: Array<string> = [];
		const date = new Date();

		lines.push(`[Diagnostics - ${date.toTimeString()}] ${path.join()}`);
		for (const diag of diagnostics) {
			lines.push(
				`  (${diag.description.category}) ${diag.description.message.value}`,
			);
		}
		this.logMessage(path, lines.join("\n"));
	}

	createFakeServerRequest(
		commandName: string,
		args: Array<string> = [],
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
				terminateWhenIdle: false,
			},
		});
	}

	unwatchProject(path: AbsoluteFilePath) {
		// TODO maybe unset all buffers?
		const req = this.lintSessions.get(path);
		if (req !== undefined) {
			req.teardown(EMPTY_SUCCESS_RESPONSE);
			this.lintSessions.delete(path);
		}
	}

	createProgress(opts?: ReporterProgressOptions): ReporterProgress {
		return new LSPProgress(this.transport, this.request.reporter, opts);
	}

	async initProject(path: AbsoluteFilePath) {
		if (this.lintSessions.has(path) || this.lintSessionsPending.has(path)) {
			return;
		}

		this.lintSessionsPending.add(path);

		const project = await this.server.projectManager.findProject(path);

		if (project === undefined) {
			// Not a Rome project
			this.lintSessionsPending.delete(path);
			return;
		}

		const req = this.createFakeServerRequest("lsp_project", [path.join()]);
		await req.init();

		// This is not awaited so it doesn't delay the initialize response
		this.watchProject(path, req);
	}

	async watchProject(path: AbsoluteFilePath, req: ServerRequest) {
		const linter = new Linter(
			req,
			{
				apply: false,
				hasDecisions: false,
				formatOnly: false,
			},
		);

		const subscription = await linter.watch({
			onRunStart: () => {},
			createProgress: () => {
				return this.createProgress();
			},
			onChanges: ({changes}) => {
				for (const {type, filename, diagnostics} of changes) {
					if (filename === undefined || type !== "absolute") {
						// Can only display absolute path diagnostics
						continue;
					}

					// We want to filter pendingFixes because we'll autoformat the file on save if necessary and it's just noise
					const processor = this.request.createDiagnosticsProcessor();
					processor.addFilter({
						category: "lint/pendingFixes",
					});
					processor.addDiagnostics(diagnostics);

					this.transport.write({
						method: "textDocument/publishDiagnostics",
						params: {
							uri: `file://${filename}`,
							diagnostics: convertDiagnosticsToLSP(
								processor.getDiagnostics(),
								this.server,
							),
						},
					});
				}
			},
		});

		req.endEvent.subscribe(() => {
			subscription.unsubscribe();
		});

		this.lintSessions.set(path, req);
		this.lintSessionsPending.delete(path);

		const date = new Date();
		this.logMessage(path, `Watching ${path.join()} at ${date.toTimeString()}`);
	}

	async shutdown() {
		// Unwatch projects
		for (const path of this.lintSessions.keys()) {
			this.unwatchProject(path);
		}
		this.lintSessions.clear();

		// Clear set buffers
		for (const path of this.fileBuffers) {
			await this.request.requestWorkerClearBuffer(path);
		}
		this.fileBuffers.clear();
	}

	async sendClientRequest(
		req: PartialServerQueryRequest,
	): Promise<ServerQueryResponse> {
		return this.server.handleRequest(
			this.client,
			{
				silent: true,
				...req,
			},
		);
	}

	async handleRequest(
		method: string,
		params: Consumer,
	): Promise<JSONPropertyValue> {
		switch (method) {
			case "initialize": {
				const rootUri = params.get("rootUri");
				if (rootUri.exists()) {
					await this.initProject(createAbsoluteFilePath(rootUri.asString()));
				}

				const workspaceFolders = params.get("workspaceFolders");
				if (workspaceFolders.exists()) {
					for (const elem of workspaceFolders.asArray()) {
						await this.initProject(getPathFromTextDocument(elem));
					}
				}

				return {
					capabilities: {
						textDocumentSync: {
							openClose: true,
							// This sends over incremental patches on change
							change: 2,
						},
						documentFormattingProvider: true,
						workspaceFolders: {
							supported: true,
							changeNotifications: true,
						},
					},
					serverInfo: {
						name: "rome",
					},
				};
			}

			case "textDocument/formatting": {
				const path = getPathFromTextDocument(params.get("textDocument"));

				const project = this.server.projectManager.findProjectExisting(path);
				if (project === undefined) {
					// Not in a Rome project
					return null;
				}

				const {value, diagnostics} = await catchDiagnostics(async () => {
					return this.request.requestWorkerFormat(path, {});
				});

				this.logDiagnostics(path, diagnostics);

				if (value === undefined) {
					// Not a file we support formatting or has diagnostics
					return null;
				}

				return diffTextEdits(value.original, value.formatted);
			}

			case "shutdown": {
				this.shutdown();
				break;
			}
		}

		return null;
	}

	async handleNotification(method: string, params: Consumer): Promise<void> {
		switch (method) {
			case "workspace/didChangeWorkspaceFolders": {
				for (const elem of params.get("added").asArray()) {
					await this.initProject(getPathFromTextDocument(elem));
				}
				for (const elem of params.get("removed").asArray()) {
					this.unwatchProject(getPathFromTextDocument(elem));
				}
				break;
			}

			case "textDocument/didOpen": {
				const path = getPathFromTextDocument(params.get("textDocument"));
				const content = params.get("textDocument").get("text").asString();
				await this.request.requestWorkerUpdateBuffer(path, content);
				this.fileBuffers.add(path);
				this.logMessage(path, `Opened: ${path.join()}`);
				break;
			}

			case "textDocument/didChange": {
				const path = getPathFromTextDocument(params.get("textDocument"));
				const contentChanges = params.get("contentChanges");

				if (contentChanges.asArray()[0].has("range")) {
					const patches = getWorkerBufferPatches(contentChanges);
					await this.request.requestWorkerPatchBuffer(path, patches);
				} else {
					const content = contentChanges.asArray()[0].get("text").asString();
					await this.request.requestWorkerUpdateBuffer(path, content);
				}
				break;
			}

			case "textDocument/didClose": {
				const path = getPathFromTextDocument(params.get("textDocument"));
				this.fileBuffers.delete(path);
				await this.request.requestWorkerClearBuffer(path);
				this.logMessage(path, `Closed: ${path.join()}`);
				break;
			}
		}
	}
}
