import {CompilerContext, Path, TransformExitResult} from "@romejs/compiler";
import {descriptions} from "@romejs/diagnostics";
import {
	getJSXAttribute,
	hasJSXAttribute,
	isJSXElement,
} from "@romejs/js-ast-utils";
import getJSXElementName from "@romejs/js-ast-utils/getJSXElementName";
import {
	ARIAProperty,
	ARIARoleDefinition,
	roles,
} from "@romejs/compiler/lint/rules/ariaHelpers";
import {AnyNode, JSXAttribute, JSXElement} from "@romejs/ast";

type CreateFixableDiagnostic = {
	context: CompilerContext;
	node: JSXElement;
	mappedRole: ARIARoleDefinition | undefined;
	roleAttribute: JSXAttribute;
};

function createFixableDiagnostic(
	{context, node, mappedRole, roleAttribute}: CreateFixableDiagnostic,
) {
	let ariaAttributesToRemove: Array<AnyNode> = [];
	if (mappedRole) {
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
	const fixed = {
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

	context.addFixableDiagnostic(
		{
			target: [roleAttribute, ...ariaAttributesToRemove],
			old: node,
			suggestions: [
				{
					title: "Remove the role attribute",
					description: "",
					fixed,
				},
			],
		},
		descriptions.LINT.JSX_A11Y_NO_REDUNDANT_ROLES,
	);
}

export default {
	name: "jsxA11YNoRedundantRoles",
	enter(path: Path): TransformExitResult {
		const {node, context} = path;

		if (isJSXElement(node) && hasJSXAttribute(node, "role")) {
			const elementName = getJSXElementName(node);

			const roleAttribute = getJSXAttribute(node, "role");
			if (
				roleAttribute &&
				roleAttribute.value &&
				roleAttribute.value.type === "JSStringLiteral"
			) {
				let elementHasARole;

				const mappedRole = roles.get(roleAttribute.value.value);
				if (mappedRole && mappedRole.baseConcepts) {
					elementHasARole = mappedRole.baseConcepts.some(({concept, module}) => {
						if (module === "HTML") {
							return concept.name === elementName;
						}
						return true;
					});
				}

				if (elementName === roleAttribute.value.value || elementHasARole) {
					createFixableDiagnostic({
						roleAttribute,
						node,
						context,
						mappedRole,
					});
				}
			}
		}
		return node;
	},
};
