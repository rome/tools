import {createVisitor, signals} from "@internal/compiler";
import {descriptions} from "@internal/diagnostics";

export default createVisitor({
	name: "css/noImportantInKeyframe",
	enter(path) {
		const {node} = path;

		if (node.type === "CSSKeyframeBlock" && node.value) {
			for (const rule of node.value) {
				if (rule.type === "CSSDeclaration") {
					if (rule.important) {
						// TODO: add fixable suggestion
						path.context.addNodeDiagnostic(
							rule,
							descriptions.LINT.CSS_NO_IMPORTANT_IN_KEYFRAME,
						);
					}
				}
			}
		}

		return signals.retain;
	},
});
