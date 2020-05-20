import {Path, TransformExitResult} from "@romejs/js-compiler";
import {descriptions} from "@romejs/diagnostics";
import {doesNodeMatchPattern, isConditional} from "@romejs/js-ast-utils";

function inComponentDidMount(path: Path): boolean {
	const func = path.findAncestry(({node}) => isConditional(node)) !== undefined;
	return (
		!func &&
		!!path.findAncestry(({node}) =>
			node.type === "JSClassMethod" &&
			node.key.type === "JSStaticPropertyKey" &&
			node.key.value.type === "JSIdentifier" &&
			node.key.value.name === "componentDidMount"
		)
	);
}

export default {
	name: "noDidMountSetState",
	enter(path: Path): TransformExitResult {
		const {node} = path;

		if (doesNodeMatchPattern(node, "this.setState") && inComponentDidMount(path)) {
			path.context.addNodeDiagnostic(
				node,
				descriptions.LINT.REACT_NO_DID_MOUNT_SET_STATE,
			);
		}

		return node;
	},
};
