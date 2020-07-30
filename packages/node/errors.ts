import {
	ErrorWithFrames,
	getErrorStructure,
	setErrorFrames,
} from "@romefrontend/v8";
import {Markup, joinMarkupLines, markup} from "@romefrontend/markup";
import {
	Diagnostic,
	DiagnosticAdvice,
	INTERNAL_ERROR_LOG_ADVICE,
	createSingleDiagnosticError,
	getErrorStackAdvice,
} from "@romefrontend/diagnostics";
import {markupToPlainText} from "@romefrontend/cli-layout";

function changeMessage(
	old: ErrorWithFrames,
	msg: Markup,
	diagnostic: boolean = true,
): Error {
	const struct = getErrorStructure(old, 0, false);
	let err: ErrorWithFrames;

	if (diagnostic) {
		let advice: DiagnosticAdvice = [];

		if (old.path !== undefined && struct.frames.length === 0) {
			// If we are an fs error with no frames then recommend adding the envvar so the @romefrontend/fs module will
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
		}

		const diag: Diagnostic = {
			description: {
				category: "internalError/fs",
				message: msg,
				advice: [...advice, INTERNAL_ERROR_LOG_ADVICE],
			},

			location: {
				filename: old.path,
			},
		};

		// Create diagnostic error
		err = createSingleDiagnosticError(diag);
	} else {
		err = new Error(joinMarkupLines(markupToPlainText(msg)));
	}

	// Add on remaining regular error props so it can be treated as a normal error if necessary

	// Inherit some NodeJS.ErrnoException props
	err.code = old.code;
	err.path = old.path;
	err.syscall = old.syscall;

	// Without doing something jank we can't retain the original Error constructor ie. TypeError etc
	// We probably don't need to or actually care
	err.name = old.name;

	// Copy frames
	setErrorFrames(err, struct.frames);

	return err;
}

function convertNodeErrorWithPath(
	err: NodeJS.ErrnoException,
	path: string,
): Error {
	switch (err.code) {
		case "ENOENT":
			return changeMessage(
				err,
				markup`<emphasis>${path}</emphasis> does not exist`,
			);

		case "EPERM":
			return changeMessage(
				err,
				markup`Operation not permitted on <emphasis>${path}</emphasis>`,
			);

		case "EACCES":
			return changeMessage(
				err,
				markup`Permission denied <emphasis>${path}</emphasis>`,
			);

		case "EISDIR":
			return changeMessage(
				err,
				markup`Trying to perform a file operation on the folder <emphasis>${path}</emphasis>`,
			);

		default:
			return err;
	}
}

export function convertPossibleNodeErrorToDiagnostic(
	err: NodeJS.ErrnoException,
): Error {
	if (err.path !== undefined) {
		return convertNodeErrorWithPath(err, err.path);
	}

	switch (err.code) {
		case "EPIPE":
			return changeMessage(err, markup`Pipe closed on other end`, false);
	}

	return err;
}
