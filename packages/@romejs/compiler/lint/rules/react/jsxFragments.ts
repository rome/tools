import {Path, TransformExitResult} from "@romejs/compiler";
import {descriptions} from "@romejs/diagnostics";
import {jsxFragment} from "@romejs/ast";
import {hasJSXAttribute} from "@romejs/js-ast-utils";
import {doesNodeMatchReactPattern} from "../../utils/react";

export default {
	name: "reactJsxFragments",
	enter(path: Path): TransformExitResult {
		const {node, context, scope} = path;

		if (
			node.type === "JSXElement" &&
			(doesNodeMatchReactPattern(node.name, scope, "Fragment") ||
			doesNodeMatchReactPattern(node.name, scope, "React.Fragment")) &&
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
