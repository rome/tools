import {CompilerPath, createLintVisitor, signals} from "@internal/compiler";
import {findClosestStringMatch, toKebabCase} from "@internal/string-utils";
import {descriptions} from "@internal/diagnostics";
import {ARIAProperty, ariaPropsMap} from "@internal/compiler/lint/utils/aria";
import {markup} from "@internal/markup";
import {
	HTMLAttribute,
	JSXAttribute,
	htmlIdentifier,
	jsxIdentifier,
} from "@internal/ast";

interface DetermineInvalidAriaProp {
	path: CompilerPath;
	node: JSXAttribute | HTMLAttribute;
	attributeName: string;
	emitFix: (closestMatch: string) => JSXAttribute | HTMLAttribute;
}

function determineInvalidAriaProp(
	{path, node, attributeName, emitFix}: DetermineInvalidAriaProp,
) {
	const ariaPropsArray = Array.from(ariaPropsMap).map((ariaProps) =>
		ariaProps[0]
	);

	const closestMatch = findClosestStringMatch(attributeName, ariaPropsArray);

	const isInvalidAriaProperty = !ariaPropsArray.includes(
		attributeName as ARIAProperty,
	);

	if (closestMatch !== undefined && isInvalidAriaProperty) {
		const fix = emitFix(closestMatch);
		return path.addFixableDiagnostic(
			{
				suggestions: [
					{
						title: markup`ARIA Spelling Mistake`,
						description: markup``,
						fixed: signals.replace(fix),
					},
				],
			},
			descriptions.LINT.A11_Y_USE_ARIA_PROPS(attributeName),
		);
	} else if (isInvalidAriaProperty) {
		path.context.addNodeDiagnostic(
			node,
			descriptions.LINT.A11_Y_USE_ARIA_PROPS(attributeName),
		);
	}
	return signals.retain;
}

export default createLintVisitor({
	name: "a11y/useAriaProps",
	enter(path) {
		const {node} = path;

		if (
			node.type === "JSXAttribute" &&
			node.name.type === "JSXIdentifier" &&
			node.name.name.startsWith("aria-")
		) {
			return determineInvalidAriaProp({
				path,
				attributeName: node.name.name,
				node,
				emitFix: (closestMatch) => {
					return {
						...node,
						// React Documentation states that ariaProps should be in kebab-case, not camelCase:
						// https://reactjs.org/docs/accessibility.html#wai-aria
						name: jsxIdentifier.create({
							name: toKebabCase(closestMatch),
						}),
					};
				},
			});
		} else if (
			node.type === "HTMLAttribute" &&
			node.name?.name.startsWith("aria-")
		) {
			return determineInvalidAriaProp({
				path,
				attributeName: node.name.name,
				node,
				emitFix: (closestMatch): HTMLAttribute => {
					return {
						...node,
						name: htmlIdentifier.create({
							name: toKebabCase(closestMatch),
						}),
					};
				},
			});
		}
		return signals.retain;
	},
});
