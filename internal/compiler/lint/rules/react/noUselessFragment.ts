import {
	CompilerPath,
	ExitSignal,
	Scope,
	createLintVisitor,
	signals,
} from "@internal/compiler";
import {descriptions} from "@internal/diagnostics";
import {hasJSXAttribute, isFunctionNode} from "@internal/js-ast-utils";
import {JSXElement, JSXFragment} from "@internal/ast";
import {doesNodeMatchReactPattern} from "../../utils/react";

function isChildOfHtmlElement(path: CompilerPath): boolean {
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

function getChildrenNode(node: JSXFragment | JSXElement): ExitSignal {
	if (node.children.length === 0) {
		return signals.remove;
	}

	if (node.children.length === 1) {
		return signals.replace(node.children[0]);
	}

	return signals.replace(node.children);
}

export default createLintVisitor({
	name: "react/noUselessFragment",
	enter(path) {
		const {node, scope} = path;

		if (node.type !== "JSXFragment" && node.type !== "JSXElement") {
			return signals.retain;
		}

		if (
			path.parent.type !== "JSReturnStatement" &&
			path.parent.type !== "JSVariableDeclarator" &&
			path.parent.type !== "JSConditionalExpression" &&
			!isFunctionNode(path.parent) &&
			isFragment(node, scope) &&
			!(node.type === "JSXElement" && hasJSXAttribute(node, "key")) &&
			(hasLessThanTwoChildren(node) || isChildOfHtmlElement(path))
		) {
			return path.addFixableDiagnostic(
				{
					fixed: getChildrenNode(node),
				},
				descriptions.LINT.REACT_NO_USELESS_FRAGMENT,
			);
		}

		return signals.retain;
	},
});
