import {createVisitor, signals} from "@internal/compiler";
import {descriptions} from "@internal/diagnostics";
import {CSSString} from "@internal/ast";

export default createVisitor({
	name: "css/noInvalidGridTemplateAreas",
	enter(path) {
		const {node} = path;

		if (
			node.type === "CSSDeclaration" &&
			node.name === "grid-template-areas" &&
			node.value.length > 0
		) {
			let length: null | number;
			let incorrectNode: null | CSSString = null;
			const correct = node.value.every((child) => {
				if (child?.type === "CSSString") {
					const childLength = child.value.split(" ").length;
					if (length) {
						const correct = childLength === length;
						if (!correct) {
							incorrectNode = child;
						}
						return correct;
					} else {
						length = childLength;
						return true;
					}
				}
				return true;
			});
			if (!correct) {
				path.context.addNodeDiagnostic(
					incorrectNode ?? node,
					descriptions.LINT.CSS_NO_INVALID_GRID_TEMPLATE_AREAS,
				);
			}
		}

		return signals.retain;
	},
});
