import {Path, Scope, createVisitor, signals} from "@internal/compiler";
import {descriptions} from "@internal/diagnostics";
import {AnyNode} from "@internal/ast";
import {
	getJSXAttribute,
	hasJSXAttribute,
	isFunctionNode,
} from "@internal/js-ast-utils";
import {doesNodeMatchReactPattern} from "../../utils/react";

function getKeyValue(path: Path): string | undefined {
	let keyValue = undefined;
	let callExpression = undefined;

	// handle React.cloneElement and cloneElement
	if (path.node.type === "JSCallExpression") {
		callExpression = path.node;
	} else if (path.parent.type === "JSCallExpression") {
		callExpression = path.parent;
	}
	if (
		callExpression?.type === "JSCallExpression" &&
		callExpression.arguments.length > 1
	) {
		const obj = callExpression.arguments[1];
		if (obj.type === "JSObjectExpression") {
			for (const prop of obj.properties) {
				if (
					prop.type === "JSObjectProperty" &&
					prop.key.type === "JSStaticPropertyKey" &&
					prop.key.value.type === "JSIdentifier" &&
					prop.key.value.name === "key"
				) {
					if (prop.value.type === "JSReferenceIdentifier") {
						keyValue = prop.value.name;
					}
				}
			}
		}
	}
	return keyValue;
}

function getReactChildrenArrayMethod(path: Path, scope: Scope): Path | undefined {
	return path.findAncestry(({node}) => {
		if (
			node.type === "JSExpressionStatement" &&
			node.expression.type === "JSCallExpression"
		) {
			const expr = node.expression;
			// Children
			if (
				expr.callee.type === "JSMemberExpression" &&
				expr.callee.object.type === "JSReferenceIdentifier" &&
				doesNodeMatchReactPattern(expr.callee.object, scope, "Children") &&
				expr.callee.property.value.type === "JSIdentifier" &&
				(expr.callee.property.value.name === "map" ||
				expr.callee.property.value.name === "forEach")
			) {
				return true;
			}

			// React.Children
			if (
				expr.callee.type === "JSMemberExpression" &&
				expr.callee.object.type === "JSMemberExpression" &&
				doesNodeMatchReactPattern(expr.callee.object, scope, "React.Children") &&
				expr.callee.object.property.type === "JSStaticMemberProperty" &&
				expr.callee.object.property.value.type === "JSIdentifier" &&
				expr.callee.object.property.value.name === "Children" &&
				expr.callee.property.value.type === "JSIdentifier" &&
				(expr.callee.property.value.name === "map" ||
				expr.callee.property.value.name === "forEach")
			) {
				return true;
			}
		}
		return false;
	});
}

function hasArrayMethod(path: Path) {
	return path.findAncestry(({node}) => {
		if (
			node.type === "JSExpressionStatement" &&
			node.expression.type === "JSCallExpression" &&
			node.expression.callee.type === "JSMemberExpression" &&
			node.expression.callee.property.type === "JSStaticMemberProperty" &&
			node.expression.callee.property.value.type === "JSIdentifier"
		) {
			return /(map|forEach|filter|some|every|find|findIndex|reduce|reduceRight)/.test(
				node.expression.callee.property.value.name,
			);
		}
		return false;
	});
}

function hasArrayIndexKey(keyValue: string, node: AnyNode) {
	if (
		node.type === "JSExpressionStatement" &&
		node.expression.type === "JSCallExpression" &&
		node.expression.arguments.length > 0
	) {
		const lastArg = node.expression.arguments[node.expression.arguments.length -
		1];
		if (isFunctionNode(lastArg)) {
			node = lastArg;
		}
	}
	if (isFunctionNode(node) && node.head.params.length > 0) {
		const lastParam = node.head.params[node.head.params.length - 1];
		if (lastParam.type === "JSBindingIdentifier") {
			return lastParam.name === keyValue;
		}
	}

	return false;
}

export default createVisitor({
	name: "react/noArrayIndexKey",
	enter(path) {
		const {node, scope} = path;

		if (doesNodeMatchReactPattern(node, scope, "cloneElement")) {
			const memberExpressionPath = path.findAncestry((path) =>
				path.node.type === "JSCallExpression"
			);
			if (memberExpressionPath) {
				const keyValue = getKeyValue(memberExpressionPath);
				const reactChildrenArrayMethod = getReactChildrenArrayMethod(
					memberExpressionPath,
					scope,
				);
				const arrayMethod = hasArrayMethod(memberExpressionPath);

				if (keyValue && (reactChildrenArrayMethod || arrayMethod)) {
					if (
						(reactChildrenArrayMethod &&
						hasArrayIndexKey(keyValue, reactChildrenArrayMethod.node)) ||
						(arrayMethod && hasArrayIndexKey(keyValue, arrayMethod.node))
					) {
						path.context.addNodeDiagnostic(
							node,
							descriptions.LINT.REACT_NO_ARRAY_INDEX_KEY,
						);
					}
				}
			}
		}

		if (doesNodeMatchReactPattern(node, scope, "React.cloneElement")) {
			const keyValue = getKeyValue(path);
			const reactChildrenArrayMethod = getReactChildrenArrayMethod(path, scope);
			const arrayMethod = hasArrayMethod(path);

			if (keyValue && (reactChildrenArrayMethod || arrayMethod)) {
				if (
					(reactChildrenArrayMethod &&
					hasArrayIndexKey(keyValue, reactChildrenArrayMethod.node)) ||
					(arrayMethod && hasArrayIndexKey(keyValue, arrayMethod.node))
				) {
					path.context.addNodeDiagnostic(
						node,
						descriptions.LINT.REACT_NO_ARRAY_INDEX_KEY,
					);
				}
			}
		}

		if (node.type === "JSXElement" && hasJSXAttribute(node, "key")) {
			let keyValue = undefined;
			const keyAttribute = getJSXAttribute(node, "key");
			if (keyAttribute) {
				if (
					keyAttribute.value?.type === "JSXExpressionContainer" &&
					keyAttribute.value.expression.type === "JSReferenceIdentifier" &&
					keyAttribute.value.expression.name
				) {
					keyValue = keyAttribute.value.expression.name;
				}
			}
			const functionExpression = path.findAncestry((path) => {
				return isFunctionNode(path.node);
			});

			if (keyValue && functionExpression) {
				if (
					hasArrayMethod(functionExpression) &&
					hasArrayIndexKey(keyValue, functionExpression.node)
				) {
					path.context.addNodeDiagnostic(
						node,
						descriptions.LINT.REACT_NO_ARRAY_INDEX_KEY,
					);
				}
			}
		}
		return signals.retain;
	},
});
