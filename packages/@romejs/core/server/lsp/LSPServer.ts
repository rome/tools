/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {Consumer, consumeUnknown} from "@romejs/consume";
import {
	LSPDiagnostic,
	LSPDiagnosticRelatedInformation,
	LSPPosition,
	LSPRange,
	LSPResponseMessage,
	LSPTextEdit,
} from "./types";
import Server, {ServerClient} from "../Server";
import {
	AbsoluteFilePath,
	AbsoluteFilePathMap,
	AbsoluteFilePathSet,
	createAbsoluteFilePath,
} from "@romejs/path";
import {
	DiagnosticLocation,
	Diagnostics,
	catchDiagnostics,
} from "@romejs/diagnostics";
import {Position} from "@romejs/parser-core";
import {Number0, ob1Coerce1To0, ob1Inc, ob1Number0} from "@romejs/ob1";
import {markupToPlainTextString} from "@romejs/string-markup";
import {
	PartialServerQueryRequest,
	ServerQueryResponse,
} from "@romejs/core/common/bridges/ServerBridge";
import Linter from "../linter/Linter";
import ServerRequest, {EMPTY_SUCCESS_RESPONSE} from "../ServerRequest";
import {DEFAULT_CLIENT_REQUEST_FLAGS} from "@romejs/core/common/types/client";
import stringDiff, {Diffs, diffConstants} from "@romejs/string-diff";
import {JSONObject, JSONPropertyValue} from "@romejs/codec-json";
import {
	Reporter,
	ReporterProgress,
	ReporterProgressBase,
	ReporterProgressOptions,
} from "@romejs/cli-reporter";

type Status = "IDLE" | "WAITING_FOR_HEADERS_END" | "WAITING_FOR_RESPONSE_END";

type Headers = {
	length: number;
	extra: Map<string, string>;
};

const HEADERS_END = "\r\n\r\n";

function parseHeaders(buffer: string): Headers {
	const headers: Map<string, string> = new Map();

	for (const line of buffer.split("\n")) {
		const clean = line.trim();
		const match = clean.match(/^(.*?): (.*?)$/);
		if (match == null) {
			throw new Error(`Invalid header: ${clean}`);
		}

		const [, key, value] = match;
		headers.set(key.toLowerCase(), value);
	}

	const length = headers.get("content-length");
	if (length === undefined) {
		throw new Error("Expected Content-Length");
	}
	headers.delete("content-length");

	return {
		length: Number(length),
		extra: headers,
	};
}

function convertPositionToLSP(pos: undefined | Position): LSPPosition {
	if (pos === undefined) {
		return {
			line: ob1Number0,
			character: ob1Number0,
		};
	} else {
		return {
			line: ob1Coerce1To0(pos.line),
			character: pos.column,
		};
	}
}

function convertDiagnosticLocationToLSPRange(
	location: DiagnosticLocation,
): LSPRange {
	return {
		start: convertPositionToLSP(location.start),
		end: convertPositionToLSP(location.end),
	};
}

function convertDiagnosticsToLSP(
	diagnostics: Diagnostics,
	server: Server,
): Array<LSPDiagnostic> {
	const lspDiagnostics: Array<LSPDiagnostic> = [];

	for (const {description, location} of diagnostics) {
		// Infer relatedInformation from log messages followed by frames
		let relatedInformation: Array<LSPDiagnosticRelatedInformation> = [];
		const {advice} = description;
		for (let i = 0; i < advice.length; i++) {
			const item = advice[i];
			const nextItem = advice[i + 1];
			if (
				item.type === "log" &&
				nextItem !== undefined &&
				nextItem.type === "frame"
			) {
				const abs = server.projectManager.getFilePathFromUidOrAbsolute(
					nextItem.location.filename,
				);
				if (abs !== undefined) {
					relatedInformation.push({
						message: markupToPlainTextString(item.text),
						location: {
							uri: `file://${abs.join()}`,
							range: convertDiagnosticLocationToLSPRange(nextItem.location),
						},
					});
				}
			}
		}

		lspDiagnostics.push({
			severity: 1,
			range: convertDiagnosticLocationToLSPRange(location),
			message: markupToPlainTextString(description.message.value),
			code: description.category,
			source: "rome",
			relatedInformation,
		});
	}

	return lspDiagnostics;
}

