import {descriptions} from "@internal/diagnostics";
import {createLintVisitor, signals} from "@internal/compiler";
import {getJSXAttribute, hasJSXAttribute} from "@internal/js-ast-utils";
import {isJSXDOMElement} from "@internal/js-ast-utils/isJSXDOMElement";
import isHTMLElement from "@internal/js-ast-utils/isHTMLElement";
import hasHTMLAttribute from "@internal/js-ast-utils/hasHTMLAttribute";
import getHTMLAttribute from "@internal/js-ast-utils/getHTMLAttribute";

export default createLintVisitor({
	name: "a11y/noAutofocus",

	enter(path) {
		const {node} = path;

		if (isJSXDOMElement(node) && hasJSXAttribute(node, "autoFocus")) {
			return path.addFixableDiagnostic(
				{
					target: getJSXAttribute(node, "autoFocus"),
					fixed: signals.replace({
						...node,
						attributes: node.attributes.filter((attribute) =>
							attribute.type !== "JSXAttribute" ||
							attribute.name.name !== "autoFocus"
						),
					}),
				},
				descriptions.LINT.A11Y_NO_AUTOFOCUS,
			);
		} else if (isHTMLElement(node) && hasHTMLAttribute(node, "autofocus")) {
			return path.addFixableDiagnostic(
				{
					target: getHTMLAttribute(node, "autofocus"),
					fixed: signals.replace({
						...node,
						attributes: node.attributes.filter((attribute) =>
							attribute.type !== "HTMLAttribute" ||
							attribute.name.name !== "autofocus"
						),
					}),
				},
				descriptions.LINT.A11Y_NO_AUTOFOCUS,
			);
		}

		return signals.retain;
	},
});
