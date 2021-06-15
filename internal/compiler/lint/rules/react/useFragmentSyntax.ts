import {createLintVisitor, signals} from "@internal/compiler";
import {descriptions} from "@internal/diagnostics";
import {jsxFragment} from "@internal/ast";
import {hasJSXAttribute} from "@internal/js-ast-utils";
import {doesNodeMatchReactPattern} from "../../utils/react";

export default createLintVisitor({
	name: "react/useFragmentSyntax",
	enter(path) {
		const {node, scope} = path;

		if (
			node.type === "JSXElement" &&
			(doesNodeMatchReactPattern(node.name, scope, "Fragment") ||
			doesNodeMatchReactPattern(node.name, scope, "React.Fragment")) &&
			!hasJSXAttribute(node, "key")
		) {
			return path.addFixableDiagnostic(
				{
					fixed: signals.replace(
						jsxFragment.create({
							children: node.children,
						}),
					),
				},
				descriptions.LINT.REACT_USE_FRAGMENT_SYNTAX,
			);
		}

		return signals.retain;
	},
});
