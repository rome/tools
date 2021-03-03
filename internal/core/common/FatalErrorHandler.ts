import {DiagnosticsPrinter} from "@internal/cli-diagnostics";
import {Reporter, WrapperFactory} from "@internal/cli-reporter";
import {StaticMarkup} from "@internal/markup";
import {
	DIAGNOSTIC_CATEGORIES,
	DiagnosticsProcessor,
	getOrDeriveDiagnosticsFromError,
} from "@internal/diagnostics";
import {ErrorCallback} from "@internal/typescript-helpers";
import {Resource, createResourceFromCallback} from "@internal/resources";
import util = require("util");

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
	}

	public handleBound: ErrorCallback;
	public wrapBound: WrapperFactory;

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
		const onUncaughtException: NodeJS.UncaughtExceptionListener = (err: Error) => {
			this.handle(err);
		};
		process.on("uncaughtException", onUncaughtException);

		const onUnhandledRejection: NodeJS.UnhandledRejectionListener = (
			errRaw: unknown,
		) => {
			let err: Error;
			if (util.types.isNativeError(errRaw)) {
				err = errRaw;
			} else {
				err = new Error(String(errRaw));
			}
			this.handle(err);
		};
		process.on("unhandledRejection", onUnhandledRejection);

		return createResourceFromCallback(
			"FatalErrorHandlerEvents",
			() => {
				process.removeListener("uncaughtException", onUncaughtException);
				process.removeListener("unhandledRejection", onUnhandledRejection);
			},
		);
	}

	public handle(error: Error, overrideSource?: StaticMarkup): void {
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

		try {
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

			const processor = new DiagnosticsProcessor();
			processor.addDiagnostics(diagnostics);
			const printer = new DiagnosticsPrinter({
				reporter,
				processor,
			});
			printer.printBodySync();
		} catch (logErr) {
			console.error(
				"Failed to output detailed fatal error information. Original error:",
			);
			console.error(error.stack);
			console.error("Log error:");
			console.error(logErr.stack);
		} finally {
			if (this.options.exit !== false) {
				// Deliberately do not use safeProcessExit
				process.exit(1);
			}
		}
	}
}
