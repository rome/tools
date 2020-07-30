/**
 * Portions Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {ConstJSProgramSyntax, ConstJSSourceType} from "@internal/ast";
import {ParserOptionsWithRequiredPath} from "@internal/parser-core";

type UserOptionsBase = {
	syntax?: Array<ConstJSProgramSyntax>;
	sourceType?: ConstJSSourceType;
	tokens?: boolean;
	allowReturnOutsideFunction?: boolean;
	manifestPath?: undefined | string;
};

export type JSParserUserOptions = ParserOptionsWithRequiredPath &
	UserOptionsBase;

export type JSParserOptions = ParserOptionsWithRequiredPath &
	Required<UserOptionsBase>;

const DEFAULT_USER_OPTIONS: Required<UserOptionsBase> = {
	// I want to kill this option very badly
	allowReturnOutsideFunction: false,
	// Source type ("template", "script" or "module") for different semantics
	sourceType: "script",
	// Whether we should be tracking tokens when parsing this file
	// NOTE: This is memory-intensive
	tokens: false,
	syntax: [],
	manifestPath: "package.json",
};

// Interpret and default an options object
export function normalizeOptions(opts: JSParserUserOptions): JSParserOptions {
	return {
		...DEFAULT_USER_OPTIONS,
		...opts,
	};
}
