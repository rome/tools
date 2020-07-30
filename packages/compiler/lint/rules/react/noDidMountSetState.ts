import {Path, createVisitor, signals} from "@romefrontend/compiler";
import {descriptions} from "@romefrontend/diagnostics";
import {doesNodeMatchPattern, isConditional} from "@romefrontend/js-ast-utils";
import {insideClassComponent} from "../../utils/react";

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

export default createVisitor({
	name: "react/noDidMountSetState",
	enter(path) {
		const {node} = path;

		if (
			insideClassComponent(path) &&
			doesNodeMatchPattern(node, "this.setState") &&
			inComponentDidMount(path)
		) {
			path.context.addNodeDiagnostic(
				node,
				descriptions.LINT.REACT_NO_DID_MOUNT_SET_STATE,
			);
		}

		return signals.retain;
	},
});
