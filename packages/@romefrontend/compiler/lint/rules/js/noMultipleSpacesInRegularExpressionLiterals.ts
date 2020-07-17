/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {
	AnyJSRegExpBodyItem,
	JSRegExpCharacter,
	JSRegExpQuantified,
	JSRegExpSubExpression,
	jsRegExpQuantified,
} from "@romefrontend/ast";
import {
	CompilerContext,
	Path,
	TransformExitResult,
} from "@romefrontend/compiler";
import {descriptions} from "@romefrontend/diagnostics";

function isSpaceChar(
	node: undefined | AnyJSRegExpBodyItem,
): node is JSRegExpCharacter {
	return (
		node !== undefined &&
		node.type === "JSRegExpCharacter" &&
		node.value === " "
	);
}

function checkRegex(
	node: JSRegExpSubExpression,
	context: CompilerContext,
): TransformExitResult {
	const newBody: Array<AnyJSRegExpBodyItem> = [];
	const diagnosticTargets: Array<AnyJSRegExpBodyItem> = [];

	let index = 0;
	while (index < node.body.length) {
		const item = node.body[index];

		// Push the item unchanged if it's not the start of consecutive spaces
		if (!isSpaceChar(item) || !isSpaceChar(node.body[index + 1])) {
			newBody.push(item);
			index++;
			continue;
		}

		// Count the number of consecutive space chars
		let spaceCount = 0;
		while (isSpaceChar(node.body[index])) {
			diagnosticTargets.push(node.body[index]);
			spaceCount++;
			index++;
		}

		// Push a new body item that represents all the consecutive spaces
		const quantifiedSpace: JSRegExpQuantified = jsRegExpQuantified.create({
			min: spaceCount,
			max: spaceCount,
			target: item,
		});
		newBody.push(quantifiedSpace);
	}

	if (diagnosticTargets.length > 0) {
		const newRegex: JSRegExpSubExpression = {
			...node,
			body: newBody,
		};
		return context.addFixableDiagnostic(
			{
				target: diagnosticTargets,
				old: node,
				fixed: newRegex,
			},
			descriptions.LINT.JS_NO_MULTIPLE_SPACES_IN_REGEX_LITERAL(
				diagnosticTargets.length,
			),
		);
	} else {
		return node;
	}
}

export default {
	name: "js/noMultipleSpacesInRegularExpressionLiterals",
	enter(path: Path): TransformExitResult {
		const {context, node} = path;

		if (node.type === "JSRegExpSubExpression") {
			return checkRegex(node, context);
		}

		return node;
	},
};
