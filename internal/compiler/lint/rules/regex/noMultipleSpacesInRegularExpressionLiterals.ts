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
} from "@internal/ast";
import {
	CompilerPath,
	ExitSignal,
	createLintVisitor,
	signals,
} from "@internal/compiler";
import {descriptions} from "@internal/diagnostics";

function isSpaceChar(
	node: undefined | AnyJSRegExpBodyItem,
): node is JSRegExpCharacter {
	return node?.type === "JSRegExpCharacter" && node.value === " ";
}

function checkRegex(path: CompilerPath, node: JSRegExpSubExpression): ExitSignal {
	const newBody: AnyJSRegExpBodyItem[] = [];
	const diagnosticTargets: AnyJSRegExpBodyItem[] = [];

	let index = 0;
	while (index < node.body.length) {
		const item = node.body[index];

		// Push the item unchanged if it's not the start of consecutive spaces
		if (!(isSpaceChar(item) && isSpaceChar(node.body[index + 1]))) {
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
		return path.addFixableDiagnostic(
			{
				target: diagnosticTargets,
				fixed: signals.replace(newRegex),
			},
			descriptions.LINT.REGEX_NO_MULTIPLE_SPACES_IN_REGEX_LITERAL(
				diagnosticTargets.length,
			),
		);
	} else {
		return signals.retain;
	}
}

export default createLintVisitor({
	name: "regex/noMultipleSpacesInRegularExpressionLiterals",
	enter(path) {
		const {node} = path;

		if (node.type === "JSRegExpSubExpression") {
			return checkRegex(path, node);
		}

		return signals.retain;
	},
});
