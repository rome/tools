/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import Builder from "../../Builder";
import {Token} from "../../tokens";
import {escapeString} from "@romejs/string-escape";
import {
	AnyNode,
	Directive,
	StringLiteral,
	TSStringLiteralTypeAnnotation,
} from "@romejs/js-ast";
import {escapeXHTMLEntities} from "@romejs/js-parser";

export default function StringLiteral(
	builder: Builder,
	node: Directive | StringLiteral | TSStringLiteralTypeAnnotation,
	parent: AnyNode,
): Token {
	// JSX Attribute strings have ridiculous alternate semantics, should probably be a distinct AST node
	const quotes =
		parent.type === "JSXAttribute" || node.value.includes('"') ? "'" : '"';

	const value =
		parent.type === "JSXAttribute" ? escapeXHTMLEntities(node.value) : node.value;

	return escapeString(value, {quote: quotes});
}
