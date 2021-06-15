import {CompilerPath, createLintVisitor, signals} from "@internal/compiler";
import {descriptions} from "@internal/diagnostics";
import {isFor} from "@internal/js-ast-utils";

function isContinueInsideLastAncestorPath(
	ancestryArr: CompilerPath[],
	path: CompilerPath,
): boolean {
	const length = ancestryArr.length;
	for (let index = length; index > 1; index--) {
		const node = ancestryArr[index - 1].node;
		if (node.type === "JSBlockStatement" && node.body.length > 0) {
			const bodySize = node.body.length;
			const lastBodyNode = node.body[bodySize - 1];
			if (
				!((length === 1 && lastBodyNode === path.node) ||
				(length > 1 && lastBodyNode === ancestryArr[index - 2].node))
			) {
				return false;
			}
		}
	}
	return true;
}

function isContinueTheLastStatement(
	ancestryArr: CompilerPath[],
	path: CompilerPath,
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

//return true if continue label is undefined or equal to its parent's looplabel
function containsParentLoopLabel(
	path: CompilerPath,
	parentPath: CompilerPath,
): boolean {
	if (path.node.type === "JSContinueStatement" && path.node.label !== undefined) {
		if (
			parentPath.parent.type === "JSLabeledStatement" &&
			path.node.label.name === parentPath.parent.label.name
		) {
			return true;
		} else {
			return false;
		}
	}
	return true;
}

function isContinueUnNecessary(path: CompilerPath): boolean {
	const ancestryArr: CompilerPath[] = [];
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
		containsParentLoopLabel(path, parentPath) &&
		isContinueInsideLastAncestorPath(ancestryArr, path)
	);
}

export default createLintVisitor({
	name: "js/noUnnecessaryContinue",
	enter(path) {
		const {node} = path;
		if (node.type !== "JSContinueStatement") {
			return signals.retain;
		} else if (
			node.type === "JSContinueStatement" &&
			isContinueUnNecessary(path)
		) {
			return path.addFixableDiagnostic(
				{
					fixed: signals.remove,
				},
				descriptions.LINT.JS_NO_UNNECESSARY_CONTINUE,
			);
		}

		return signals.retain;
	},
});
