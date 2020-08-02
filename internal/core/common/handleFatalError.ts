import {printDiagnostics} from "@internal/cli-diagnostics";
import {Reporter} from "@internal/cli-reporter";
import {StaticMarkup} from "@internal/markup";
import {
	DiagnosticsProcessor,
	getOrDeriveDiagnosticsFromError,
} from "@internal/diagnostics";

export default async function handleFatalError(
	{error, source, reporter}: {
		source: StaticMarkup;
		error: Error;
		reporter: Reporter;
	},
) {
	try {
		const diagnostics = getOrDeriveDiagnosticsFromError(
			error,
			{
				description: {
					category: "internalError/fatal",
				},
			},
		);

		const processor = new DiagnosticsProcessor({
			normalizeOptions: {
				label: source,
				tags: {
					internal: true,
					fatal: true,
				},
			},
		});

		await printDiagnostics({
			diagnostics,
			suppressions: [],
			excludeFooter: true,
			printerOptions: {
				reporter,
				processor,
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
		process.exit(1);
	}
}
