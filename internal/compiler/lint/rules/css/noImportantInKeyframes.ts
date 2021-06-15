import {createLintVisitor, signals} from "@internal/compiler";
import {descriptions} from "@internal/diagnostics";
import {
	CSSAtRule,
	CSSDeclaration,
	cssDeclaration,
	cssKeyframeBlock,
} from "@internal/ast";

function hasImportant(values: Array<CSSAtRule | CSSDeclaration>) {
	for (const rule of values) {
		if (rule.type === "CSSDeclaration") {
			if (rule.important) {
				return true;
			}
		}
	}
	return false;
}
export default createLintVisitor({
	name: "css/noImportantInKeyframes",
	enter(path) {
		const {node} = path;

		if (
			node.type === "CSSKeyframeBlock" &&
			node.value &&
			hasImportant(node.value)
		) {
			let target: CSSDeclaration[] = [];
			const newNode = cssKeyframeBlock.create({
				...node,
				value: node.value.map((rule) => {
					if (rule.type === "CSSDeclaration") {
						if (rule.important) {
							target.push(rule);
							return cssDeclaration.create({
								...rule,
								important: false,
							});
						}
					}
					return rule;
				}),
			});

			return path.addFixableDiagnostic(
				{
					target,
					fixed: signals.replace(newNode),
				},
				descriptions.LINT.CSS_NO_IMPORTANT_IN_KEYFRAME,
			);
		}

		return signals.retain;
	},
});
