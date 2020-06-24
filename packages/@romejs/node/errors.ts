import {ERROR_FRAMES_PROP, ErrorWithFrames} from "@romejs/v8";

function changeMessage(old: ErrorWithFrames, msg: string): Error {
	const err: ErrorWithFrames = new Error(msg);

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

		default:
			return err;
	}
}

export function convertPossibleNodeError(err: NodeJS.ErrnoException): Error {
	if (err.path !== undefined) {
		return convertNodeErrorWithPath(err, err.path);
	}

	return err;
}
