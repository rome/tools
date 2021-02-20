import {Position} from "@internal/parser-core";
import {ErrorFrame, ErrorFrames} from "@internal/errors";
import {isPlainObject} from "@internal/typescript-helpers";
import {OneIndexed, ZeroIndexed} from "@internal/math";
import {NodeSystemError, NodeSystemErrorProperties} from "./types";
import {
	DiagnosticLocation,
	convertPossibleNodeErrorToDiagnostic,
} from "@internal/diagnostics";
import {UNKNOWN_PATH} from "@internal/path";

export function setNodeErrorProps(
	err: NodeSystemError,
	props: Partial<NodeSystemErrorProperties>,
) {
	err.address = props.address;
	err.code = props.code;
	err.dest = props.dest;
	err.errno = props.errno;
	err.path = props.path;
	err.port = props.port;
	err.syscall = props.syscall;
}

export * from "./types";

export const ERROR_FRAMES_PROP = "ERROR_FRAMES";

export type ErrorWithFrames = NodeSystemError & {
	[ERROR_FRAMES_PROP]?: unknown;
};

export type StructuredError = {
	name: string;
	message?: string;
	stack?: string;
	frames: ErrorFrames;
	node: NodeSystemErrorProperties;
};

export function setErrorFrames(
	err: ErrorWithFrames,
	frames: undefined | ErrorFrames,
) {
	Object.defineProperty(err, ERROR_FRAMES_PROP, {
		enumerable: false,
		configurable: true,
		writable: true,
		value: frames,
	});
}

export function getErrorStructure(
	err: unknown,
	framesToShift: number = 0,
	shouldConvertPossibleNodeError: boolean = true,
): StructuredError {
	// Make some node errors more pretty
	if (err instanceof Error && shouldConvertPossibleNodeError) {
		err = convertPossibleNodeErrorToDiagnostic(err);
	}

	let name = "Error";
	let message = "Unknown message";
	let stack = undefined;
	let frames: ErrorFrames = [];
	let looksLikeValidError = false;

	if (isPlainObject<ErrorWithFrames>(err)) {
		if (typeof err.name === "string") {
			looksLikeValidError = true;
			name = err.name;
		}

		if (typeof err.message === "string") {
			looksLikeValidError = true;
			message = err.message;
		}

		if (typeof err.stack === "string") {
			looksLikeValidError = true;
			stack = err.stack;
		}

		if (Array.isArray(err[ERROR_FRAMES_PROP])) {
			// @ts-ignore
			frames = err[ERROR_FRAMES_PROP];
		}
	}

	frames = frames.slice(framesToShift);

	if (!looksLikeValidError) {
		message = `Not an error instance: ${String(err)}`;
	}

	return {
		name,
		message,
		stack,
		frames,
		node: extractNodeSystemErrorProperties(err),
	};
}

export function extractNodeSystemErrorProperties(
	err: unknown,
): NodeSystemErrorProperties {
	if (isPlainObject(err)) {
		return {
			address: typeof err.address === "string" ? err.address : undefined,
			code: typeof err.code === "string" ? err.code : undefined,
			dest: typeof err.dest === "string" ? err.dest : undefined,
			errno: typeof err.errno === "number" ? err.errno : undefined,
			path: typeof err.path === "string" ? err.path : undefined,
			port: typeof err.port === "string" ? err.port : undefined,
			syscall: typeof err.syscall === "string" ? err.syscall : undefined,
		};
	} else {
		return {
			address: undefined,
			code: undefined,
			dest: undefined,
			errno: undefined,
			path: undefined,
			port: undefined,
			syscall: undefined,
		};
	}
}

export function getDiagnosticLocationFromErrorFrame(
	frame: undefined | ErrorFrame,
): DiagnosticLocation {
	if (frame === undefined) {
		return {
			path: UNKNOWN_PATH,
		};
	}

	const pos: Position = {
		line: frame.lineNumber ?? new OneIndexed(),
		column: frame.columnNumber ?? new ZeroIndexed(),
	};

	return {
		path: frame.path ?? UNKNOWN_PATH,
		start: pos,
		end: pos,
	};
}
