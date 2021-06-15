import {descriptions} from "@internal/diagnostics";
import {createLintVisitor, signals} from "@internal/compiler";
import {doesNodeMatchPattern, hasJSXAttribute} from "@internal/js-ast-utils";
import {isJSXDOMElement} from "@internal/js-ast-utils/isJSXDOMElement";
import isHTMLElement from "@internal/js-ast-utils/isHTMLElement";
import hasHTMLAttribute from "@internal/js-ast-utils/hasHTMLAttribute";

export default createLintVisitor({
	name: "a11y/noHeaderScope",

	enter(path) {
		const {node} = path;

		if (
			isJSXDOMElement(node) &&
			hasJSXAttribute(node, "scope") &&
			!doesNodeMatchPattern(node.name, "th")
		) {
			return path.addFixableDiagnostic(
				{
					fixed: signals.replace({
						...node,
						attributes: node.attributes.filter((attribute) =>
							attribute.type !== "JSXAttribute" ||
							attribute.name.name !== "scope"
						),
					}),
				},
				descriptions.LINT.A11Y_NO_SCOPE,
			);
		} else if (
			isHTMLElement(node) &&
			hasHTMLAttribute(node, "scope") &&
			node.name.name !== "th"
		) {
			return path.addFixableDiagnostic(
				{
					fixed: signals.replace({
						...node,
						attributes: node.attributes.filter((attribute) =>
							attribute.type !== "HTMLAttribute" ||
							attribute.name.name !== "scope"
						),
					}),
				},
				descriptions.LINT.A11Y_NO_SCOPE,
			);
		}

		return signals.retain;
	},
});
