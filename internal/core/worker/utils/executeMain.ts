/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {UnknownObject} from "@internal/typescript-helpers";
import vm = require("vm");
import {OneIndexed, ZeroIndexed} from "@internal/numbers";
import {
	Diagnostic,
	descriptions,
	truncateSourceText,
} from "@internal/diagnostics";
import {AbsoluteFilePath} from "@internal/path";
import {Position} from "@internal/parser-core";
import Worker from "../Worker";
import {getRequire} from "../../common/IntegrationLoader";

type ExecuteMainOptions = {
	commandName: string;
	path: AbsoluteFilePath;
	code: string;
	contextDirectory: AbsoluteFilePath;
	cwd: AbsoluteFilePath;
	args: string[];
	globals?: UnknownObject;
};

export function getExecuteMainFilename(path: AbsoluteFilePath): string {
	return `executeMain:${path.join()}`;
}

export default async function executeMain(
	worker: Worker,
	opts: ExecuteMainOptions,
): Promise<{
	syntaxError: undefined | Diagnostic;
	exitCode: undefined | number;
}> {
	const {commandName, path, code, contextDirectory, globals = {}, args} = opts;

	const filename = path.join();

	// TODO get cwd passed in
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
		require: getRequire(path),
		console,
		__dirname: contextDirectory.getUnique().join(),
		__filename: contextDirectory.append("file").join(),
		...globals,
		process: Object.setPrototypeOf(
			{
				argv: [`rome ${commandName}`, filename, "--", ...args],
				env: {
					...worker.env,
					ROME_PROCESS_TYPE: undefined,
					ROME_PROCESS_VERSION: undefined,
				},
				// @ts-expect-error
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
				filename: getExecuteMainFilename(path),
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
				column: new ZeroIndexed(),
				line: new OneIndexed(line),
			};

			const syntaxError: Diagnostic = {
				description: descriptions.V8.SYNTAX_ERROR(err.message),
				location: {
					start: pos,
					end: pos,
					path,
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
