import {ERROR_FRAMES_PROP, ErrorWithFrames} from "@romefrontend/v8";

function changeMessage(old: ErrorWithFrames, msg: string): Error {
	const err: ErrorWithFrames = new Error(msg);

	// Inherit some NodeJS.ErrnoException props
	err.code = old.code;
	err.path = old.path;
	err.syscall = old.syscall;

	// Without doing something jank we can't retain the original Error constructor ie. TypeError etc
	// We probably don't need to or actually care
	err.name = old.name;

	// Populate ERROR_FRAMES_PROP
	old.stack;
	err[ERROR_FRAMES_PROP] = old[ERROR_FRAMES_PROP];
	err.stack;

	return err;
}

function convertNodeErrorWithPath(
	err: NodeJS.ErrnoException,
	path: string,
): Error {
	switch (err.code) {
		case "ENOENT":
			return changeMessage(err, `'${path}' does not exist`);

		case "EPERM":
			return changeMessage(err, `Cannot access '${path}'`);

		case "EISDIR":
			return changeMessage(
				err,
				`Trying to perform a file operation on the folder '${path}'`,
			);

		default:
			return err;
	}
}

export function convertPossibleNodeError(err: NodeJS.ErrnoException): Error {
	if (err.path !== undefined) {
		return convertNodeErrorWithPath(err, err.path);
	}

	switch (err.code) {
		case "EPIPE":
			return changeMessage(err, "Pipe closed on other end");
	}

	return err;
}
