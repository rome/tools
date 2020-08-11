/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {
	BundlerConfig,
	ClientFlags,
	PLATFORMS,
	Server,
	ServerRequest,
	WebBridge,
} from "@internal/core";
import Bundler from "../bundler/Bundler";
import http = require("http");
import {Reporter} from "@internal/cli-reporter";
import {
	ServerQueryRequest,
	ServerQueryResponse,
} from "../../common/bridges/ServerBridge";
import {ServerMarker} from "../Server";

import WebRequest, {stripBundleSuffix} from "./WebRequest";

import {AbsoluteFilePath} from "@internal/path";

import {ConsumableUrl} from "@internal/codec-url";
import {DEFAULT_TERMINAL_FEATURES} from "@internal/cli-environment";
import {markup} from "@internal/markup";

export type WebServerTime = {
	startTime: number;
	endTime: undefined | number;
};

export type WebServerClient = WebServerTime & {
	id: number;
	flags: ClientFlags;
	stdoutAnsi: string;
	stdoutHTML: string;
};

export type WebServerRequest = WebServerTime & {
	id: number;
	client: number;
	query: ServerQueryRequest;
	markers: Array<ServerMarker>;
	response: undefined | ServerQueryResponse;
};

export class WebServer {
	constructor(req: ServerRequest) {
		const {server} = req;

		this.serverRequest = req;
		this.reporter = req.reporter;
		this.server = server;

		this.bundlerCache = new Map();

		this.savingRequests = false;
		this.clientRequestHistory = new Map();
		this.clientHistory = new Map();

		this.websocketBridges = new Set();

		this.httpServer = http.createServer((req, res) => {
			const webRequest = new WebRequest({
				req,
				res,
				server: this.server,
				serverRequest: this.serverRequest,
				webServer: this,
			});
			server.wrapFatalPromise(webRequest.dispatch());
		});

		server.clientStartEvent.subscribe((client) => {
			if (!this.savingRequests) {
				return;
			}

			const data: WebServerClient = {
				id: client.id,
				flags: client.flags,
				startTime: Date.now(),
				endTime: undefined,
				stdoutAnsi: "",
				stdoutHTML: "",
			};
			this.clientHistory.set(client.id, data);
			this.refreshRequests();

			const ansiReporterStream = client.reporter.addStream({
				format: "ansi",
				features: DEFAULT_TERMINAL_FEATURES,
				write(chunk) {
					data.stdoutAnsi += chunk;
				},
			});

			const htmlReporterStream = client.reporter.addStream({
				format: "html",
				features: DEFAULT_TERMINAL_FEATURES,
				write(chunk) {
					data.stdoutHTML += chunk;
				},
			});

			const handles = [
				ansiReporterStream,
				htmlReporterStream,
				server.connectedReporters.addAttachedStream(ansiReporterStream.stream),
				server.connectedReporters.addAttachedStream(htmlReporterStream.stream),
			];

			client.bridge.endEvent.subscribe(() => {
				for (const handle of handles) {
					handle.remove();
				}

				data.endTime = Date.now();
				this.refreshRequests();
			});
		});

		server.requestStartEvent.subscribe((request) => {
			if (!this.savingRequests) {
				return;
			}

			const data: WebServerRequest = {
				id: request.id,
				client: request.client.id,
				query: request.query,
				markers: [],
				response: undefined,
				startTime: Date.now(),
				endTime: undefined,
			};
			this.clientRequestHistory.set(request.id, data);
			this.refreshRequests();

			request.markerEvent.subscribe((marker) => {
				data.markers.push(marker);
				this.refreshRequests();
			});

			request.endEvent.subscribe((response) => {
				// Update completion fields
				data.response = response;
				data.endTime = Date.now();
				this.refreshRequests();
			});
		});
	}

	public reporter: Reporter;

	private bundlerCache: Map<string, Bundler>;

	private savingRequests: boolean;
	private clientRequestHistory: Map<number, WebServerRequest>;
	private clientHistory: Map<number, WebServerClient>;

	private websocketBridges: Set<WebBridge>;

	private serverRequest: ServerRequest;
	private server: Server;
	private httpServer: http.Server;

	public onWebsocketBridge(req: http.IncomingMessage, bridge: WebBridge) {
		this.websocketBridges.add(bridge);

		req.socket.on(
			"close",
			() => {
				this.websocketBridges.delete(bridge);
			},
		);
	}

	public sendRequests(bridge: WebBridge) {
		bridge.requests.send({
			requests: Array.from(this.clientRequestHistory.values()),
			clients: Array.from(this.clientHistory.values()),
		});
	}

	private refreshRequests() {
		for (const bridge of this.websocketBridges) {
			this.sendRequests(bridge);
		}
	}

	public close() {
		this.httpServer.close();
	}

	public listen(port: number) {
		this.httpServer.listen(port);

		this.reporter.clearScreen();
		const url = `http://localhost:${String(port)}`;
		this.reporter.success(
			markup`Listening on <hyperlink emphasis>${url}</hyperlink>`,
		);
		this.reporter.info(
			markup`Web console available at <hyperlink emphasis>${url}/__rome__</hyperlink>`,
		);
	}

	public async pathnameToAbsolutePath(
		pathname: string,
	): Promise<undefined | AbsoluteFilePath> {
		const project = await this.serverRequest.assertClientCwdProject();
		const possibleStaticPath = project.directory.append(pathname);

		// This check makes sure that files outside of the project directory cannot be served
		if (possibleStaticPath.isRelativeTo(project.directory)) {
			return possibleStaticPath;
		} else {
			return undefined;
		}
	}

	public async getBundler(
		url: ConsumableUrl,
	): Promise<{
		bundler: Bundler;
		path: AbsoluteFilePath;
	}> {
		const pathname = stripBundleSuffix(String(url.path.asString()));

		const absolute = await this.pathnameToAbsolutePath(pathname);
		if (absolute === undefined) {
			throw new Error("Pathname is attempting to escalate out of cwd");
		}

		const pathPointer = url.path.getDiagnosticLocation();
		const path = await this.server.resolver.resolveEntryAssertPath(
			{
				origin: this.serverRequest.client.flags.cwd,
				source: absolute,
			},
			pathPointer === undefined ? undefined : {location: pathPointer},
		);

		const platform = url.query.get("platform").asStringSetOrVoid(PLATFORMS);
		const cacheKey = JSON.stringify({
			platform,
		});

		const cached = this.bundlerCache.get(cacheKey);
		if (cached !== undefined) {
			return {bundler: cached, path};
		}

		const bundlerConfig: BundlerConfig = this.serverRequest.getBundlerConfigFromFlags({
			platform,
		});

		const bundler = new Bundler(this.serverRequest, bundlerConfig);
		this.bundlerCache.set(cacheKey, bundler);
		return {bundler, path};
	}
}
