import {CompilerPath, createLintVisitor, signals} from "@internal/compiler";
import {descriptions} from "@internal/diagnostics";
import {
	getJSXAttribute,
	getJSXElementName,
	hasJSXAttribute,
} from "@internal/js-ast-utils";
import {
	AnyNode,
	HTMLAttribute,
	HTMLElement,
	JSXAttribute,
	JSXElement,
} from "@internal/ast";
import {
	ARIAProperty,
	ARIARoleDefinition,
	ariaRolesMap,
} from "@internal/compiler/lint/utils/aria";
import {markup} from "@internal/markup";
import {isJSXDOMElement} from "@internal/js-ast-utils/isJSXDOMElement";
import isHTMLElement from "@internal/js-ast-utils/isHTMLElement";
import getHTMLAttribute from "@internal/js-ast-utils/getHTMLAttribute";
import hasHTMLAttribute from "@internal/js-ast-utils/hasHTMLAttribute";

type JSXOrHTMLElement = JSXElement | HTMLElement;
type JSXOrHTMLAttribute = JSXAttribute | HTMLAttribute;

type CreateFixableDiagnostic = {
	path: CompilerPath;
	node: JSXOrHTMLElement;
	mappedRole: ARIARoleDefinition | undefined;
	roleAttribute: JSXOrHTMLAttribute;
	elementName: string;
	roleName: string;
};

function getAttribute(node: JSXOrHTMLElement, prop: string) {
	if (isHTMLElement(node)) {
		return getHTMLAttribute(node, prop);
	} else {
		return getJSXAttribute(node, prop);
	}
}

function getFixed(node: JSXOrHTMLElement, mappedRole?: ARIARoleDefinition) {
	if (isHTMLElement(node)) {
		return {
			...node,
			attributes: node.attributes.filter((attr) => {
				return attr.type === "HTMLAttribute" && attr.name.name !== "role";
			}).filter((attr) => {
				if (attr.type === "HTMLAttribute") {
					if (mappedRole) {
						return (
							attr.type === "HTMLAttribute" &&
							!mappedRole.requiredProps.includes(attr.name.name as ARIAProperty)
						);
					}
					return attr.name.name !== "role";
				}

				return true;
			}),
		};
	} else {
		return {
			...node,
			attributes: node.attributes.filter((attr) => {
				return attr.type === "JSXAttribute" && attr.name.name !== "role";
			}).filter((attr) => {
				if (attr.type === "JSXAttribute") {
					if (mappedRole) {
						return (
							attr.type === "JSXAttribute" &&
							!mappedRole.requiredProps.includes(attr.name.name as ARIAProperty)
						);
					}
					return attr.name.name !== "role";
				}

				return true;
			}),
		};
	}
}

function createFixableDiagnostic(
	{path, node, mappedRole, roleAttribute, elementName, roleName}: CreateFixableDiagnostic,
) {
	let ariaAttributesToRemove: AnyNode[] = [];
	if (mappedRole) {
		// here we retrieve the aria-* attributes that are not needed
		// e.g. role="heading" aria-level="1"
		ariaAttributesToRemove = mappedRole.requiredProps.reduce(
			(nodes, prop) => {
				const attr = getAttribute(node, prop as string);
				if (attr) {
					nodes.push(attr);
				}
				return nodes;
			},
			[] as AnyNode[],
		);
	}
	const titleSuggestion =
		ariaAttributesToRemove.length > 0
			? markup`Remove the role attribute and ARIA attributes.`
			: markup`Remove the role attribute.`;

	const fixed = getFixed(node, mappedRole);

	return path.addFixableDiagnostic(
		{
			target: [roleAttribute, ...ariaAttributesToRemove],
			suggestions: [
				{
					title: titleSuggestion,
					description: markup``,
					fixed: signals.replace(fixed),
				},
			],
		},
		descriptions.LINT.A11Y_NO_REDUNDANT_ROLES(roleName, elementName),
	);
}

function getElementName(node: JSXOrHTMLElement) {
	if (isHTMLElement(node)) {
		return node.name.name;
	}
	return getJSXElementName(node);
}

export default createLintVisitor({
	name: "a11y/noRedundantRoles",
	enter(path) {
		const {node} = path;

		if (
			(isHTMLElement(node) && hasHTMLAttribute(node, "role")) ||
			(isJSXDOMElement(node) && hasJSXAttribute(node, "role"))
		) {
			const elementName = getElementName(node);

			const roleAttribute = getAttribute(node, "role");
			if (
				roleAttribute?.value?.type === "JSStringLiteral" ||
				roleAttribute?.value?.type === "HTMLString"
			) {
				let elementHasARole;

				const mappedRole = ariaRolesMap.get(roleAttribute.value.value);
				// here we cover cases where "role" attribute and the element name differs in naming
				// e.g. h1 and role="heading"
				if (mappedRole?.baseConcepts) {
					elementHasARole = mappedRole.baseConcepts.some(({concept, module}) => {
						if (module === "HTML") {
							// here we should also match additional attributes
							// e.g. role="checkbox" <=> <input type="checkbox" />
							if (concept.attributes) {
								return concept.attributes.every(({name, value}) => {
									const attr = getAttribute(node, name);
									return (
										(attr?.value?.type === "JSStringLiteral" ||
										attr?.value?.type === "HTMLString") &&
										attr.value.value === value
									);
								});
							} else {
								return concept.name === elementName;
							}
						}
						return true;
					});
				}

				if (elementName === roleAttribute.value.value || elementHasARole) {
					createFixableDiagnostic({
						roleAttribute,
						node,
						path,
						mappedRole,
						elementName,
						roleName: roleAttribute.value.value,
					});
				}
			}
		}

		return signals.retain;
	},
});
