import {
	createPrefixCSSBlockVisitor,
	createPrefixCSSRootVisitor,
	prefixPseudoSelectorInCSSBlock,
	prefixPseudoSelectorInCSSRoot,
} from "../utils";

const pseudoSelectorsFeatures = new Map([
	// classes
	// https://github.com/Fyrd/caniuse/blob/main/features-json/css-any-link.json
	["any-link", "css-any-link"],

	// https://github.com/Fyrd/caniuse/blob/main/features-json/fullscreen.json
	["fullscreen", "fullscreen"],

	// https://github.com/Fyrd/caniuse/blob/main/features-json/css-read-only-write.json
	["read-only", "css-read-only-write"],
	["write-only", "css-read-only-write"],

	// elements
	// https://github.com/Fyrd/caniuse/blob/main/features-json/css-selection.json
	["selection", "css-selection"],

	// https://github.com/Fyrd/caniuse/blob/main/features-json/css-placeholder.json
	["placeholder", "css-placeholder"],

	// https://github.com/Fyrd/caniuse/blob/main/features-json/fullscreen.json
	["backdrop", "fullscreen"],
]);

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
