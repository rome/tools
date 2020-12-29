import {createVisitor, signals} from "@internal/compiler";
import {findClosestStringMatch, toKebabCase} from "@internal/string-utils";
import {descriptions} from "@internal/diagnostics";
import {ARIAProperty, ariaPropsMap} from "@internal/compiler/lint/utils/aria";
import {markup} from "@internal/markup";

export default createVisitor({
	name: "jsx-a11y/useAriaProps",
	enter(path) {
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
				node.name.name as ARIAProperty,
			);
			if (fixed !== undefined && isInvalidAriaProperty) {
				return path.addFixableDiagnostic(
					{
						suggestions: [
							{
								title: markup`ARIA Spelling Mistake`,
								description: markup``,
								fixed: signals.replace(fixed),
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
		return signals.retain;
	},
});