function getPathFromTextDocument(consumer: Consumer): AbsoluteFilePath {
	return createAbsoluteFilePath(consumer.get("uri").asString());
}

function diffTextEdits(original: string, desired: string): Array<LSPTextEdit> {
	const edits: Array<LSPTextEdit> = [];

	const diffs: Diffs = stringDiff(original, desired);

	let currLine: Number0 = ob1Number0;
	let currChar: Number0 = ob1Number0;

	function advance(str: string) {
		for (const char of str) {
			if (char === "\n") {
				currLine = ob1Inc(currLine);
				currChar = ob1Number0;
			} else {
				currChar = ob1Inc(currChar);
			}
		}
	}

	function getPosition(): LSPPosition {
		return {
			line: currLine,
			character: currChar,
		};
	}

	for (const [type, text] of diffs) {
		switch (type) {
			case diffConstants.ADD: {
				const pos = getPosition();
				edits.push({
					range: {
						start: pos,
						end: pos,
					},
					newText: text,
				});
				break;
			}

			case diffConstants.DELETE: {
				const start: LSPPosition = getPosition();
				advance(text);
				const end: LSPPosition = getPosition();
				edits.push({
					range: {
						start,
						end,
					},
					newText: "",
				});
				break;
			}

			case diffConstants.EQUAL: {
				advance(text);
				break;
			}
		}
	}

	return edits;
}

let progressTokenCounter = 0;

class LSPProgress extends ReporterProgressBase {
	constructor(
		server: LSPServer,
		reporter: Reporter,
		opts?: ReporterProgressOptions,
	) {
		super(reporter, opts);
		this.server = server;
		this.token = progressTokenCounter++;
		this.lastRenderKey = "";

		server.write({
			type: "$/progress",
			params: {
				token: this.token,
				value: {
					kind: "begin",
					cancellable: false,
					title: this.title,
					percentage: 0,
				},
			},
		});
	}

	lastRenderKey: string;
	token: number;
	server: LSPServer;

	render() {
		const total = this.total === undefined ? 0 : this.total;
		const percentage = Math.floor(100 / total * this.current);

		// Make sure we don't send pointless duplicate messages
		const renderKey = `percent:${percentage},text:${this.text}`;
		if (this.lastRenderKey === renderKey) {
			return;
		}

		this.lastRenderKey = renderKey;
		this.server.write({
			type: "$/progress",
			params: {
				token: this.token,
				value: {
					kind: "report",
					cancellable: false,
					message: this.text,
					percentage,
				},
			},
		});
	}

	end() {
		this.server.write({
			type: "$/progress",
			params: {
				token: this.token,
				value: {
					kind: "end",
				},
			},
		});
	}
}

export default class LSPServer {
	constructor(request: ServerRequest) {
		this.status = "IDLE";
		this.socketBuffer = "";
		this.nextHeaders = undefined;

		this.request = request;
		this.server = request.server;
		this.client = request.client;

		this.lintSessionsPending = new AbsoluteFilePathSet();
		this.lintSessions = new AbsoluteFilePathMap();

		this.fileBuffers = new AbsoluteFilePathSet();

		request.endEvent.subscribe(async () => {
			await this.shutdown();
		});
	}

	request: ServerRequest;
	client: ServerClient;
	server: Server;
	nextHeaders: undefined | Headers;
	status: Status;
	socketBuffer: string;

	fileBuffers: AbsoluteFilePathSet;
	lintSessionsPending: AbsoluteFilePathSet;
	lintSessions: AbsoluteFilePathMap<ServerRequest>;

	write(res: JSONObject) {
		const json = JSON.stringify(res);
		const out = `Content-Length: ${String(json.length)}${HEADERS_END}${json}`;
		this.client.bridge.lspFromServerBuffer.send(out);
	}

