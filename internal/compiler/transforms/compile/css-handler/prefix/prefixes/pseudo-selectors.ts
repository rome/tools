import {
	createPrefixCSSBlockVisitor,
	createPrefixCSSRootVisitor,
	prefixPseudoSelectorInCSSBlock,
	prefixPseudoSelectorInCSSRoot,
} from "../utils";

const pseudoSelectorsFeatures = new Map([
	// classes
	["any-link", "css-any-link"],
	["fullscreen", "fullscreen"],
	["read-only", "css-read-only-write"],
	["write-only", "css-read-only-write"],

	// elements
	["selection", "css-selection"],
	["placeholder", "css-placeholder"],
	["backdrop", "fullscreen"],
])

export default [
	createPrefixCSSRootVisitor({
		name: "pseudo-selectors",
		enter: (path) => {
			return prefixPseudoSelectorInCSSRoot(path, pseudoSelectorsFeatures);
		},
	}),
	createPrefixCSSBlockVisitor({
		name: "pseudo-selectors",
		enter: (path) => {
			return prefixPseudoSelectorInCSSBlock(path, pseudoSelectorsFeatures);
		},
	}),
];
