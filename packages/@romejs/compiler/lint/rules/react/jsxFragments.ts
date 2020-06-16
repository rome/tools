import {Path, TransformExitResult} from "@romejs/compiler";
import {descriptions} from "@romejs/diagnostics";
import {jsxFragment} from "@romejs/ast";
import {doesNodeMatchPattern, hasJSXAttribute} from "@romejs/js-ast-utils";

export default {
	name: "reactJsxFragments",
	enter(path: Path): TransformExitResult {
		const {node, context} = path;

		if (
			node.type === "JSXElement" &&
			(doesNodeMatchPattern(node.name, "Fragment") ||
			doesNodeMatchPattern(node.name, "React.Fragment")) &&
			!hasJSXAttribute(node, "key")
		) {
			return context.addFixableDiagnostic(
				{
					old: node,
					fixed: jsxFragment.create({
						children: node.children,
					}),
				},
				descriptions.LINT.REACT_JSX_FRAGMENTS,
			);
		}

		return node;
	},
};
