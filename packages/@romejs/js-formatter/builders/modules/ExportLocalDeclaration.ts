/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {ExportDefaultDeclaration, ExportLocalDeclaration} from "@romejs/js-ast";
import {isDeclaration} from "@romejs/js-ast-utils";
import Builder from "../../Builder";
import {
	Token,
	concat,
	group,
	hardline,
	ifBreak,
	indent,
	softline,
	space,
} from "../../tokens";
import {hasInnerComments} from "../comments";
import {printCommaList} from "../utils";

export default function ExportLocalDeclaration(
	builder: Builder,
	node: ExportLocalDeclaration,
): Token {
	if (node.exportKind === "type" && !builder.options.typeAnnotations) {
		return "";
	}

	return concat(["export", space, printExportDeclaration(builder, node)]);
}

export function printExportDeclaration(
	builder: Builder,
	node: ExportDefaultDeclaration | ExportLocalDeclaration,
): Token {
	if (node.declaration) {
		const tokens = [builder.tokenize(node.declaration, node)];
		if (!isDeclaration(node.declaration)) {
			tokens.push(";");
		}
		return concat(tokens);
	} else {
		if (node.type !== "ExportLocalDeclaration") {
			throw new Error("Expected ExportLocalDeclaration");
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
						indent(concat([softline, printCommaList(builder, specifiers, node)])),
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
