import {DiagnosticsPrinter} from "@internal/cli-diagnostics";
import {Reporter, WrapperFactory} from "@internal/cli-reporter";
import {StaticMarkup} from "@internal/markup";
import {
	DIAGNOSTIC_CATEGORIES,
	DiagnosticsProcessor,
	getOrDeriveDiagnosticsFromError,
} from "@internal/diagnostics";
import {ErrorCallback} from "@internal/typescript-helpers";
import {
	Resource,
	createResourceFromCallback,
	safeProcessExit,
} from "@internal/resources";
import util = require("util");
import {GlobalLock} from "@internal/async";

type FatalErrorHandlerOptions = {
	exit?: boolean;
	source?: StaticMarkup;
	overrideHandle?: (err: Error) => boolean;
	getReporter?: () => Reporter;
};

export default class FatalErrorHandler {
	constructor(opts: FatalErrorHandlerOptions) {
		this.options = opts;
		this.handleBound = this.handle.bind(this);
		this.wrapBound = this.wrap.bind(this);
		this.handleQueue = new GlobalLock();
	}

	public handleBound: ErrorCallback;
	public wrapBound: WrapperFactory;

	private handleQueue: GlobalLock;
	private options: FatalErrorHandlerOptions;

	public wrapPromise(promise: Promise<unknown>): void {
		promise.catch(this.handleBound);
	}

	// rome-ignore lint/ts/noExplicitAny: future cleanup
	public wrap<T extends (...args: any[]) => any>(callback: T): T {
		return ((...args: any[]): any => {
			try {
				const res = callback(...args);
				if (res instanceof Promise) {
					res.catch(this.handleBound);
				}
				return res;
			} catch (err) {
				this.handle(err);
				throw err;
			}
		}) as T;
	}

	public setupGlobalHandlers(): Resource {
		// Only pass first argument to handler
		const handleError = (err: Error) => this.handleBound(err);

		process.on("uncaughtException", handleError);
		process.on("unhandledRejection", handleError);

		return createResourceFromCallback(
			"FatalErrorHandlerEvents",
			() => {
				process.removeListener("uncaughtException", handleError);
				process.removeListener("unhandledRejection", handleError);
			},
		);
	}

	private async printErrorAsDiagnostics(reporter: Reporter, error: Error, overrideSource?: StaticMarkup): Promise<void> {
		const diagnostics = getOrDeriveDiagnosticsFromError(
			error,
			{
				description: {
					category: DIAGNOSTIC_CATEGORIES["internalError/fatal"],
				},
				label: overrideSource ?? this.options.source,
				tags: {
					fatal: true,
				},
			},
		);

		const processor = new DiagnosticsProcessor({
			normalizeOptions: {
				defaultTags: {
					fatal: true,
				},
			},
		});
		processor.addDiagnostics(diagnostics);

		const printer = new DiagnosticsPrinter({
			reporter,
			processor,
			flags: {
				truncateDiagnostics: false,
			},
		});
		await printer.print({
			showFooter: false,
		});
	}

	public handle(raw: Error, overrideSource?: StaticMarkup): Promise<void> {
		return this.handleQueue.series(async () => {
			let error: Error;
			if (util.types.isNativeError(raw)) {
				error = raw;
			} else {
				error = new Error(String(raw));
			}

			try {
				const {getReporter, overrideHandle} = this.options;
				if (overrideHandle !== undefined) {
					const handled = overrideHandle(error);
					if (handled) {
						return;
					}
				}

				let reporter: Reporter;
				if (getReporter === undefined) {
					reporter = Reporter.fromProcess();
				} else {
					reporter = getReporter();
				}

				if (reporter.hasStreams()) {
					await this.printErrorAsDiagnostics(reporter, error, overrideSource);
				} else {
					console.error("Reporter had no available streams. Error:");
					console.error(error.stack);
				}
			} catch (logErr) {
				console.error("Failed to handle fatal error. Original error:");
				console.error(error.stack);
				console.error("Log error:");
				console.error(logErr.stack);
			} finally {
				if (this.options.exit !== false) {
					await safeProcessExit(70);
				}
			}
		});
	}
}
