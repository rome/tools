/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import Parser, {ParserInterface, ParserOptions} from "./Parser";
import {Reporter} from "@romefrontend/cli-reporter";

export {FlagValue} from "./Parser";
export {ParserInterface as FlagParser};

export function parseCLIFlags<T>(
	reporter: Reporter,
	args: Array<string>,
	opts: ParserOptions<T>,
): ParserInterface<T> {
	const parser = new Parser(reporter, opts, args);
	return parser.getInterface();
}

export function parseCLIFlagsFromProcess<T>(
	opts: ParserOptions<T>,
): ParserInterface<T> {
	return parseCLIFlags(
		Reporter.fromProcess(),
		process.argv.slice(2),
		{
			...opts,
			programName: opts.programName === undefined
				? process.argv[1]
				: opts.programName,
		},
	);
}

export * from "./serializeCLIFlags";
