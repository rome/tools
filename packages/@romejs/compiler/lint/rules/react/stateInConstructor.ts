import {Path, TransformExitResult} from "@romejs/compiler";
import {descriptions} from "@romejs/diagnostics";
import {doesNodeMatchPattern} from "@romejs/js-ast-utils";
import { insideClassComponent } from "../../utils/react";

function isInConstructor(path: Path) {
	const ancestor = path.findAncestry(({node}) =>
		node.type === "JSClassMethod" &&
		node.key.type === "JSStaticPropertyKey" &&
		node.key.value.type === "JSIdentifier" &&
		node.key.value.name === "constructor"
	);

	return ancestor !== undefined;
}

export default {
	name: "stateInConstructor",
	enter(path: Path): TransformExitResult {
		const {node} = path;
		if (
			node.type === "JSClassProperty" &&
			insideClassComponent(path) &&
			doesNodeMatchPattern(node.key.value, "state") &&
			!isInConstructor(path)
		) {
			path.context.addNodeDiagnostic(
				node,
				descriptions.LINT.REACT_STATE_IN_CONSTRUCTOR,
			);
		}
		return node;
	},
};
