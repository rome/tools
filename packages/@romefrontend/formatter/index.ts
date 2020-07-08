/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {AnyComment, AnyNode, MOCK_PARENT} from "@romefrontend/ast";
import Builder from "./Builder";
import {PrinterOutput, printTokenToString} from "./Printer";

export {Builder};
export {BuilderMethod} from "./Builder";
export * from "./tokens";

export type FormatterOptions = {
	typeAnnotations?: boolean;
	format?: "pretty" | "compact";
	indent?: number;
	comments?: Array<AnyComment>;
	sourceMaps?: boolean;
	sourceText?: string;
	allowInterpreterDirective?: boolean;
};

export function formatAST(
	ast: AnyNode,
	{
		format = "pretty",
		typeAnnotations = true,
		sourceMaps = false,
		comments,
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
		ast.type === "JSRoot" ? ast.comments : comments,
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
