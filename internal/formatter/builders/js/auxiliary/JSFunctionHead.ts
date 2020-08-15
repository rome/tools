/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {JSFunctionHead} from "@internal/ast";
import {Builder, Token, concat, group, space} from "@internal/formatter";
import {printBindingPatternParams} from "../utils";

export default function JSFunctionHead(
	builder: Builder,
	node: JSFunctionHead,
): Token {
	const tokens: Array<Token> = [];

	if (builder.options.typeAnnotations && node.typeParameters) {
		tokens.push(builder.tokenize(node.typeParameters, node));
	}

	const printedParameters = printBindingPatternParams(
		builder,
		node,
		node.thisType ? [node.thisType, ...node.params] : node.params,
		node.rest,
	);

	let printedReturnType: Token = "";
	if (builder.options.typeAnnotations) {
		if (node.returnType) {
			const tokens: Array<Token> = [":"];
			tokens.push(space, builder.tokenize(node.returnType, node));
			printedReturnType = concat(tokens);
		}
	}

	tokens.push(group(concat([printedParameters, printedReturnType])));

	return concat(tokens);
}
