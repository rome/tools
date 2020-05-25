/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {Builder, Token} from "@romejs/formatter";

import {escapeJSString} from "@romejs/string-escape";
import {
	AnyNode,
	JSDirective,
	JSStringLiteral,
	TSStringLiteralTypeAnnotation,
} from "@romejs/ast";
import {escapeXHTMLEntities} from "@romejs/js-parser";

export default function JSStringLiteral(
	builder: Builder,
	node: JSDirective | JSStringLiteral | TSStringLiteralTypeAnnotation,
	parent: AnyNode,
): Token {
	// JSX Attribute strings have ridiculous alternate semantics, should probably be a distinct AST node
	const quotes =
		parent.type === "JSXAttribute" || node.value.includes('"') ? "'" : '"';

	const value =
		parent.type === "JSXAttribute"
			? escapeXHTMLEntities(node.value)
			: node.value;

	return escapeJSString(value, {quote: quotes});
}
