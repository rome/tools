/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {Server, ServerRequest, WebBridge} from "@internal/core";
import {
	deriveDiagnosticFromError,
	getDiagnosticsFromError,
} from "@internal/diagnostics";
import {dedent, removePrefix, removeSuffix} from "@internal/string-utils";
import Bundler from "../bundler/Bundler";
import {WebSocketInterface, createKey} from "@internal/codec-websocket";
import {Reporter} from "@internal/cli-reporter";
import {createBridgeFromWebSocketInterface} from "@internal/events";
import {createUnknownFilePath} from "@internal/path";
import {WebServer} from "./index";
import {ProjectDefinition} from "@internal/project";
import {HmrClientMessage} from "./hmr";
import {ConsumableUrl, consumeUrl} from "@internal/codec-url";
import http = require("http");
import {markup} from "@internal/markup";

const waitForever = new Promise(() => {});

export function stripBundleSuffix(pathname: string): string {
	return removePrefix(removeSuffix(pathname, ".bundle"), "/");
}

export default class WebRequest {
	constructor(
		webServer: WebServer,
		req: http.IncomingMessage,
		res: http.ServerResponse,
	) {
		this.req = req;
		this.res = res;
		this.webServer = webServer;
		this.reporter = webServer.reporter;
		this.server = webServer.server;
		this.serverRequest = webServer.serverRequest;

		const reqUrl = req.url;
		if (reqUrl === undefined) {
			throw new Error("req.url should not be undefined");
		}
		this.url = consumeUrl(reqUrl);
	}

	reporter: Reporter;
	webServer: WebServer;
	server: Server;
	serverRequest: ServerRequest;

	url: ConsumableUrl;

	req: http.IncomingMessage;
	res: http.ServerResponse;

	loadRawBody(): Promise<string> {
		const {req} = this;

		req.setEncoding("utf8");
		let rawBody = "";

		return new Promise((resolve) => {
			req.on(
				"data",
				(chunk) => {
					rawBody += chunk;
				},
			);

			req.on(
				"end",
				() => {
					resolve(rawBody);
				},
			);
		});
	}

	async dispatch(): Promise<void> {
		const {res} = this;

		try {
			const rawBody = await this.loadRawBody();
			await this.dispatchWithBody(rawBody);
			res.end();
		} catch (err) {
			res.writeHead(500, {"Content-Type": "text/plain"});

			let diagnostics = getDiagnosticsFromError(err);
			if (diagnostics === undefined) {
				diagnostics = [
					deriveDiagnosticFromError(
						err,
						{
							description: {
								category: "internalError/httpServer",
							},
						},
					),
				];
			}

			//this.request.reporter.clear();
			try {
				const printer = this.serverRequest.createDiagnosticsPrinter(
					this.server.createDiagnosticsProcessor({
						origins: [
							{
								category: "WebRequest",
							},
						],
					}),
				);
				printer.processor.addDiagnostics(diagnostics);
				await printer.print();
			} catch (err) {
				this.reporter.warn(markup`Failed trying to print diagnostics`);
				this.reporter.error(err.stack);
			}

			res.end("Diagnostics available, see console");
		}
	}

	async dispatchWithBody(body: string): Promise<void> {
		const {res} = this;
		const pathname = this.url.path.asString();
		body;

		switch (pathname) {
			case "/favicon.ico": {
				res.end("");
				break;
			}

			case "/__rome__/websocket":
				return this.handleFrontendWebsocket();

			case "/__rome__/script.js":
				return this.handleFrontendScript();

			case "/__rome__": {
				res.writeHead(200, {"Content-Type": "text/html"});
				res.end(
					dedent`
            <!doctype html>
            <html>
              <head>
                <meta charset="utf-8">
                <title>Rome</title>
                <link rel="stylesheet" href="https://meyerweb.com/eric/tools/css/reset/reset.css">
              </head>
              <body>
                <div id="app"></div>
                <script src="/__rome__/script.js"></script>
              </body>
            </html>
          `,
				);
				break;
			}

			case "/hot":
				return this.handleDeviceWebsocket();

			default:
				return this.handleWildcard(pathname);
		}
	}

