/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {Builder, Token} from "@internal/formatter";

import {AnyNode, JSRegExpCharacter} from "@internal/ast";
import {escapeJSString} from "@internal/string-escape";

export default function JSRegExpCharacter(
	builder: Builder,
	node: JSRegExpCharacter,
	parent: AnyNode,
): Token {
	const isInCharSet = parent.type === "JSRegExpCharSet";
	if (isInCharSet) {
		switch (node.value) {
			case "$":
			case "^":
			case ".":
			case "?":
			case "{":
			case "}":
			case "+":
			case "*":
			case "[":
			case "(":
			case ")":
			case "|":
				return node.value;

			case "-":
				return "\\-";
		}
	}

	switch (node.value) {
		case "\t":
			return "\\t";

		case "\n":
			return "\\n";

		case "\r":
			return "\\r";

		case "\x0b":
			return "\\v";

		case "\f":
			return "\\f";

		case "\b":
			return "\\b";

		case "/":
		case "\\":
		case "$":
		case "^":
		case ".":
		case "?":
		case "{":
		case "}":
		case "+":
		case "*":
		case "[":
		case "]":
		case "(":
		case ")":
		case "|":
			return `\\${node.value}`;

		default:
			return escapeJSString(node.value, {json: true, unicodeOnly: true});
	}
}
