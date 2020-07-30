/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {
	Builder,
	Token,
	concat,
	group,
	indent,
	lineOrSpace,
	space,
} from "@internal/formatter";

import {JSClassHead} from "@internal/ast";
import {printCommaList} from "../utils";

export default function JSClassHead(builder: Builder, node: JSClassHead): Token {
	const tokens: Array<Token> = [];
	const tokenGroups: Array<Token> = [];

	tokens.push(builder.tokenize(node.typeParameters, node));

	if (node.superClass) {
		tokenGroups.push(
			group(
				concat([
					lineOrSpace,
					"extends",
					space,
					builder.tokenize(node.superClass, node),
					builder.tokenize(node.superTypeParameters, node),
				]),
			),
		);
	}

	if (
		builder.options.typeAnnotations &&
		node.implements &&
		node.implements.length > 0
	) {
		tokenGroups.push(
			lineOrSpace,
			"implements",
			group(
				indent(
					concat([lineOrSpace, printCommaList(builder, node.implements, node)]),
				),
			),
		);
	}

	if (tokenGroups.length > 0) {
		tokens.push(group(indent(concat(tokenGroups))));
	}

	return concat(tokens);
}
