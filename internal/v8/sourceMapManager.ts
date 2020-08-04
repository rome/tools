/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {SourceMapConsumerCollection} from "@internal/codec-source-map";
import {ErrorFrame} from "@internal/v8";
import {ob1Coerce1, ob1Coerce1To0} from "@internal/ob1";
import {
	ERROR_FRAMES_PROP,
	ErrorWithFrames,
	getErrorStructure,
	setErrorFrames,
} from "./errors";

let inited: boolean = false;

function prepareStackTrace(err: Error, frames: Array<NodeJS.CallSite>) {
	try {
		addErrorFrames(err, frames);
		return buildStackString(err);
	} catch (err2) {
		return `${err.name}: ${err.message}\n  Failed to generate stacktrace: ${err2.message}`;
	}
}

export function initErrorHooks() {
	if (!inited) {
		inited = true;
		Error.prepareStackTrace = prepareStackTrace;
	}
}

export function teardown() {
	Error.prepareStackTrace = undefined;
}

function buildStackString(err: Error): string {
	const {frames} = getErrorStructure(err);
	const lines: Array<string> = [];

	lines.push(`${err.name}: ${err.message}`);

	for (const frame of frames) {
		const {
			resolvedLocation,
			methodName,
			functionName,
			typeName,
			isNative,
			isAsync,
			isEval,
			isConstructor,
			filename,
			lineNumber,
			columnNumber,
		} = frame;
		const parts: Array<string> = [];

		if (isAsync) {
			parts.push("await");
		}

		if (isEval) {
			parts.push("eval");
		}

		if (isConstructor) {
			parts.push("new");
		}

		let name = "<anonymous>";
		if (functionName !== undefined) {
			name = functionName;
		}
		if (methodName !== undefined) {
			name = methodName;
		}
		if (typeName !== undefined) {
			parts.push(`${typeName}.${name}`);
		} else {
			parts.push(name);
		}

		if (isNative) {
			parts.push("native");
		} else if (
			filename !== undefined &&
			lineNumber !== undefined &&
			columnNumber !== undefined
		) {
			parts.push(`(${filename}:${lineNumber}:${columnNumber})`);
		}

		if (!resolvedLocation) {
			parts.push("generated source location");
		}

		lines.push(`  at ${parts.join(" ")}`);
	}

	return lines.join("\n");
}

function noNull<T>(val: null | T): undefined | T {
	if (val === null) {
		return undefined;
	} else {
		return val;
	}
}

function addErrorFrames(
	err: ErrorWithFrames,
	frames: Array<NodeJS.CallSite>,
): void {
	if (err[ERROR_FRAMES_PROP]) {
		return;
	}

	let builtFrames = frames.map((frameApi): ErrorFrame => {
		const filename = frameApi.getFileName();
		const lineNumber = frameApi.getLineNumber();
		const columnNumber = frameApi.getColumnNumber();

		const frame: ErrorFrame = {
			typeName: noNull(frameApi.getTypeName()),
			functionName: noNull(frameApi.getFunctionName()),
			methodName: noNull(frameApi.getMethodName()),
			isTopLevel: frameApi.isToplevel(),
			isEval: frameApi.isEval(),
			isNative: frameApi.isNative(),
			isConstructor: frameApi.isConstructor(),
			// TODO frameApi.isAsync
			isAsync: false,
			resolvedLocation: true,
			filename: noNull(filename),
			lineNumber: lineNumber == null ? undefined : ob1Coerce1(lineNumber),
			// Rome expects 0-indexed columns, V8 provides 1-indexed
			columnNumber: columnNumber == null
				? undefined
				: ob1Coerce1To0(columnNumber),
		};

		if (
			frame.filename !== undefined &&
			frame.lineNumber !== undefined &&
			frame.columnNumber !== undefined
		) {
			const {found, line, column, source, name} = sourceMaps.assertApproxOriginalPositionFor(
				frame.filename,
				frame.lineNumber,
				frame.columnNumber,
			);

			return {
				...frame,
				functionName: frame.functionName ?? name,
				methodName: frame.methodName ?? name,
				resolvedLocation: found,
				lineNumber: line,
				columnNumber: column,
				filename: source,
			};
		} else {
			return frame;
		}
	});

	setErrorFrames(err, builtFrames);
}

const sourceMaps = new SourceMapConsumerCollection();
export default sourceMaps;
