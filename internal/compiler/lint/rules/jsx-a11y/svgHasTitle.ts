import {createVisitor, signals} from "@internal/compiler";
import {descriptions} from "@internal/diagnostics";
import {isJSXElement, cleanJSXText} from "@internal/js-ast-utils";
import {JSXElement, jsStringLiteral} from "@internal/ast";

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
	if(!node.children) {
		return false;
	}
	const title = (node.children.find((child) => isJSXElement(child, "title")) as JSXElement);

	return title ? title.children[0]?.type === "JSXText" && !!cleanJSXText(title.children[0].value) : false
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
