/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {
	ERROR_FRAMES_PROP,
	ErrorFrame,
	ErrorWithFrames,
	getErrorStructure,
	setErrorFrames,
} from "@internal/errors";
import {Path, createPath, createUIDPath} from "@internal/path";
import {OneIndexed} from "@internal/numbers";
import {SourceMapConsumerCollection} from "@internal/codec-source-map";
const module = require("module");

let inited: boolean = false;

export const errorSourceMaps = new SourceMapConsumerCollection();

function prepareStackTrace(err: Error, frames: NodeJS.CallSite[]) {
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
	const lines: string[] = [];

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
			path,
			lineNumber,
			columnNumber,
		} = frame;
		const parts: string[] = [];

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
			path !== undefined &&
			lineNumber !== undefined &&
			columnNumber !== undefined
		) {
			parts.push(`(${path.format()}:${lineNumber}:${columnNumber})`);
		}

		if (!resolvedLocation) {
			parts.push("generated source location");
		}

		lines.push(`  at ${parts.join(" ")}`);
	}

	return lines.join("\n");
}

function cleanIdentifier(name: null | string): undefined | string {
	if (name == null) {
		return undefined;
	} else if (name.startsWith("___R$")) {
		// We produce these really long identifiers in the bundler, until we get better source map support
		// for these, implicitly handle them
		const parts = name.split("$");
		const part = parts.pop()!;
		if (part === "default") {
			return parts.pop()! ?? part;
		} else {
			return part;
		}
	} else {
		return name;
	}
}

function noNull<T>(val: null | T): undefined | T {
	if (val === null) {
		return undefined;
	} else {
		return val;
	}
}

function toNodeFilename(filename: string): undefined | string {
	if (filename.startsWith("node:")) {
		// Node v15 internal module paths start with node: in stack traces
		// https://github.com/nodejs/node/pull/35498
		return filename;
	}

	// Simulate it for Node v14
	if (filename.startsWith("internal/")) {
		return `node:${filename}`;
	}
	if (module.builtinModules.includes(filename.replace(/\.js$/g, ""))) {
		return `node:${filename}`;
	}

	return undefined;
}

function addErrorFrames(err: ErrorWithFrames, frames: NodeJS.CallSite[]): void {
	if (err[ERROR_FRAMES_PROP]) {
		return;
	}

	let builtFrames = frames.map((frameApi): ErrorFrame => {
		const filename = noNull(frameApi.getFileName());
		const lineNumber = frameApi.getLineNumber();
		const columnNumber = frameApi.getColumnNumber();

		let path: undefined | Path;
		if (filename !== undefined) {
			let nodeFilename = toNodeFilename(filename);
			if (nodeFilename === undefined) {
				path = createPath(filename);
			} else {
				path = createUIDPath(nodeFilename);
			}
		}

		let frame: ErrorFrame = {
			typeName: cleanIdentifier(frameApi.getTypeName()),
			functionName: cleanIdentifier(frameApi.getFunctionName()),
			methodName: cleanIdentifier(frameApi.getMethodName()),
			isTopLevel: frameApi.isToplevel(),
			isEval: frameApi.isEval(),
			isNative: frameApi.isNative(),
			isConstructor: frameApi.isConstructor(),
			// TODO frameApi.isAsync
			isAsync: false,
			resolvedLocation: true,
			path,
			lineNumber: lineNumber == null ? undefined : new OneIndexed(lineNumber),
			// Rome expects 0-indexed columns, V8 provides 1-indexed
			columnNumber: columnNumber == null
				? undefined
				: new OneIndexed(columnNumber).toZeroIndexed(),
		};

		if (
			frame.path !== undefined &&
			frame.lineNumber !== undefined &&
			frame.columnNumber !== undefined
		) {
			const {found, line, column, source, name} = errorSourceMaps.assertApproxOriginalPositionFor(
				frame.path,
				frame.lineNumber,
				frame.columnNumber,
			);
			if (found) {
				return {
					...frame,
					functionName: frame.functionName ?? name,
					methodName: frame.methodName ?? name,
					resolvedLocation: true,
					lineNumber: line,
					columnNumber: column,
					path: source,
				};
			}
		}

		return frame;
	});

	setErrorFrames(err, builtFrames);
}
