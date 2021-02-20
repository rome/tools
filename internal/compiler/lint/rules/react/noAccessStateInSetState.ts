import {createVisitor, signals} from "@internal/compiler";
import {descriptions} from "@internal/diagnostics";
import {doesNodeMatchPattern} from "@internal/js-ast-utils";
import {insideClassComponent} from "../../utils/react";

export default createVisitor({
	name: "react/noAccessStateInSetState",
	enter(path) {
		const {node} = path;
		if (
			node.type === "JSCallExpression" &&
			node.arguments.length > 0 &&
			node.arguments[0].type === "JSObjectExpression" &&
			doesNodeMatchPattern(node.callee, "this.setState")
		) {
			const hasThisState = node.arguments[0].properties.some((arg) => {
				if (arg.type === "JSObjectProperty") {
					if (
						arg.value.type === "JSMemberExpression" &&
						doesNodeMatchPattern(arg.value.object, "this.state")
					) {
						return true;
					}
					if (
						(arg.value.type === "JSUnaryExpression" &&
						arg.value.argument.type === "JSMemberExpression" &&
						doesNodeMatchPattern(arg.value.argument.object, "this.state")) ||
						(arg.value.type === "JSUnaryExpression" &&
						arg.value.argument.type === "JSUnaryExpression" &&
						arg.value.argument.argument.type === "JSMemberExpression" &&
						doesNodeMatchPattern(
							arg.value.argument.argument.object,
							"this.state",
						))
					) {
						return true;
					}
					if (arg.value.type === "JSBinaryExpression") {
						if (
							arg.value.left.type === "JSMemberExpression" &&
							doesNodeMatchPattern(arg.value.left.object, "this.state")
						) {
							return true;
						}
						if (
							arg.value.right.type === "JSMemberExpression" &&
							doesNodeMatchPattern(arg.value.right.object, "this.state")
						) {
							return true;
						}
					}
				}
				return false;
			});
			if (hasThisState && insideClassComponent(path)) {
				path.context.addNodeDiagnostic(
					node,
					descriptions.LINT.REACT_NO_ACCESS_STATE_IN_SET_STATE,
				);
			}
		}

		return signals.retain;
	},
});
