import {Path, TransformExitResult} from "@romejs/js-compiler";
import {descriptions} from "@romejs/diagnostics";
import {AnyNode, JSIdentifier} from "@romejs/ast";

const UNSAFE_METHODS = [
	{
		oldMethod: "UNSAFE_componentWillMount",
		newMethod: "componentDidMount",
		details: "See https://reactjs.org/blog/2018/03/27/update-on-async-rendering.html.",
	},
	{
		oldMethod: "UNSAFE_componentWillReceiveProps",
		newMethod: "getDerivedStateFromProps",
		details: "See https://reactjs.org/blog/2018/03/27/update-on-async-rendering.html.",
	},
	{
		oldMethod: "UNSAFE_componentWillUpdate",
		newMethod: "componentDidUpdate",
		details: "See https://reactjs.org/blog/2018/03/27/update-on-async-rendering.html.",
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
	name: "noUnsafe",

	enter(path: Path): TransformExitResult {
		const {node} = path;

		const unsafeDef = getUnsafeDef(node);

		if (unsafeDef) {
			path.context.addNodeDiagnostic(
				node,
				descriptions.LINT.REACT_NO_UNSAFE(
					unsafeDef.oldMethod,
					unsafeDef.newMethod,
					unsafeDef.details,
				),
			);
		}

		return node;
	},
};
