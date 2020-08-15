/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {Position} from "@internal/parser-core";
import {ErrorFrame, ErrorFrames} from "./types";
import {isPlainObject} from "@internal/typescript-helpers";
import {ob1Number0, ob1Number1} from "@internal/ob1";
import {
	NodeSystemError,
	convertPossibleNodeErrorToDiagnostic,
} from "@internal/node";
import {DiagnosticLocation} from "@internal/diagnostics";

export * from "./types";

export const ERROR_FRAMES_PROP = "ERROR_FRAMES";

export type ErrorWithFrames = NodeSystemError & {
	[ERROR_FRAMES_PROP]?: unknown;
};

export type StructuredNodeSystemErrorProperties = {
	address: undefined | string;
	code: undefined | string;
	dest: undefined | string;
	errno: undefined | number;
	path: undefined | string;
	port: undefined | string;
	syscall: undefined | string;
};

export type StructuredError = {
	name: string;
	message?: string;
	stack?: string;
	frames: ErrorFrames;
	node: StructuredNodeSystemErrorProperties;
};

export function setErrorFrames(
	err: ErrorWithFrames,
	frames: undefined | ErrorFrames,
) {
	err[ERROR_FRAMES_PROP] = frames;
}

export function setNodeErrorProps(
	err: NodeSystemError,
	props: Partial<StructuredNodeSystemErrorProperties>,
) {
	err.address = props.address;
	err.code = props.code;
	err.dest = props.dest;
	err.errno = props.errno;
	err.path = props.path;
	err.port = props.port;
	err.syscall = props.syscall;
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
): StructuredNodeSystemErrorProperties {
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
		return {};
	}

	const pos: Position = {
		line: frame.lineNumber ?? ob1Number1,
		column: frame.columnNumber ?? ob1Number0,
	};

	return {
		filename: frame.filename ?? "unknown",
		start: pos,
		end: pos,
	};
}
