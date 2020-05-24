import {Path, TransformExitResult} from "@romejs/compiler";
import {descriptions} from "@romejs/diagnostics";
import {doesNodeMatchPattern} from "@romejs/js-ast-utils";

export default {
	name: "reactNoRedundantShouldComponentUpdate",
	enter(path: Path): TransformExitResult {
		const {node} = path;

		if (
			node.type === "JSClassDeclaration" &&
			!!node.meta.body.find((member) =>
				member.type === "JSClassMethod" &&
				member.key.type === "JSStaticPropertyKey" &&
				member.key.value.type === "JSIdentifier" &&
				member.key.value.name === "shouldComponentUpdate"
			) &&
			(doesNodeMatchPattern(node.meta.superClass, "React.PureComponent") ||
			doesNodeMatchPattern(node.meta.superClass, "PureComponent"))
		) {
			path.context.addNodeDiagnostic(
				node,
				descriptions.LINT.REACT_NO_REDUNDANT_SHOULD_COMPONENT_UPDATE,
			);
		}

		return node;
	},
};
