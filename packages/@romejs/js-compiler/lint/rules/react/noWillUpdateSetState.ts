import {Path, TransformExitResult} from "@romejs/js-compiler";
import {descriptions} from "@romejs/diagnostics";
import {doesNodeMatchPattern} from "@romejs/js-ast-utils";

function inComponentWillUpdate(path: Path): boolean {
	return (
		path.findAncestry(({node}) =>
			node.type === "ClassMethod" &&
			node.key.type === "StaticPropertyKey" &&
			node.key.value.type === "Identifier" &&
			(node.key.value.name === "componentWillUpdate" ||
			node.key.value.name === "UNSAFE_componentWillUpdate")
		) !== undefined
	);
}

export default {
	name: "noWillUpdateSetState",
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
