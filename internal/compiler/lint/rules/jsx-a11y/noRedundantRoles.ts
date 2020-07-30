import {Path, createVisitor, signals} from "@internal/compiler";
import {descriptions} from "@internal/diagnostics";
import {
	getJSXAttribute,
	getJSXElementName,
	hasJSXAttribute,
} from "@internal/js-ast-utils";
import {AnyNode, JSXAttribute, JSXElement} from "@internal/ast";
import {
	ARIAProperty,
	ARIARoleDefinition,
	ariaRolesMap,
} from "@internal/compiler/lint/utils/aria";
import {markup} from "@internal/markup";

type CreateFixableDiagnostic = {
	path: Path;
	node: JSXElement;
	mappedRole: ARIARoleDefinition | undefined;
	roleAttribute: JSXAttribute;
	elementName: string;
	roleName: string;
};

function createFixableDiagnostic(
	{path, node, mappedRole, roleAttribute, elementName, roleName}: CreateFixableDiagnostic,
) {
	let ariaAttributesToRemove: Array<AnyNode> = [];
	if (mappedRole) {
		// here we retrieve the aria-* attributes that are not needed
		// e.g. role="heading" aria-level="1"
		ariaAttributesToRemove = mappedRole.requiredProps.reduce(
			(nodes, prop) => {
				const attr = getJSXAttribute(node, (prop as string));
				if (attr) {
					nodes.push(attr);
				}
				return nodes;
			},
			([] as Array<AnyNode>),
		);
	}
	const titleSuggestion =
		ariaAttributesToRemove.length > 0
			? markup`Remove the role attribute and ARIA attributes.`
			: markup`Remove the role attribute.`;

	const fixed: JSXElement = {
		...node,
		attributes: node.attributes.filter((attr) => {
			return attr.type === "JSXAttribute" && attr.name.name !== "role";
		}).filter((attr) => {
			if (attr.type === "JSXAttribute") {
				if (mappedRole) {
					return (
						attr.type === "JSXAttribute" &&
						!mappedRole.requiredProps.includes((attr.name.name as ARIAProperty))
					);
				}
				return attr.name.name !== "role";
			}

			return true;
		}),
	};

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
		descriptions.LINT.JSX_A11Y_NO_REDUNDANT_ROLES(roleName, elementName),
	);
}

export default createVisitor({
	name: "jsx-a11y/noRedundantRoles",
	enter(path) {
		const {node} = path;

		if (node.type === "JSXElement" && hasJSXAttribute(node, "role")) {
			const elementName = getJSXElementName(node);

			const roleAttribute = getJSXAttribute(node, "role");
			if (
				roleAttribute &&
				roleAttribute.value &&
				roleAttribute.value.type === "JSStringLiteral"
			) {
				let elementHasARole;

				const mappedRole = ariaRolesMap.get(roleAttribute.value.value);
				// here we cover cases where "role" attribute and the element name differs in naming
				// e.g. h1 and role="heading"
				if (mappedRole && mappedRole.baseConcepts) {
					elementHasARole = mappedRole.baseConcepts.some(({concept, module}) => {
						if (module === "HTML") {
							// here we should also match additional attributes
							// e.g. role="checkbox" <=> <input type="checkbox" />
							if (concept.attributes) {
								return concept.attributes.every(({name, value}) => {
									const attr = getJSXAttribute(node, name);
									return (
										attr &&
										attr.value &&
										attr.value.type === "JSStringLiteral" &&
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
