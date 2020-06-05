import {Path, TransformExitResult} from "@romejs/compiler";
import {descriptions} from "@romejs/diagnostics";
import {
	doesNodeMatchPattern,
	hasJSXAttribute,
	isFunctionNode,
} from "@romejs/js-ast-utils";
import {AnyNode, JSXElement, JSXFragment} from "@romejs/ast";
import {REDUCE_REMOVE} from "@romejs/compiler/constants";

function isChildOfHtmlElement(path: Path): boolean {
	const parentNode = path.parent;
	return (
		parentNode.type === "JSXElement" &&
		parentNode.name.type === "JSXIdentifier" &&
		/^[a-z]+$/.test(parentNode.name.name)
	);
}

function isFragment(node: JSXFragment | JSXElement): boolean {
	return (
		node.type === "JSXFragment" ||
		(node.type === "JSXElement" &&
		(doesNodeMatchPattern(node.name, "Fragment") ||
		doesNodeMatchPattern(node.name, "React.Fragment")))
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
		const {node, context} = path;

		if (node.type !== "JSXFragment" && node.type !== "JSXElement") {
			return node;
		}

		if (
			path.parent.type !== "JSReturnStatement" &&
			path.parent.type !== "JSVariableDeclarator" &&
			!isFunctionNode(path.parent) &&
			isFragment(node) &&
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
