import {Path, TransformExitResult} from "@romejs/compiler";
import {descriptions} from "@romejs/diagnostics";
import {doesNodeMatchReactPattern} from "../../utils/react";

export default {
	name: "reactNoRedundantShouldComponentUpdate",
	enter(path: Path): TransformExitResult {
		const {node, scope} = path;

		if (
			node.type === "JSClassDeclaration" &&
			!!node.meta.body.find((member) =>
				member.type === "JSClassMethod" &&
				member.key.type === "JSStaticPropertyKey" &&
				member.key.value.type === "JSIdentifier" &&
				member.key.value.name === "shouldComponentUpdate"
			) &&
			node.meta.superClass &&
			(doesNodeMatchReactPattern(
				node.meta.superClass,
				scope,
				"React.PureComponent",
			) ||
			doesNodeMatchReactPattern(node.meta.superClass, scope, "PureComponent"))
		) {
			path.context.addNodeDiagnostic(
				node,
				descriptions.LINT.REACT_NO_REDUNDANT_SHOULD_COMPONENT_UPDATE,
			);
		}

		return node;
	},
};
