/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {AnyJSStatement, AnyNode} from "@internal/ast";
import {isDeclaration} from "./isDeclaration";

export function isStatement(node: undefined | AnyNode): node is AnyJSStatement {
	if (node === undefined) {
		return false;
	}

	if (isDeclaration(node)) {
		return true;
	}

	switch (node.type) {
		case "JSBlockStatement":
		case "JSBreakStatement":
		case "JSContinueStatement":
		case "JSDebuggerStatement":
		case "JSDoWhileStatement":
		case "JSEmptyStatement":
		case "JSExpressionStatement":
		case "JSForInStatement":
		case "JSForStatement":
		case "JSIfStatement":
		case "JSLabeledStatement":
		case "JSReturnStatement":
		case "JSSwitchStatement":
		case "JSThrowStatement":
		case "JSTryStatement":
		case "JSWhileStatement":
		case "JSWithStatement":
		case "JSForOfStatement": {
			const statement: AnyJSStatement = node;
			statement;
			return true;
		}

		default: {
			// Assert that all statements were handled
			const notStatement: Exclude<AnyNode, AnyJSStatement> = node;
			notStatement;
			return false;
		}
	}
}
