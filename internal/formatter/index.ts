/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {AnyNode, MOCK_PARENT} from "@internal/ast";
import Builder from "./Builder";
import {PrinterOutput, printTokenToString} from "./Printer";
import {isRoot} from "@internal/ast-utils";

export {Builder};
export {BuilderMethod} from "./Builder";
export * from "./tokens";

export type FormatterOptions = {
	typeAnnotations?: boolean;
	format?: "pretty" | "compact";
	indent?: number;
	sourceMaps?: boolean;
	allowInterpreterDirective?: boolean;
};

export function formatAST(
	ast: AnyNode,
	{
		format = "pretty",
		typeAnnotations = true,
		sourceMaps = false,
		indent = 0,
		allowInterpreterDirective = true,
	}: FormatterOptions = {},
): PrinterOutput {
	const builder = new Builder(
		{
			format,
			sourceMaps,
			typeAnnotations,
			allowInterpreterDirective,
		},
		isRoot(ast) ? ast.comments : [],
	);
	const token = builder.tokenize(ast, MOCK_PARENT);
	const formatted = printTokenToString(
		token,
		{
			printWidth: format === "pretty" ? 80 : Infinity,
			rootIndent: indent,
			tabWidth: 2,
		},
	);

	return formatted;
}
