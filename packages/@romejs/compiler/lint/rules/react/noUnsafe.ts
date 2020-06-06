import {Path, TransformExitResult} from "@romejs/compiler";
import {descriptions} from "@romejs/diagnostics";
import {AnyNode, JSIdentifier} from "@romejs/ast";

const UNSAFE_METHODS = [
	{
		oldMethod: "UNSAFE_componentWillMount",
		newMethod: "componentDidMount",
	},
	{
		oldMethod: "UNSAFE_componentWillReceiveProps",
		newMethod: "getDerivedStateFromProps",
	},
	{
		oldMethod: "UNSAFE_componentWillUpdate",
		newMethod: "componentDidUpdate",
	},
];

function getUnsafeDef(node: AnyNode) {
	return (
		node.type === "JSClassMethod" &&
		node.key.type === "JSStaticPropertyKey" &&
		node.key.value.type === "JSIdentifier" &&
		UNSAFE_METHODS.find((def) =>
			def.oldMethod === (node.key.value as JSIdentifier).name
		)
	);
}

export default {
	name: "reactNoUnsafe",

	enter(path: Path): TransformExitResult {
		const {node} = path;

		const unsafeDef = getUnsafeDef(node);

		if (unsafeDef) {
			path.context.addNodeDiagnostic(
				node,
				descriptions.LINT.REACT_NO_UNSAFE(
					unsafeDef.oldMethod,
					unsafeDef.newMethod,
				),
			);
		}

		return node;
	},
};
