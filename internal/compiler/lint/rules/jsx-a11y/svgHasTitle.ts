import {createVisitor, signals} from "@internal/compiler";
import {descriptions} from "@internal/diagnostics";
import {isJSXElement} from "@internal/js-ast-utils";
import {JSXElement} from "@internal/ast";

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

function hasSvgTitle(node: JSXElement): boolean {
	return (
		node.children?.length > 0 &&
		node.children.some((child) => isJSXElement(child, "title"))
	);
}

function processChild(node: JSXElement): boolean {
	const svgShape = SVG_SHAPES.find((shape) => isJSXElement(node, shape));
	const isSvgGroup = isJSXElement(node, SVG_GROUP);
	const svgGroupHasTitle =
		isSvgGroup &&
		!!node.children.find((child) => processChild((child as JSXElement)));
	return (
		hasSvgTitle(node) ||
		(isSvgGroup && svgGroupHasTitle) ||
		(!!svgShape &&
		!!SVG_SHAPES.find((shape) => isJSXElement(node, shape)) &&
		hasSvgTitle(node))
	);
}

export default createVisitor({
	name: "jsx-a11y/svgHasTitle",
	enter(path) {
		const {node} = path;
		if (
			isJSXElement(node, "svg") &&
			!hasSvgTitle(node) &&
			!node.children.find((child) =>
				child.type === "JSXElement" && processChild(child)
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
