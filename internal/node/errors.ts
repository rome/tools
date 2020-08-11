import {
	StructuredNodeSystemErrorProperties,
	getDiagnosticLocationFromErrorFrame,
	getErrorStructure,
	setErrorFrames,
	setNodeErrorProps,
} from "@internal/v8";
import {StaticMarkup, markup} from "@internal/markup";
import {
	Diagnostic,
	DiagnosticAdvice,
	DiagnosticCategory,
	DiagnosticLocation,
	DiagnosticsError,
	createSingleDiagnosticError,
	getErrorStackAdvice,
} from "@internal/diagnostics";
import {prettyFormatEager} from "@internal/pretty-format";
import {createAbsoluteFilePath} from "@internal/path";
import {lstatSync} from "@internal/fs";

function getPathFromNodeError(err: NodeSystemError): undefined | string {
	return err.path ?? err.address;
}

function getMessageFromNodeError(
	err: NodeSystemError,
): {
	message: StaticMarkup;
	advice?: DiagnosticAdvice;
} {
	const path = getPathFromNodeError(err);

	switch (err.code) {
		case "ENOENT":
			return {message: markup`<emphasis>${path}</emphasis> does not exist`};

		case "EPERM":
			return {
				message: markup`Operation not permitted on <emphasis>${path}</emphasis>`,
			};

		case "EACCES":
			return {
				message: markup`Permission denied for <emphasis>${path}</emphasis>`,
			};

		case "EISDIR":
			return {
				message: markup`Trying to perform a file operation on the folder <emphasis>${path}</emphasis>`,
			};

		case "EROFS":
			return {
				message: markup`<emphasis>${path}</emphasis> is a read-only file system`,
			};

		case "EPIPE":
			return {message: markup`Pipe closed on other end`};

		case "EMFILE":
			return {
				message: markup`Too many open files`,
				advice: [
					{
						type: "log",
						category: "info",
						text: markup`This is encountered when opening many files at once in parallel. To increase this limit you can try running:`,
					},
					{
						type: "command",
						command: "ulimit -n 2048",
					},
					{
						type: "log",
						category: "info",
						text: markup`Inside the current terminal then run this command again.`,
					},
				],
			};

		default:
			return {message: markup`${err.message}`};
	}
}

export function convertPossibleNodeErrorToDiagnostic(
	err: NodeSystemError,
): Error {
	if (err.code === undefined || err instanceof DiagnosticsError) {
		return err;
	}

	let {message, advice = []} = getMessageFromNodeError(err);
	const struct = getErrorStructure(err, 0, false);
	let location: DiagnosticLocation = {};

	if (err.path !== undefined && struct.frames.length === 0) {
		// If we are an fs error with no frames then recommend adding the envvar so the @internal/fs module will
		// manually capture and set stacktraces
		advice.push({
			type: "log",
			category: "warn",
			text: markup`No stacktrace available for this error. This is a Node.js limitation: <hyperlink target="https://github.com/nodejs/node/issues/30944" />`,
		});

		advice.push({
			type: "log",
			category: "info",
			text: markup`Try setting the <code>ROME_FS_ERRORS=1</code> envvar to capture stacktraces for fs calls.`,
		});
	} else {
		advice = getErrorStackAdvice(struct);
		location = getDiagnosticLocationFromErrorFrame(struct.frames[0]);
	}

	let category: DiagnosticCategory = "internalError/fatal";
	if (err.path !== undefined) {
		category = "internalError/fs";
	} else {
		// TODO probably others
	}

	// If provided with a path, try and get some debug stats
	const pathStr = getPathFromNodeError(err);
	if (pathStr !== undefined) {
		try {
			const path = createAbsoluteFilePath(pathStr);

			for (const parent of path.getChain()) {
				try {
					const stat = lstatSync(parent);

					advice.push({
						type: "group",
						title: markup`File stats for <emphasis>${parent}</emphasis>`,
						advice: [
							{
								type: "log",
								category: "none",
								text: prettyFormatEager({
									...stat,
									isFile: stat.isFile(),
									isDirectory: stat.isDirectory(),
									isBlockDevice: stat.isBlockDevice(),
									isCharacterDevice: stat.isCharacterDevice(),
									isSymbolicLink: stat.isSymbolicLink(),
									isFIFO: stat.isFIFO(),
									isSocket: stat.isSocket(),
								}),
							},
						],
					});
					break;
				} catch (err) {
					// Swallow
				}
			}
		} catch (err) {
			// Swallow
		}
	}

	const diag: Diagnostic = {
		description: {
			category,
			message,
			advice,
		},
		location,
		tags: {
			internal: true,
		},
	};

	// Create diagnostic error
	const diagErr: NodeSystemError = createSingleDiagnosticError(diag);

	// Add on remaining regular error props so it can be treated as a normal error if necessary

	// Inherit NodeSystemError props
	setNodeErrorProps(diagErr, err);

	// Without doing something jank we can't retain the original Error constructor ie. TypeError etc
	// We probably don't need to or actually care
	diagErr.name = err.name;

	// Copy frames
	setErrorFrames(diagErr, struct.frames);

	return diagErr;
}

// https://nodejs.org/api/errors.html#errors_class_systemerror
export type NodeSystemError = Error &
	Partial<StructuredNodeSystemErrorProperties>;
