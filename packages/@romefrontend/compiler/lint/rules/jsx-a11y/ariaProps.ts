import {Path} from "@romefrontend/compiler";
import {findClosestStringMatch, toKebabCase} from "@romefrontend/string-utils";
import {descriptions} from "@romefrontend/diagnostics";
import {TransformExitResult} from "@romefrontend/compiler/types";
import {
	ARIAProperty,
	ariaPropsMap,
} from "@romefrontend/compiler/lint/utils/aria";

export default {
	name: "jsx-a11y/ariaProps",
	enter(path: Path): TransformExitResult {
		const {node, context} = path;

		if (
			node.type === "JSXAttribute" &&
			node.name.type === "JSXIdentifier" &&
			node.name.name.indexOf("aria-") === 0
		) {
			const ariaPropsArray = Array.from(ariaPropsMap).map((ariaProps) =>
				ariaProps[0]
			);

			const closestMatch = findClosestStringMatch(
				node.name.name,
				ariaPropsArray,
			);

			let fixed;
			if (closestMatch !== undefined) {
				fixed = {
					...node,
					name: {
						...node.name,
						// React Documentation states that ariaProps should be in kebab-case, not camelCase:
						// https://reactjs.org/docs/accessibility.html#wai-aria
						name: toKebabCase(closestMatch),
					},
				};
			}

			const isInvalidAriaProperty = !ariaPropsArray.includes(
				(node.name.name as ARIAProperty),
			);
			if (fixed !== undefined && isInvalidAriaProperty) {
				return context.addFixableDiagnostic(
					{
						old: node,
						suggestions: [
							{
								title: "ARIA Spelling Mistake",
								description: "",
								fixed,
							},
						],
					},
					descriptions.LINT.JSX_A11Y_ARIA_PROPS(node.name.name),
				);
			} else if (isInvalidAriaProperty) {
				context.addNodeDiagnostic(
					node,
					descriptions.LINT.JSX_A11Y_ARIA_PROPS(node.name.name),
				);
			}
		}
		return node;
	},
};
