/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import Parser, {ParserInterface, ParserOptions} from "./Parser";
import {Reporter} from "@internal/cli-reporter";
import {OptionalProps} from "@internal/typescript-helpers";
import {CWD_PATH} from "@internal/path";

export {Examples, FlagValue} from "./Parser";
export {ParserInterface as FlagParser};

export function parseCLIFlags<T>(opts: ParserOptions<T>): ParserInterface<T> {
	const parser = new Parser(opts);
	return parser.getInterface();
}

export function parseCLIFlagsFromProcess<T>(
	opts: OptionalProps<
		ParserOptions<T>,
		"cwd" | "programName" | "args" | "reporter"
	>,
): ParserInterface<T> {
	let programName: string;
	let args: Array<string>;

	if (opts.args === undefined) {
		programName = process.argv[1];
		args = process.argv.slice(2);
	} else {
		programName = process.argv.join(" ");
		args = opts.args;
	}

	return parseCLIFlags({
		...opts,
		reporter: opts.reporter ?? Reporter.fromProcess(),
		args,
		cwd: opts.cwd ?? CWD_PATH,
		programName: opts.programName ?? programName,
	});
}

export * from "./serializeCLIFlags";
