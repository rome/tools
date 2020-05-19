/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {Position, SourceLocation} from "@romejs/parser-core";
import {ErrorFrame, ErrorFrames} from "./types";
import {isPlainObject} from "@romejs/typescript-helpers";
import {ob1Number0, ob1Number0Neg1, ob1Number1} from "@romejs/ob1";

export * from "./types";

export const ERROR_FRAMES_PROP = Symbol();

export type StructuredError = {
	name: string;
	message?: string;
	stack?: string;
	frames: ErrorFrames;
};

export function getErrorStructure(
	err: unknown,
	framesToShift: number = 0,
): StructuredError {
	let name = "Error";
	let message = "Unknown message";
	let stack = undefined;
	let frames: ErrorFrames = [];
	let looksLikeValidError = false;

	if (
		isPlainObject<{
			[ERROR_FRAMES_PROP]: unknown;
		}>(err)
	) {
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
		line: frame.lineNumber === undefined ? ob1Number1 : frame.lineNumber,
		column: frame.columnNumber === undefined ? ob1Number0 : frame.columnNumber,
	};

	return {
		filename: frame.filename === undefined ? "unknown" : frame.filename,
		start: pos,
		end: pos,
	};
}
