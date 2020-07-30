/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {AnyNode} from "@internal/ast";
import {Scope} from "@internal/compiler";

export function hasPotentialSideEffects(
	node: undefined | AnyNode,
	scope: Scope,
): boolean {
	if (node === undefined) {
		return false;
	}

	switch (node.type) {
		case "JSExportLocalDeclaration":
			if (node.declaration === undefined) {
				return false;
			} else {
				return hasPotentialSideEffects(node.declaration, scope);
			}

		case "JSExportExternalDeclaration":
			return true;

		case "JSFunctionExpression":
		case "JSFunctionDeclaration":
			return false;

		case "JSClassDeclaration":
			return (
				node.meta.superClass !== undefined ||
				!hasPotentialSideEffects(node.meta.superClass, scope)
			);

		case "JSReferenceIdentifier":
			// Variables that aren't in scope and aren't registered globals could trigger a getter
			// Unlikely but let's aim for 100% correctness
			return (
				scope.getRootScope().isGlobal(node.name) || scope.hasBinding(node.name)
			);

		case "JSVariableDeclaration": {
			for (const declarator of node.declarations) {
				if (hasPotentialSideEffects(declarator, scope)) {
					return true;
				}
			}
			return false;
		}

		case "JSVariableDeclarator":
			return (
				hasPotentialSideEffects(node.id, scope) ||
				hasPotentialSideEffects(node.init, scope)
			);

		case "JSSpreadProperty":
		case "JSSpreadElement":
			return hasPotentialSideEffects(node.argument, scope);

		case "JSBindingAssignmentPattern":
			return hasPotentialSideEffects(node.right, scope);

		case "JSObjectExpression":
		case "JSBindingObjectPattern": {
			for (const prop of node.properties) {
				if (hasPotentialSideEffects(prop, scope)) {
					return true;
				}
			}
			return false;
		}

		case "JSStaticPropertyKey":
			return false;

		case "JSComputedPropertyKey":
			return hasPotentialSideEffects(node.value, scope);

		case "JSBindingObjectPatternProperty":
		case "JSObjectProperty":
			return (
				hasPotentialSideEffects(node.key, scope) ||
				hasPotentialSideEffects(node.value, scope)
			);

		case "JSBindingArrayPattern":
		case "JSArrayExpression": {
			for (const elem of node.elements) {
				if (hasPotentialSideEffects(elem, scope)) {
					return true;
				}
			}
			return false;
		}

		case "JSStringLiteral":
		case "JSNumericLiteral":
		case "JSBooleanLiteral":
		case "JSNullLiteral":
			return false;
	}

	return true;
}
