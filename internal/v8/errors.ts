/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {Position, SourceLocation} from "@internal/parser-core";
import {ErrorFrame, ErrorFrames} from "./types";
import {isPlainObject} from "@internal/typescript-helpers";
import {ob1Number0, ob1Number0Neg1, ob1Number1} from "@internal/ob1";
import {convertPossibleNodeErrorToDiagnostic} from "@internal/node";

export * from "./types";

export const ERROR_FRAMES_PROP = "ERROR_FRAMES";

export type ErrorWithFrames = NodeJS.ErrnoException & {
	[ERROR_FRAMES_PROP]?: unknown;
};

export type StructuredError = {
	name: string;
	message?: string;
	stack?: string;
	frames: ErrorFrames;
};

export function setErrorFrames(
	err: ErrorWithFrames,
	frames: undefined | ErrorFrames,
) {
	err[ERROR_FRAMES_PROP] = frames;
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
	};
}

export function getSourceLocationFromErrorFrame(
	frame: ErrorFrame,
): SourceLocation {
	const pos: Position = {
		index: ob1Number0Neg1,
		line: frame.lineNumber ?? ob1Number1,
		column: frame.columnNumber ?? ob1Number0,
	};

	return {
		filename: frame.filename ?? "unknown",
		start: pos,
		end: pos,
	};
}
