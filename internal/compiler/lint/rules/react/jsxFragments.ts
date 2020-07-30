import {createVisitor, signals} from "@internal/compiler";
import {descriptions} from "@internal/diagnostics";
import {jsxFragment} from "@internal/ast";
import {hasJSXAttribute} from "@internal/js-ast-utils";
import {doesNodeMatchReactPattern} from "../../utils/react";

export default createVisitor({
	name: "react/jsxFragments",
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
				descriptions.LINT.REACT_JSX_FRAGMENTS,
			);
		}

		return signals.retain;
	},
});
