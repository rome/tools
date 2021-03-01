import {printDiagnostics} from "@internal/cli-diagnostics";
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

type FatalErrorHandlerOptions = {
	getOptions: (
		err: Error,
	) =>
		| false
		| {
				source: StaticMarkup;
				reporter: Reporter;
				exit?: boolean;
			};
};

export default class FatalErrorHandler {
	constructor(opts: FatalErrorHandlerOptions) {
		this.options = opts;
		this.handling = false;
		this.handleBound = this.handle.bind(this);
		this.wrapBound = this.wrap.bind(this);
	}

	public handleBound: ErrorCallback;
	public wrapBound: WrapperFactory;

	private options: FatalErrorHandlerOptions;
	private handling: boolean;

	public handle(err: Error, overrideSource?: StaticMarkup) {
		// Swallow promise. Should never throw an error.
		this.handleAsync(err, overrideSource).then();
	}

	public wrapPromise(promise: Promise<unknown>) {
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
			err: unknown,
		) => {
			this.handle(err instanceof Error ? err : new Error(String(err)));
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

	public async handleAsync(
		error: Error,
		overrideSource?: StaticMarkup,
	): Promise<void> {
		const {getOptions} = this.options;
		const options = getOptions(error);
		if (options === false) {
			return;
		}

		if (this.handling) {
			// Already have a fatal error queued
			return;
		}
		this.handling = true;

		const {reporter, exit = true} = options;
		const source = overrideSource ?? options.source;

		try {
			const diagnostics = getOrDeriveDiagnosticsFromError(
				error,
				{
					description: {
						category: DIAGNOSTIC_CATEGORIES["internalError/fatal"],
					},
					label: source,
					tags: {
						fatal: true,
					},
				},
			);

			await printDiagnostics({
				diagnostics,
				suppressions: [],
				excludeFooter: true,
				printerOptions: {
					reporter,
					processor: new DiagnosticsProcessor(),
				},
			});
		} catch (logErr) {
			console.error(
				"Failed to output detailed fatal error information. Original error:",
			);
			console.error(error.stack);
			console.error("Log error:");
			console.error(logErr.stack);
		} finally {
			if (exit) {
				await safeProcessExit(1);
			}
		}
	}
}
