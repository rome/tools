import {CompilerPath, createLintVisitor, signals} from "@internal/compiler";
import {descriptions} from "@internal/diagnostics";
import {
	cleanJSXText,
	hasJSXAttribute,
	isJSXElement,
} from "@internal/js-ast-utils";
import {
	AnyHTMLChildNode,
	HTMLElement,
	JSXElement,
	JSXExpressionContainer,
	JSXFragment,
	JSXSpreadChild,
	JSXText,
} from "@internal/ast";
import isHTMLElement from "@internal/js-ast-utils/isHTMLElement";
import getHTMLAttribute from "@internal/js-ast-utils/getHTMLAttribute";

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

function hasSvgTitle(
	node: JSXElement | HTMLElement,
	path: CompilerPath,
): boolean {
	if (!node.children) {
		return false;
	}

	if (isHTMLElement(node)) {
		const ariaHidden = getHTMLAttribute(node, "aria-hidden");
		if (ariaHidden?.value?.value === "true") {
			return true;
		}
		const title = node.children.find((child) =>
			isHTMLElement(child) && child.name.name === "title"
		) as HTMLElement | undefined;

		if (title) {
			const text = title.children.find((c) => c.type === "HTMLText");

			if (!text) {
				path.context.addNodeDiagnostic(
					node,
					descriptions.LINT.A11_Y_NO_SVG_WITHOUT_TITLE,
				);
				return false;
			}
			return true;
		}

		return false;
	} else {
		if (hasJSXAttribute(node, "aria-hidden")) {
			return true;
		}
		const title = node.children.find((child) => isJSXElement(child, "title")) as
			| JSXElement
			| undefined;

		if (
			title &&
			(title.children[0]?.type !== "JSXText" ||
			!cleanJSXText(title.children[0].value))
		) {
			path.context.addNodeDiagnostic(
				node,
				descriptions.LINT.A11_Y_NO_SVG_WITHOUT_TITLE,
			);
			return true;
		}

		return !!title;
	}
}

function processChild(
	node:
		| JSXText
		| JSXExpressionContainer
		| JSXSpreadChild
		| JSXElement
		| JSXFragment,
	path: CompilerPath,
): boolean {
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

function processHtmlChild(node: AnyHTMLChildNode, path: CompilerPath) {
	if (!isHTMLElement(node)) {
		return false;
	}
	if (node.name.name === SVG_GROUP) {
		const svgGroupHasTitle = node.children.some((child) =>
			processHtmlChild(child, path)
		);
		return hasSvgTitle(node, path) || svgGroupHasTitle;
	} else {
		const hasSvgShape = SVG_SHAPES.some((shape) => node.name.name === shape);

		return hasSvgTitle(node, path) || hasSvgShape;
	}
}

export default createLintVisitor({
	name: "a11y/noSvgWithoutTitle",
	enter(path) {
		const {node} = path;
		if (isHTMLElement(node) && node.name.name === "svg") {
			if (
				!(hasSvgTitle(node, path) ||
				node.children.some((child) => processHtmlChild(child, path)))
			) {
				path.context.addNodeDiagnostic(
					node,
					descriptions.LINT.A11_Y_NO_SVG_WITHOUT_TITLE,
				);
			}
		} else if (
			isJSXElement(node, "svg") &&
			!hasSvgTitle(node, path) &&
			!node.children.some((child) => processChild(child, path))
		) {
			path.context.addNodeDiagnostic(
				node,
				descriptions.LINT.A11_Y_NO_SVG_WITHOUT_TITLE,
			);
		}

		return signals.retain;
	},
});
