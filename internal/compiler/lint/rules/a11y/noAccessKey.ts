import {descriptions} from "@internal/diagnostics";
import {createLintVisitor, signals} from "@internal/compiler";
import {getJSXAttribute, hasJSXAttribute} from "@internal/js-ast-utils";
import {isJSXDOMElement} from "@internal/js-ast-utils/isJSXDOMElement";
import isHTMLElement from "@internal/js-ast-utils/isHTMLElement";
import getHTMLAttribute from "@internal/js-ast-utils/getHTMLAttribute";

export default createLintVisitor({
	name: "a11y/noAccessKey",

	enter(path) {
		const {node} = path;

		if (isJSXDOMElement(node) && hasJSXAttribute(node, "accessKey")) {
			return path.addFixableDiagnostic(
				{
					target: getJSXAttribute(node, "accessKey"),
					fixed: signals.replace({
						...node,
						attributes: node.attributes.filter((attribute) =>
							attribute.type !== "JSXAttribute" ||
							attribute.name.name !== "accessKey"
						),
					}),
				},
				descriptions.LINT.A11Y_NO_ACCESS_KEY,
			);
		} else if (isHTMLElement(node)) {
			const accessKeyAttribute = getHTMLAttribute(node, "accesskey");
			if (accessKeyAttribute) {
				return path.addFixableDiagnostic(
					{
						target: accessKeyAttribute,
						fixed: signals.replace({
							...node,
							attributes: node.attributes.filter((attribute) =>
								attribute.type !== "HTMLAttribute" ||
								attribute.name.name !== "accesskey"
							),
						}),
					},
					descriptions.LINT.A11Y_NO_ACCESS_KEY,
				);
			}
		}

		return signals.retain;
	},
});
