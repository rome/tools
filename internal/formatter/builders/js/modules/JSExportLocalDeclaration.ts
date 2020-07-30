/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {
	JSExportDefaultDeclaration,
	JSExportLocalDeclaration,
} from "@internal/ast";
import {isDeclaration} from "@internal/js-ast-utils";
import {
	Builder,
	Token,
	concat,
	group,
	hardline,
	ifBreak,
	indent,
	softline,
	space,
} from "@internal/formatter";

import {hasInnerComments} from "../../comments";
import {printCommaList} from "../utils";

export default function JSExportLocalDeclaration(
	builder: Builder,
	node: JSExportLocalDeclaration,
): Token {
	if (node.exportKind === "type" && !builder.options.typeAnnotations) {
		return "";
	}

	return concat(["export", space, printExportDeclaration(builder, node)]);
}

export function printExportDeclaration(
	builder: Builder,
	node: JSExportDefaultDeclaration | JSExportLocalDeclaration,
): Token {
	if (node.declaration) {
		const tokens = [builder.tokenize(node.declaration, node)];
		if (!isDeclaration(node.declaration)) {
			tokens.push(";");
		}
		return concat(tokens);
	} else {
		if (node.type !== "JSExportLocalDeclaration") {
			throw new Error("Expected JSExportLocalDeclaration");
		}

		const {specifiers} = node;
		if (specifiers === undefined) {
			throw new Error("Expected specifiers since there was no declaration");
		}

		const tokens: Array<Token> = [];

		if (node.exportKind === "type") {
			tokens.push("type", space);
		}

		if (specifiers.length === 0) {
			if (hasInnerComments(node)) {
				tokens.push(
					concat(["{", builder.tokenizeInnerComments(node, true), hardline, "}"]),
				);
			} else {
				tokens.push("{}");
			}
		} else {
			tokens.push(
				group(
					concat([
						"{",
						indent(
							concat([softline, printCommaList(builder, specifiers, node)]),
						),
						ifBreak(","),
						softline,
						"}",
					]),
				),
			);
		}

		tokens.push(";");

		return concat(tokens);
	}
}
