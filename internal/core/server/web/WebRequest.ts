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
import {Reporter, ReporterCaptureStream} from "@internal/cli-reporter";
import {createBridgeFromWebSocketInterface} from "@internal/events";
import {createUnknownPath} from "@internal/path";
import {WebServer} from "./index";
import {ProjectDefinition} from "@internal/project";
import {ConsumableUrl, consumeUrl} from "@internal/codec-url";
import http = require("http");
import {markup} from "@internal/markup";

const waitForever = new Promise(() => {});

export function stripBundleSuffix(pathname: string): string {
	return removePrefix(removeSuffix(pathname, ".bundle"), "/");
}

export default class WebRequest {
	constructor(
		{req, res, server, serverRequest, webServer}: {
			webServer: WebServer;
			server: Server;
			serverRequest: ServerRequest;
			req: http.IncomingMessage;
			res: http.ServerResponse;
		},
	) {
		this.req = req;
		this.res = res;
		this.webServer = webServer;
		this.reporter = new Reporter();
		this.reporterStream = this.reporter.attachCaptureStream("markup");
		this.server = server;
		this.serverRequest = serverRequest;

		const reqUrl = req.url;
		if (reqUrl === undefined) {
			throw new Error("req.url should not be undefined");
		}
		this.url = consumeUrl(reqUrl);
	}

	private reporter: Reporter;
	private reporterStream: ReporterCaptureStream;
	private webServer: WebServer;
	private server: Server;
	private serverRequest: ServerRequest;

	private url: ConsumableUrl;

	private req: http.IncomingMessage;
	private res: http.ServerResponse;

	private loadRawBody(): Promise<string> {
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

	public async dispatch(): Promise<void> {
		const {req, res} = this;

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
		} finally {
			// Format status code
			// https://en.wikipedia.org/wiki/List_of_HTTP_status_codes
			const statusCode = res.statusCode;
			let status = markup`${statusCode}`;
			if (statusCode >= 100 && statusCode <= 199) {
				// 1xx informational response – the request was received, continuing process
				status = markup`<info>${status}</info>`;
			} else if (statusCode >= 200 && statusCode <= 299) {
				// 2xx successful – the request was successfully received, understood, and accepted
				status = markup`<success>${status}</success>`;
			} else if (statusCode >= 300 && statusCode <= 399) {
				// 3xx redirection – further action needs to be taken in order to complete the request
				status = markup`<info>${status}</info>`;
			} else if (statusCode >= 400 && statusCode <= 499) {
				// 4xx client error – the request contains bad syntax or cannot be fulfilled
				status = markup`<error>${status}</error>`;
			} else if (statusCode >= 500 && statusCode <= 599) {
				// 5xx server error – the server failed to fulfil an apparently valid request
				status = markup`<error>${status}</error>`;
			}

			// Log <METHOD> <URL> <STATUS>
			this.webServer.reporter.log(
				this.reporterStream.readAsMarkup(),
				{noNewline: true},
			);
			this.reporter.log(markup`<dim>${req.method}</dim> ${req.url} ${status}`);
		}
	}

	private async dispatchWithBody(body: string): Promise<void> {
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

			default:
				return this.handleWildcard(pathname);
		}
	}

	private async handleWildcard(pathname: string) {
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

	private async handlePossibleStatic(
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

	private async handleFrontendScript() {
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
			source: createUnknownPath("@internal/web-ui"),
		});
		const bundle = await bundler.bundle(resolved);
		res.end(bundle.entry.js.content);
	}

	private negotiateWebsocket() {
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

	private async handleFrontendWebsocket() {
		const {req} = this;
		this.negotiateWebsocket();

		const socket = new WebSocketInterface({type: "server", socket: req.socket});
		const bridge = createBridgeFromWebSocketInterface(
			WebBridge,
			socket,
			{
				type: "client",
			},
		);
		this.webServer.onWebsocketBridge(req, bridge);

		await bridge.handshake();

		this.reporter.success(markup`websocket client connected`);

		this.webServer.sendRequests(bridge);

		await waitForever;
	}

	private async handleBundleRequest() {
		const {res} = this;

		const {bundler, path} = await this.webServer.getBundler(this.url);
		const bundle = await bundler.bundle(path);
		const content = bundle.entry.js.content;

		res.writeHead(200, {"Content-Type": "application/javascript"});
		res.end(content);
		return true;
	}
}
