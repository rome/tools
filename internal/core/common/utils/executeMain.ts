/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {UnknownObject} from "@internal/typescript-helpers";
import {SourceMapConsumer} from "@internal/codec-source-map";
import {sourceMapManager} from "@internal/v8";
import internalModule = require("module");

import vm = require("vm");

import {
	Diagnostic,
	descriptions,
	truncateSourceText,
} from "@internal/diagnostics";
import {AbsoluteFilePath} from "@internal/path";
import {Position} from "@internal/parser-core";
import {ob1Coerce1, ob1Number0} from "@internal/ob1";

type ExecuteMainOptions = {
	path: AbsoluteFilePath;
	code: string;
	args?: Array<string>;
	sourceMap?: SourceMapConsumer;
	globals?: UnknownObject;
};

export default async function executeMain(
	opts: ExecuteMainOptions,
): Promise<{
	syntaxError: undefined | Diagnostic;
	exitCode: undefined | number;
}> {
	const {path, code, sourceMap, globals = {}, args = []} = opts;

	const filename = path.join();

	// Create global context
	const sandbox: UnknownObject = {
		// TODO Find a more reliable way to do this
		TextEncoder,
		TextDecoder,
		Buffer,
		clearImmediate,
		clearInterval,
		clearTimeout,
		setImmediate,
		setInterval,
		setTimeout,
		require: internalModule.createRequire
			? internalModule.createRequire(filename)
			: internalModule.createRequireFromPath(filename),
		console,
		__dirname: path.getParent().join(),
		__filename: filename,
		...globals,
		process: Object.setPrototypeOf(
			{
				argv: ["rome", "run", filename, "--", ...args],
				// @ts-ignore
				...globals.process,
			},
			process,
		),
	};
	sandbox.global = sandbox;
	const context = vm.createContext(sandbox);

	// Here we do some gymnastics to catch a syntax error to correctly identify it as being our fault
	let script;
	try {
		script = new vm.Script(
			code,
			{
				filename,
				displayErrors: true,
			},
		);
	} catch (err) {
		if (err instanceof SyntaxError && err.stack !== undefined) {
			const lineMatch = err.stack.match(/^(.*?):(\d+)/);
			if (lineMatch == null) {
				throw err;
			}

			const line = Number(lineMatch[2]);

			const pos: Position = {
				column: ob1Number0,
				line: ob1Coerce1(line),
			};

			const syntaxError: Diagnostic = {
				description: descriptions.V8.SYNTAX_ERROR(err.message),
				location: {
					start: pos,
					end: pos,
					filename,
					sourceText: truncateSourceText(code, pos, pos),
				},
				tags: {
					internal: true,
				},
			};
			return {syntaxError, exitCode: undefined};
		}

		throw err;
	}

	// Execute the script if there was no syntax error
	if (sourceMap !== undefined) {
		sourceMapManager.add(filename, sourceMap);
	}
	const res = await script.runInContext(context);

	let exitCode: undefined | number;

	if (typeof res === "object" && res != null && typeof res.main === "function") {
		let code = await Promise.resolve(res.main(args));
		if (typeof code === "number") {
			exitCode = code;
		} else {
			exitCode = 0;
		}
	}

	return {syntaxError: undefined, exitCode};
}