	logMessage(path: AbsoluteFilePath, message: string) {
		this.write({
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
		return new LSPProgress(this, this.request.reporter, opts);
	}

	async watchProject(path: AbsoluteFilePath) {
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

		const linter = new Linter(
			req,
			{
				save: false,
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
				for (const {ref, diagnostics} of changes) {
					if (ref === undefined) {
						// Cannot display diagnostics without a reference
						continue;
					}

					// We want to filter pendingFixes because we'll autoformat the file on save if necessary and it's just noise
					const processor = this.request.createDiagnosticsProcessor();
					processor.addFilter({
						category: "lint/pendingFixes",
					});
					processor.addDiagnostics(diagnostics);

					this.write({
						method: "textDocument/publishDiagnostics",
						params: {
							uri: `file://${ref.real.join()}`,
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
					this.watchProject(createAbsoluteFilePath(rootUri.asString()));
				}

				const workspaceFolders = params.get("workspaceFolders");
				if (workspaceFolders.exists()) {
					for (const elem of workspaceFolders.asArray()) {
						this.watchProject(getPathFromTextDocument(elem));
					}
				}

				return {
					capabilities: {
						textDocumentSync: {
							openClose: true,
							// This sends over the full text on change. We should make this incremental later
							change: 1,
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
					this.watchProject(getPathFromTextDocument(elem));
				}
				for (const elem of params.get("removed").asArray()) {
					this.unwatchProject(getPathFromTextDocument(elem));
				}
				break;
			}

			case "textDocument/didChange": {
				const path = getPathFromTextDocument(params.get("textDocument"));
				const content = params.get("contentChanges").asArray()[0].get("text").asString();
				await this.request.requestWorkerUpdateBuffer(path, content);
				this.fileBuffers.add(path);
				break;
			}

			case "textDocument/didSave": {
				const path = getPathFromTextDocument(params.get("textDocument"));
				this.fileBuffers.delete(path);
				await this.request.requestWorkerClearBuffer(path);
				break;
			}
		}
	}

	normalizeMessage(content: string): undefined | Consumer {
		try {
			const data = JSON.parse(content);
			const consumer = consumeUnknown(data, "lsp/parse");
			return consumer;
		} catch (err) {
			if (err instanceof SyntaxError) {
				console.error("JSON parse error", content);
				return undefined;
			} else {
				throw err;
			}
		}
	}

	async onMessage(headers: Headers, content: string) {
		const consumer = this.normalizeMessage(content);
		if (consumer === undefined) {
			return;
		}

		if (!consumer.has("method")) {
			console.error("NO METHOD", content);
			return;
		}

		const method: string = consumer.get("method").asString();
		const params = consumer.get("params");

		if (consumer.has("id")) {
			const id = consumer.get("id").asNumber();

			try {
				const res: LSPResponseMessage = {
					id,
					result: await this.handleRequest(method, params),
				};
				this.write(res);
			} catch (err) {
				const res: LSPResponseMessage = {
					id,
					error: {
						code: -32_603,
						message: err.message,
					},
				};
				this.write(res);
			}
		} else {
			await this.handleNotification(method, params);
		}
	}

	process() {
		switch (this.status) {
			case "IDLE": {
				if (this.socketBuffer.length > 0) {
					this.status = "WAITING_FOR_HEADERS_END";
					this.process();
				}
				break;
			}

			case "WAITING_FOR_HEADERS_END": {
				const endIndex = this.socketBuffer.indexOf(HEADERS_END);
				if (endIndex !== -1) {
					// Parse headers
					const rawHeaders = this.socketBuffer.slice(0, endIndex);
					this.nextHeaders = parseHeaders(rawHeaders);

					// Process rest of the buffer
					this.status = "WAITING_FOR_RESPONSE_END";
					this.socketBuffer = this.socketBuffer.slice(
						endIndex + HEADERS_END.length,
					);
					this.process();
				}
				break;
			}

			case "WAITING_FOR_RESPONSE_END": {
				const headers = this.nextHeaders;
				if (headers === undefined) {
					throw new Error("Expected headers due to our status");
				}
				if (this.socketBuffer.length >= headers.length) {
					const content = this.socketBuffer.slice(0, headers.length);
					this.onMessage(headers, content);

					// Reset headers and trim content
					this.nextHeaders = undefined;
					this.socketBuffer = this.socketBuffer.slice(headers.length);

					// Process rest of the buffer
					this.status = "IDLE";
					this.process();
				}
				break;
			}
		}
	}

	append(data: string) {
		this.socketBuffer += data;
		this.process();
	}
}
