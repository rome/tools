/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {AnyNode, JSFunctionHead} from "@internal/ast";
import {Builder, Token, concat, group, space} from "@internal/formatter";
import {printBindingPatternParams} from "../utils";

export default function JSFunctionHead(
	builder: Builder,
	node: JSFunctionHead,
	parent: AnyNode,
): Token {
	const tokens: Token[] = [];

	if (builder.options.typeAnnotations && node.typeParameters) {
		const isTsx = /\.tsx$/.test(node.loc?.path.toString() ?? "");
		if (
			isTsx &&
			parent.type === "JSArrowFunctionExpression" &&
			node.typeParameters.params.length === 1 &&
			!node.typeParameters.params[0].constraint
		) {
			tokens.push(
				concat([
					"<",
					builder.tokenize(node.typeParameters.params[0], node),
					",",
					">",
				]),
			);
		} else {
			tokens.push(builder.tokenize(node.typeParameters, node));
		}
	}

	const printedParameters = printBindingPatternParams(
		builder,
		node,
		node.thisType && builder.options.typeAnnotations
			? [node.thisType, ...node.params]
			: node.params,
		node.rest,
	);

	let printedReturnType: Token = "";
	if (builder.options.typeAnnotations) {
		if (node.returnType) {
			const tokens: Token[] = [":"];
			tokens.push(space, builder.tokenize(node.returnType, node));
			printedReturnType = concat(tokens);
		}
	}

	tokens.push(group(concat([printedParameters, printedReturnType])));

	return concat(tokens);
}
