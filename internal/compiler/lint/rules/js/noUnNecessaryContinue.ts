import {Path, createVisitor, signals} from "@internal/compiler";
import {descriptions} from "@internal/diagnostics";
import {isFor} from "@internal/js-ast-utils";

function isContinueInsideLastAncestorPath(
	ancestryArr: Array<Path>,
	path: Path,
): boolean {
	const length = ancestryArr.length;
	for (let i = length; i > 1; i--) {
		const node = ancestryArr[i - 1].node;
		if (node.type === "JSBlockStatement" && node.body.length > 0) {
			const bodySize = node.body.length;
			const lastBodyNode = node.body[bodySize - 1];
			if (
				!(length === 1 && lastBodyNode === path.node) &&
				!(length > 1 && lastBodyNode === ancestryArr[i - 2].node)
			) {
				return false;
			}
		}
	}
	return true;
}

function isContinueTheLastStatement(
	ancestryArr: Array<Path>,
	path: Path,
): boolean {
	const node = ancestryArr[0].node;
	if (node.type === "JSBlockStatement") {
		const bodySize = node.body.length;
		if (node.body[bodySize - 1] === path.node) {
			return true;
		}
	}
	return false;
}

function isContinueUnNecessary(path: Path): boolean {
	const ancestryArr: Array<Path> = [];
	const parentPath = path.findAncestry((p) => {
		if (isFor(p.node) || p.node.type === "JSWhileStatement") {
			return true;
		} else {
			ancestryArr.push(p);
			return false;
		}
	});
	if (parentPath === undefined) {
		return false;
	}
	if (ancestryArr.length === 0) {
		return true;
	}
	return (
		isContinueTheLastStatement(ancestryArr, path) &&
		isContinueInsideLastAncestorPath(ancestryArr, path)
	);
}

export default createVisitor({
	name: "js/noUnNecessaryContinue",
	enter(path) {
		const {node} = path;
		if (node.type !== "JSContinueStatement") {
			return signals.retain;
		}
		if (node.type === "JSContinueStatement" && isContinueUnNecessary(path)) {
			return path.addFixableDiagnostic(
				{
					fixed: signals.remove,
				},
				descriptions.LINT.JS_NO_UN_NECESSARY_CONTINUE,
			);
		}

		return signals.retain;
	},
});
