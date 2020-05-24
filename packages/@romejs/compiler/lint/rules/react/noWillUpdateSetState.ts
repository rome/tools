import {Path, TransformExitResult} from "@romejs/compiler";
import {descriptions} from "@romejs/diagnostics";
import {doesNodeMatchPattern, isConditional} from "@romejs/js-ast-utils";

function inComponentWillUpdate(path: Path): boolean {
	const func = path.findAncestry(({node}) => isConditional(node)) !== undefined;
	return (
		!func &&
		path.findAncestry(({node}) =>
			node.type === "JSClassMethod" &&
			node.key.type === "JSStaticPropertyKey" &&
			node.key.value.type === "JSIdentifier" &&
			(node.key.value.name === "componentWillUpdate" ||
			node.key.value.name === "UNSAFE_componentWillUpdate")
		) !== undefined
	);
}

export default {
	name: "reactNoWillUpdateSetState",
	enter(path: Path): TransformExitResult {
		const {node} = path;

		if (
			doesNodeMatchPattern(node, "this.setState") &&
			inComponentWillUpdate(path)
		) {
			path.context.addNodeDiagnostic(
				node,
				descriptions.LINT.REACT_NO_WILL_UPDATE_SET_STATE,
			);
		}

		return node;
	},
};
