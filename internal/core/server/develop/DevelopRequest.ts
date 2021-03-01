import DevelopServer from "./DevelopServer";
import {Path, RelativePath, URLPath, createURLPath} from "@internal/path";
import {getFileHandlerFromPath} from "@internal/core/common/file-handlers";
import stream = require("stream");
import http = require("http");
import {AnyMarkup, markup} from "@internal/markup";
import {sha256} from "@internal/string-utils";

type DevelopRequestOptions = {
	httpServer: http.Server;
	request: http.IncomingMessage;
	response: http.ServerResponse;
	server: DevelopServer;
};

function isLocalAddress(address: undefined | string): boolean {
	return address === "localhost" || address === "127.0.0.1" || address === "::1";
}

export default class DevelopRequest {
	constructor({request, response, httpServer, server}: DevelopRequestOptions) {
		this.req = request;
		this.res = response;
		this.httpServer = httpServer;
		this.server = server;

		// Infer `host`
		let host = request.headers.host;
		if (host === undefined) {
			const addressInfo = httpServer.address();
			if (typeof addressInfo === "string") {
				host = addressInfo;
			} else if (addressInfo == null) {
				throw new Error(
					"server.address() should not be null as in order to receive a socket we must be listening",
				);
			} else {
				host = `${addressInfo.address}`;
				if (addressInfo.port !== 80) {
					host += `:${addressInfo.port}`;
				}
			}
		}

		// Fetch requested pathname
		const pathnameStr = request.url;
		if (typeof pathnameStr !== "string" || pathnameStr[0] !== "/") {
			throw new Error("Malformed req.url");
		}

		this.url = createURLPath(`http://${host}${pathnameStr}`);
		this.pathname = this.url.getPathname();
	}

	private httpServer: http.Server;
	private req: http.IncomingMessage;
	private res: http.ServerResponse;
	private server: DevelopServer;
	private url: URLPath;
	private pathname: RelativePath;

	private end(chunk?: string | ArrayBuffer): Promise<void> {
		return new Promise((resolve) => {
			if (chunk instanceof ArrayBuffer) {
				chunk = Buffer.from(chunk);
			}

			this.res.end(chunk, resolve);
		});
	}

	private pipe(val: stream.Readable | string | ArrayBuffer): Promise<void> {
		if (val instanceof ArrayBuffer || typeof val === "string") {
			return this.end(val);
		}

		return new Promise((resolve, reject) => {
			val.pipe(this.res);
			val.on("error", reject);
			val.on("close", resolve);
		});
	}

	private setResponseMimeType(path: Path) {
		const {handler} = getFileHandlerFromPath(
			path,
			this.server.resolution.project.config,
		);
		if (handler !== undefined) {
			this.res.setHeader("Content-Type", handler.mime);
		}
	}

	private async matchETag(etag: string): Promise<boolean> {
		const wrapped = `"${etag}"`;
		this.res.setHeader("ETag", wrapped);
		this.res.setHeader("Cache-Control", "must-revalidate");
		if (this.req.headers["if-none-match"] === wrapped) {
			this.res.statusCode = 304;
			await this.end();
			return true;
		} else {
			return false;
		}
	}

	private async maybeServeBundleFile(): Promise<boolean> {
		const bundleFile = this.server.knownBundleFiles.get(this.pathname);
		if (bundleFile === undefined) {
			return false;
		}

		this.setResponseMimeType(this.pathname);

		if (await this.matchETag(bundleFile.etag)) {
			return true;
		}

		const buff = await bundleFile.content();
		await this.pipe(buff);
		return true;
	}

	private async maybeServeStaticFile(
		relative: string | RelativePath,
	): Promise<boolean> {
		const {server} = this;
		const path = server.staticPath.append(relative);

		// Verify this is a path inside of staticPath, it could have been relativized out
		if (!path.isRelativeTo(server.staticPath)) {
			return false;
		}

		if (await path.notExists()) {
			return false;
		}

		const stats = await path.lstat();
		if (!stats.isFile()) {
			return false;
		}

		let knownEtag = this.server.staticETags.get(path);
		if (knownEtag?.mtime === stats.mtimeMs) {
			if (await this.matchETag(knownEtag.etag)) {
				return true;
			}
		}

		this.setResponseMimeType(path);

		const stream = path.createReadStream();
		const hasher = sha256.async(stream);
		await this.pipe(stream);

		// Save etag for next time
		const hash = await hasher;
		this.server.staticETags.set(
			path,
			{
				mtime: stats.mtimeMs,
				etag: hash,
			},
		);

		return true;
	}

	private async serve404(): Promise<void> {
		const {res} = this;
		res.statusCode = 404;

		const served404 = await this.maybeServeStaticFile("404.html");
		if (served404) {
			return;
		}

		// Default
		await this.end("Not found");
	}

	private log() {
		const {res, req} = this;

		let statusCode = res.statusCode;
		let statusCodeText = markup``;
		if (statusCode >= 100 && statusCode < 200) {
			statusCodeText = markup`<info>${statusCode}</info>`;
		} else if (statusCode >= 200 && statusCode < 300) {
			statusCodeText = markup`<success>${statusCode}</success>`;
		} else if (statusCode >= 300 && statusCode < 400) {
			statusCodeText = markup`<info>${statusCode}</info>`;
		} else if (statusCode >= 400 && statusCode < 500) {
			statusCodeText = markup`<warn>${statusCode}</warn>`;
		} else if (statusCode >= 500 && statusCode < 600) {
			statusCodeText = markup`<error>${statusCode}</error>`;
		} else {
			statusCodeText = markup`<dim>${statusCode}</dim>`;
		}

		let url: AnyMarkup = this.url;

		// Only include the pathname when it's localhost connecting to itself
		if (
			isLocalAddress(this.url.getHostname()) &&
			isLocalAddress(this.req.socket.remoteAddress)
		) {
			url = markup`<hyperlink target="${this.url.join()}">${this.url.joinPathname()}</hyperlink>`;
		}

		this.server.reporter.log(
			markup`<dim>${req.method}</dim> ${url} ${statusCodeText}`,
		);
	}

	public async handle(): Promise<void> {
		try {
			await this._handle();
		} catch (err) {
			throw err;
		} finally {
			this.log();
		}
	}

	public async _handle() {
		const {server, res} = this;

		// If we are still initializing and discovering what files can be accessed, return a placeholder that will
		// automatically refresh
		if (!server.ready) {
			res.setHeader("Content-Type", "text/html");
			await this.end(`<meta http-equiv="refresh" content="2">Starting...`);
			return;
		}

		// Check knownBundleFiles
		const servedBundleFile = await this.maybeServeBundleFile();
		if (servedBundleFile) {
			return;
		}

		// Check static
		const servedStaticVerbatim = await this.maybeServeStaticFile(this.pathname);
		if (servedStaticVerbatim) {
			return;
		}

		// Check if this is a directory
		const servedStaticIndex = await this.maybeServeStaticFile(
			this.pathname.append("index.html"),
		);
		if (servedStaticIndex) {
			return;
		}

		await this.serve404();
	}
}
