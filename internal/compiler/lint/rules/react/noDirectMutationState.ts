import {Path, createVisitor, signals} from "@internal/compiler";
import {AnyNode} from "@internal/ast";
import {descriptions} from "@internal/diagnostics";
import {doesNodeMatchPattern} from "@internal/js-ast-utils";
import {insideClassComponent} from "../../utils/react";

// Check if this.state mutation was in the constructor
function isMutationInConstructor(path: Path): boolean {
	// Find the first instance of a constructor or a call method
	const ancestor = path.findAncestry(({node}) =>
		(node.type === "JSClassMethod" &&
		node.key.type === "JSStaticPropertyKey" &&
		node.key.value.type === "JSIdentifier" &&
		node.key.value.name === "constructor") ||
		node.type === "JSCallExpression"
	);

	// If undefined, or a call expression, then its not in a constructor
	return ancestor !== undefined && ancestor.node.type !== "JSCallExpression";
}

// Checks if the node contains this.state that is mutated (binary and unary expr)
function isStateMutated(node: AnyNode): boolean {
	// Check if node is a binary expression where this.state is the left side
	if (
		node.type === "JSAssignmentExpression" &&
		(doesNodeMatchPattern(node.left, "this.state") ||
		doesNodeMatchPattern(node.left, "this.state.**"))
	) {
		return true;
	}

	// Check if the node is an update expression (++ and --)
	if (
		node.type === "JSUpdateExpression" &&
		(doesNodeMatchPattern(node.argument, "this.state") ||
		doesNodeMatchPattern(node.argument, "this.state.**"))
	) {
		return true;
	}

	// Check if the delete operator is used
	if (
		node.type === "JSUnaryExpression" &&
		node.operator === "delete" &&
		(doesNodeMatchPattern(node.argument, "this.state") ||
		doesNodeMatchPattern(node.argument, "this.state.**"))
	) {
		return true;
	}

	return false;
}

export default createVisitor({
	name: "react/noDirectMutationState",
	enter(path) {
		const {node} = path;

		// If the state is mutated anywhere except in a constructor, show message
		if (
			isStateMutated(node) &&
			insideClassComponent(path) &&
			!isMutationInConstructor(path)
		) {
			path.context.addNodeDiagnostic(
				node,
				descriptions.LINT.REACT_NO_DIRECT_MUTATION_STATE,
			);
		}

		return signals.retain;
	},
});
