import {printDiagnostics} from "@internal/cli-diagnostics";
import {Reporter} from "@internal/cli-reporter";
import {StaticMarkup} from "@internal/markup";
import {
	DiagnosticsProcessor,
	getOrDeriveDiagnosticsFromError,
} from "@internal/diagnostics";

type HandleFatalErrorOptions = {
	source: StaticMarkup;
	error: Error;
	reporter: Reporter;
	exit?: boolean;
};

export default function handleFatalError(opts: HandleFatalErrorOptions) {
	// Swallow promise. Should never throw an error.
	_handleFatalError(opts).then();
}

async function _handleFatalError(
	{error, source, reporter, exit = true}: HandleFatalErrorOptions,
) {
	try {
		const diagnostics = getOrDeriveDiagnosticsFromError(
			error,
			{
				description: {
					category: "internalError/fatal",
				},
				label: source,
				tags: {
					internal: true,
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
			process.exit(1);
		}
	}
}