	async handleWildcard(pathname: string) {
		const {req, res} = this;

		// Check for *.bundle
		if (pathname.endsWith(".bundle")) {
			const handled = await this.handleBundleRequest();
			if (handled) {
				return;
			}
		}

		// Look up static file
		const project = await this.serverRequest.assertClientCwdProject();
		if (project.config.develop.serveStatic) {
			const handled = await this.handlePossibleStatic(pathname, project);
			if (handled) {
				return;
			}
		}

		this.reporter.error(markup`Unknown request for ${String(req.url)}`);
		res.writeHead(404);
		res.end("Not found");
	}

	async handlePossibleStatic(
		pathname: string,
		project: ProjectDefinition,
	): Promise<boolean> {
		project;

		const possibleStaticPath = await this.webServer.pathnameToAbsolutePath(
			pathname,
		);

		// TODO check if it is a file
		if (
			possibleStaticPath !== undefined &&
			(await this.server.memoryFs.existsHard(possibleStaticPath))
		) {
			return true;
		}

		return false;
	}

	async handleFrontendScript() {
		const {res} = this;
		res.writeHead(200, {"Content-Type": "application/javascript"});

		const bundler = new Bundler(
			this.serverRequest,
			{
				inlineSourceMap: false,
				cwd: this.serverRequest.client.flags.cwd,
				resolver: {
					platform: "web",
				},
			},
		);
		const resolved = await this.server.resolver.resolveEntryAssertPath({
			origin: this.serverRequest.client.flags.cwd,
			source: createUnknownFilePath("@internal/web-ui"),
		});
		const bundle = await bundler.bundle(resolved);
		res.end(bundle.entry.js);
	}

	negotiateWebsocket() {
		const {req} = this;

		const digest = createKey(String(req.headers["sec-websocket-key"]));

		const headers = [
			"HTTP/1.1 101 Switching Protocols",
			"Upgrade: websocket",
			"Connection: Upgrade",
			"Sec-WebSocket-Protocol: rome",
			`Sec-WebSocket-Accept: ${digest}`,
			"",
			"",
		];

		req.socket.write(headers.join("\r\n"));
	}

	async handleDeviceWebsocketMessage(
		socket: WebSocketInterface,
		data: HmrClientMessage,
	) {
		switch (data.type) {
			case "log":
				return this.webServer.printConsoleLog(data);

			case "log-opt-in":
				// ???
				return;

			case "register-entrypoints":
				/// ???
				return;

			default:
				console.log("UNKNOWN MESSAGE", data);
		}
	}

	async handleDeviceWebsocket() {
		const {req} = this;
		this.negotiateWebsocket();

		const socket = new WebSocketInterface("server", req.socket);
		this.webServer.deviceWebsockets.add(socket);

		req.socket.on(
			"error",
			(err) => {
				console.log(err.stack);
			},
		);

		this.reporter.success(markup`Device websocket client connected`);

		socket.completeFrameEvent.subscribe((frame) => {
			const text = frame.payload.toString();
			try {
				const json = JSON.parse(text);
				this.handleDeviceWebsocketMessage(socket, json);
			} catch (err) {
				if (err instanceof SyntaxError) {
					console.log("UNKNOWN FRAME", text);
					return;
				} else {
					throw err;
				}
			}
		});

		socket.errorEvent.subscribe((err) => {
			console.log(err);
		});

		socket.endEvent.subscribe(() => {
			console.log("END");
			this.webServer.deviceWebsockets.delete(socket);
		});

		await waitForever;
	}

	async handleFrontendWebsocket() {
		const {req} = this;
		this.negotiateWebsocket();

		const socket = new WebSocketInterface("server", req.socket);
		const bridge = createBridgeFromWebSocketInterface(
			WebBridge,
			socket,
			{
				type: "client",
			},
		);
		this.webServer.frontendWebsocketBridges.add(bridge);

		req.socket.on(
			"close",
			() => {
				this.webServer.frontendWebsocketBridges.delete(bridge);
			},
		);

		await bridge.handshake();

		this.reporter.success(markup`Frontend websocket client connected`);

		this.webServer.sendRequests(bridge);

		await waitForever;
	}

	async handleBundleRequest() {
		const {res} = this;

		const {bundler, path} = await this.webServer.getBundler(this.url);
		const bundle = await bundler.bundle(path);
		const content = bundle.entry.js.content;

		res.writeHead(200, {"Content-Type": "application/javascript"});
		res.end(content);
		return true;
	}
}
