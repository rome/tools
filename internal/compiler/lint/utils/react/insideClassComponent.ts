import {Path} from "@internal/compiler";
import doesNodeMatchReactPattern from "./doesNodeMatchReactPattern";

function checkClassComponentAncestry({node, scope}: Path): boolean {
	return (
		node.type === "JSClassHead" &&
		node.superClass !== undefined &&
		(doesNodeMatchReactPattern(node.superClass, scope, "React.Component") ||
		doesNodeMatchReactPattern(node.superClass, scope, "Component") ||
		doesNodeMatchReactPattern(node.superClass, scope, "React.PureComponent") ||
		doesNodeMatchReactPattern(node.superClass, scope, "PureComponent"))
	);
}

export default function insideClassComponent(path: Path): boolean {
	if (checkClassComponentAncestry(path)) {
		return true;
	}
	const ancestor = path.findAncestry(checkClassComponentAncestry);
	return !!ancestor;
}
