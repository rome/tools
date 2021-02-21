import { Reporter } from "@internal/cli-reporter";
import { ServerRequest } from "@internal/core";
import { BundlerFile } from "@internal/core/common/types/bundler";
import {Event} from "@internal/events";
import Bundler from "../bundler/Bundler";
import http = require("http");

export default class DevelopServer {
  constructor(req: ServerRequest, reporter: Reporter) {
    this.request = req;
    this.reporter = reporter;
    this.bundler = Bundler.createFromServerRequest(req);
    this.closeEvent = new Event({
      name: "closeEvent",
    });

    this.knownBundleFiles = new Map();
  }

  private reporter: Reporter;
  private request: ServerRequest;
  private knownBundleFiles: Map<string, BundlerFile>;
  private bundler: Bundler;
  private closeEvent: Event<void, void>;

  public async init() {
    const {bundler} = this;
		const resolution = await bundler.getResolvedEntry(".");
    const {diagnosticsEvent, filesEvent, changeEvent} = bundler.bundleManifestWatch(resolution);

    diagnosticsEvent.subscribe(async (diagnostics) => {
      reporter.clearScreen();
      const printer = req.createDiagnosticsPrinter();
      printer.processor.addDiagnostics(diagnostics);
      await printer.print();
    });

    filesEvent.subscribe(([name]) => {
      console.log(name);
    });

    changeEvent.subscribe(paths => {
      if (paths.length === 1) {
        reporter.info(markup`File change ${paths[0]}`);
      } else {
        reporter.info(markup`Multiple file changes`);
        reporter.list(paths);
      }
    });
  }

  private async handleRequest(req: http.IncomingMessage, res: http.ServerResponse) {

  }

  public listen(port: number): http.Server {
    const server = http.createServer((req: http.IncomingMessage, res: http.ServerResponse) => {
      this.handleRequest(req, res).catch(err => {

      });
    });

    server.listen(port);

    return server;
  }

  public close() {}
}