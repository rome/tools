import {Path, Scope, TransformExitResult} from "@romefrontend/compiler";
import {descriptions} from "@romefrontend/diagnostics";
import {hasJSXAttribute, isFunctionNode} from "@romefrontend/js-ast-utils";
import {AnyNode, JSXElement, JSXFragment} from "@romefrontend/ast";
import {REDUCE_REMOVE} from "@romefrontend/compiler/constants";
import {doesNodeMatchReactPattern} from "../../utils/react";

function isChildOfHtmlElement(path: Path): boolean {
	const parentNode = path.parent;
	return (
		parentNode.type === "JSXElement" &&
		parentNode.name.type === "JSXIdentifier" &&
		/^[a-z]+$/.test(parentNode.name.name)
	);
}

function isFragment(node: JSXFragment | JSXElement, scope: Scope): boolean {
	return (
		node.type === "JSXFragment" ||
		(node.type === "JSXElement" &&
		(doesNodeMatchReactPattern(node.name, scope, "Fragment") ||
		doesNodeMatchReactPattern(node.name, scope, "React.Fragment")))
	);
}

function hasLessThanTwoChildren(node: JSXFragment | JSXElement): boolean {
	return node.children.length < 2;
}

function getChildrenNode(
	node: JSXFragment | JSXElement,
): AnyNode | typeof REDUCE_REMOVE | Array<AnyNode> {
	if (node.children.length === 0) {
		return REDUCE_REMOVE;
	}
	if (node.children.length === 1) {
		return node.children[0];
	}
	return node.children;
}

export default {
	name: "noUselessFragment",
	enter(path: Path): TransformExitResult {
		const {node, context, scope} = path;

		if (node.type !== "JSXFragment" && node.type !== "JSXElement") {
			return node;
		}

		if (
			path.parent.type !== "JSReturnStatement" &&
			path.parent.type !== "JSVariableDeclarator" &&
			!isFunctionNode(path.parent) &&
			isFragment(node, scope) &&
			!(node.type === "JSXElement" && hasJSXAttribute(node, "key")) &&
			(hasLessThanTwoChildren(node) || isChildOfHtmlElement(path))
		) {
			return context.addFixableDiagnostic(
				{
					old: node,
					fixed: getChildrenNode(node),
				},
				descriptions.LINT.REACT_NO_USELESS_FRAGMENT,
			);
		}

		return node;
	},
};
