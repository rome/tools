/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import Builder from "../../Builder";
import {SwitchCase} from "@romejs/js-ast";
import {Token, concat, hardline, indent, space} from "../../tokens";

export default function SwitchCase(builder: Builder, node: SwitchCase): Token {
	const tokens: Array<Token> = [];

	if (node.test) {
		tokens.push("case", space, builder.tokenize(node.test, node), ":");
	} else {
		tokens.push("default", ":");
	}

	const {consequent} = node;
	if (consequent.length === 1 && consequent[0].type === "BlockStatement") {
		tokens.push(space);
		tokens.push(builder.tokenize(consequent[0], node));
	} else if (consequent.length > 0) {
		tokens.push(
			indent(
				concat([hardline, builder.tokenizeStatementList(consequent, node)]),
			),
		);
	}

	return concat(tokens);
}
