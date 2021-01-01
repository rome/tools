/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {Builder, Token, concat, hardline, indent} from "@internal/formatter";
import {AnyNode, JSTemplateLiteral} from "@internal/ast";
import {getIndentRegex} from "@internal/string-utils";

export default function JSTemplateLiteral(
	builder: Builder,
	node: JSTemplateLiteral,
	parent: AnyNode,
): Token {
	const isIndent =
		parent.type === "JSTaggedTemplateExpression" &&
		parent.tag.type === "JSReferenceIdentifier" &&
		parent.tag.name === "dedent";
	const tokens: Token[] = [];
	const quasis = node.quasis;

	let indentRegex: RegExp;
	if (isIndent) {
		indentRegex = getIndentRegex(quasis.map((elem) => elem.raw).join(""));
	}

	for (let i = 0; i < quasis.length; i++) {
		const isFirst = i === 0;
		const isLast = i === quasis.length - 1;

		if (!isFirst) {
			tokens.push("}");
		}

		const elem = quasis[i];
		let {raw} = elem;

		// Remove common indentation from all lines and push them as individual tokens
		if (isIndent) {
			// We will always output a trailing and leading newline
			if (isFirst) {
				raw = raw.trimLeft();
			}
			if (isLast) {
				raw = raw.trimRight();
			}

			const lines = raw.split("\n");
			for (let i = 0; i < lines.length; i++) {
				let line = lines[i];

				// Remove indent
				line = line.replace(indentRegex!, "");
				tokens.push(line);

				if (i < lines.length - 1) {
					tokens.push(hardline);
				}
			}
		} else {
			tokens.push(elem.raw);
		}

		if (!isLast) {
			tokens.push("${");
		}

		if (i + 1 < quasis.length) {
			tokens.push(builder.tokenize(node.expressions[i], node));
		}
	}

	let inner = concat(tokens);

	if (isIndent) {
		inner = concat([indent(inner, true), hardline]);
	}

	return concat(["`", inner, "`"]);
}
