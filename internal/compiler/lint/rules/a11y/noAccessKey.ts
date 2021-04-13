import {descriptions} from "@internal/diagnostics";
import {createVisitor, signals} from "@internal/compiler";
import {getJSXAttribute, hasJSXAttribute} from "@internal/js-ast-utils";
import {isJSXDOMElement} from "@internal/js-ast-utils/isJSXDOMElement";
import isHTMLElement from "@internal/js-ast-utils/isHTMLElement";
import hasHTMLAttribute from "@internal/js-ast-utils/hasHTMLAttribute";
import getHTMLAttribute from "@internal/js-ast-utils/getHTMLAttribute";

export default createVisitor({
	name: "a11y/noAccessKey",

	enter(path) {
		const {node} = path;

		if (isHTMLElement(node) && hasHTMLAttribute(node, "accesskey")) {
			return path.addFixableDiagnostic(
				{
					target: getHTMLAttribute(node, "accesskey"),
					fixed: signals.replace({
						...node,
						attributes: node.attributes.filter((attribute) =>
							attribute.type !== "HTMLAttribute" ||
							attribute.name.name !== "accesskey"
						),
					}),
				},
				descriptions.LINT.A11_Y_NO_ACCESS_KEY,
			);
		} else if (isJSXDOMElement(node) && hasJSXAttribute(node, "accessKey")) {
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
				descriptions.LINT.A11_Y_NO_ACCESS_KEY,
			);
		}

		return signals.retain;
	},
});
