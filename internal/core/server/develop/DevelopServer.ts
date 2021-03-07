import {Reporter} from "@internal/cli-reporter";
import {ServerRequest} from "@internal/core";
import {
	BundlerEntryResolution,
	BundlerFile,
} from "@internal/core/common/types/bundler";
import Bundler from "../bundler/Bundler";
import http = require("http");
import {markup} from "@internal/markup";
import {
	AbsoluteFilePath,
	AbsoluteFilePathMap,
	RelativePathMap,
} from "@internal/path";
import DevelopRequest from "./DevelopRequest";
import {Resource} from "@internal/resources";

type DevelopServerOptions = {
	bundler: Bundler;
	resolution: BundlerEntryResolution;
	reporter: Reporter;
	request: ServerRequest;
};

export type DevelopServerListenOptions = {
	port: number;
	public: boolean;
};

export default class DevelopServer {
	constructor(opts: DevelopServerOptions) {
		this.request = opts.request;
		this.reporter = opts.reporter;
		this.bundler = opts.bundler;

		this.resolution = opts.resolution;
		this.staticPath = opts.resolution.project.directory.append("static");

		this.resources = this.request.resources.createContainer("DevelopServer");
		this.ready = false;
		this.knownBundleFiles = new RelativePathMap();
		this.staticETags = new AbsoluteFilePathMap();
	}

	private request: ServerRequest;
	private resources: Resource;

	public reporter: Reporter;
	public knownBundleFiles: RelativePathMap<BundlerFile>;
	public bundler: Bundler;
	public resolution: BundlerEntryResolution;
	public staticPath: AbsoluteFilePath;
	public ready: boolean;
	public staticETags: AbsoluteFilePathMap<{
		mtime: bigint;
		etag: string;
	}>;

	public async init() {
		const {bundler, reporter, request, resolution} = this;
		const {diagnosticsEvent, filesEvent, changeEvent, subscription} = bundler.bundleManifestWatch(
			resolution,
		);

		let hasDiagnostics = false;

		diagnosticsEvent.subscribe(async (diagnostics) => {
			hasDiagnostics = true;
			reporter.clearScreen();

			const stream = reporter.attachCaptureStream("html");

			const printer = request.createDiagnosticsPrinter();
			printer.processor.addDiagnostics(diagnostics);
			await printer.print();

			stream.resources.release();
			const html = stream.read();
			// TODO send HTML
		});

		filesEvent.subscribe((files) => {
			for (const [name, def] of files) {
				if (def === undefined) {
					this.knownBundleFiles.delete(name);
				} else {
					this.knownBundleFiles.set(name, def);
				}
			}

			// Consider ourselves initialized when we've received our first batch of files
			this.ready = true;

			if (hasDiagnostics) {
				hasDiagnostics = false;
				reporter.clearScreen();
			}

			this.refresh();
		});

		// TODO also watch `static` and refresh on changes

		changeEvent.subscribe((paths) => {
			if (paths.size === 1) {
				reporter.info(markup`File change ${Array.from(paths)[0]}`);
			} else {
				reporter.info(markup`Multiple file changes`);
				reporter.list(Array.from(paths));
			}
		});

		this.resources.add(subscription);
	}

	private async refresh() {
		// TODO tell all connected websockets to refresh
	}

	public async listen(opts: DevelopServerListenOptions): Promise<http.Server> {
		const {reporter} = this;

		const server = http.createServer((
			request: http.IncomingMessage,
			response: http.ServerResponse,
		) => {
			const devRequest = new DevelopRequest({
				httpServer: server,
				request,
				response,
				server: this,
			});
			devRequest.handle();
		});

		return new Promise((resolve, reject) => {
			server.addListener("error", reject);

			const {port} = opts;
			const address = opts.public ? "0.0.0.0" : "127.0.0.1";

			server.listen(
				port,
				address,
				() => {
					server.removeListener("error", reject);
					server.addListener(
						"error",
						this.request.server.fatalErrorHandler.handleBound,
					);

					reporter.success(
						markup`Ready at <emphasis><hyperlink target="http://localhost:${String(
							port,
						)}" /></emphasis>`,
					);

					if (opts.public) {
						reporter.warn(
							markup`The <emphasis>public</emphasis> flag has been set which makes the server accessible to others on your network, or possibly the whole internet depending on your network configuration. Express caution when using this flag.`,
						);
					} else {
						// NB: Not sure if it's worth pointing out that we're on listening on a loopback interface
					}

					resolve(server);
				},
			);
		});
	}

	public async close() {
		await this.resources.release();
	}
}
