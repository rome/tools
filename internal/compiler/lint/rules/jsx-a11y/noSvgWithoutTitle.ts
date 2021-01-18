import {Path, createVisitor, signals} from "@internal/compiler";
import {descriptions} from "@internal/diagnostics";
import {
	cleanJSXText,
	hasJSXAttribute,
	isJSXElement,
} from "@internal/js-ast-utils";
import {AnyNode, JSXElement} from "@internal/ast";

const SVG_SHAPES = [
	"circle",
	"ellipse",
	"line",
	"path",
	"polygon",
	"polyline",
	"rect",
];
const SVG_GROUP = "g";

function hasSvgTitle(node: JSXElement, path: Path): boolean {
	if (!node.children) {
		return false;
	}

	if (hasJSXAttribute(node, "aria-hidden")) {
		return true;
	}
	const title = node.children.find((child) => isJSXElement(child, "title")) as JSXElement;
	const hasTitle = !!title;

	if (
		hasTitle &&
		(title.children[0]?.type !== "JSXText" ||
		!cleanJSXText(title.children[0].value))
	) {
		path.context.addNodeDiagnostic(
			title,
			descriptions.LINT.JSX_A11Y_SVG_TITLE_IS_EMPTY,
		);
		return true;
	}

	return hasTitle;
}

function processChild(node: AnyNode, path: Path): boolean {
	if (node.type !== "JSXElement") {
		return false;
	}
	const hasSvgShape = SVG_SHAPES.some((shape) => isJSXElement(node, shape));
	const isSvgGroup = isJSXElement(node, SVG_GROUP);
	const svgGroupHasTitle =
		isSvgGroup && node.children.some((child) => processChild(child, path));
	return (
		hasSvgTitle(node, path) ||
		(isSvgGroup && svgGroupHasTitle) ||
		(hasSvgShape &&
		SVG_SHAPES.some((shape) => isJSXElement(node, shape)) &&
		hasSvgTitle(node, path))
	);
}

export default createVisitor({
	name: "jsx-a11y/noSvgWithoutTitle",
	enter(path) {
		const {node} = path;
		if (
			isJSXElement(node, "svg") &&
			!hasSvgTitle(node, path) &&
			!node.children.some((child) =>
				child.type === "JSXElement" && processChild(child, path)
			)
		) {
			path.context.addNodeDiagnostic(
				node,
				descriptions.LINT.JSX_A11Y_SVG_HAS_TITLE,
			);
		}

		return signals.retain;
	},
});
