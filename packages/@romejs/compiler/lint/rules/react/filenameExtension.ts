import {Path, TransformExitResult} from "@romejs/compiler";
import {descriptions} from "@romejs/diagnostics";
import {doesNodeMatchReactPattern} from "@romejs/compiler/lint/utils/react";
import {AnyNode} from "@romejs/ast";

const ALLOWED_EXTENSIONS = [".jsx", ".tsx"];

function isClassComponent(node: AnyNode, path: Path) {
	return (
		(node.type === "JSClassHead" &&
		node.superClass !== undefined &&
		doesNodeMatchReactPattern(node, path.scope, "React.Component")) ||
		doesNodeMatchReactPattern(node, path.scope, "Component") ||
		doesNodeMatchReactPattern(node, path.scope, "React.PureComponent") ||
		doesNodeMatchReactPattern(node, path.scope, "PureComponent")
	);
}

export default {
	name: "reactFilenameExtension",
	enter(path: Path): TransformExitResult {
		const {node, context} = path;

		const extension = context.path.getExtensions();
		if (
			isClassComponent(node, path) &&
			ALLOWED_EXTENSIONS.includes(extension) === false
		) {
			context.addNodeDiagnostic(
				node,
				descriptions.LINT.REACT_FILENAME_EXTENSION,
			);
		}

		return node;
	},
};
