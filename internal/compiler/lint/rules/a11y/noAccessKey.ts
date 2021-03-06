import {descriptions} from "@internal/diagnostics";
import {createVisitor, signals} from "@internal/compiler";
import {getJSXAttribute, hasJSXAttribute} from "@internal/js-ast-utils";
import {isJSXDOMElement} from "@internal/js-ast-utils/isJSXDOMElement";
import isHTMLElement from "@internal/js-ast-utils/isHTMLElement";
import getHTMLAttribute from "@internal/js-ast-utils/getHTMLAttribute";

export default createVisitor({
	name: "jsx-a11y/noAccessKey",

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
				descriptions.LINT.JSX_A11Y_NO_ACCESS_KEY,
			);
		} else if (isHTMLElement(node) && node.name.name === "input") {
			const accessKeyAttribute = getHTMLAttribute(node, "accesskey");
			if (accessKeyAttribute) {
				path.context.addNodeDiagnostic(
					node,
					descriptions.LINT.A11Y_NO_ACCESS_KEY,
				);
			}
		}

		return signals.retain;
	},
});
