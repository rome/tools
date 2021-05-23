import {
	createPrefixCSSBlockVisitor,
	createPrefixCSSRootVisitor,
	prefixPseudoInCSSBlock,
	prefixPseudoInCSSRoot,
} from "../utils";

const pseudoClassesFeatures = new Map([
	["any-link", "css-any-link"],
	["fullscreen", "fullscreen"],
	["read-only", "css-read-only-write"],
	["write-only", "css-read-only-write"],
]);

// COMMENT: unlike other prefixers, here I return a single visitor for all properties
// This is to allow grouping prefixes in an efficient manner

export default [
	createPrefixCSSRootVisitor({
		name: "pseudo-classes",
		enter: (path) => {
			return prefixPseudoInCSSRoot(path, pseudoClassesFeatures);
		},
	}),
	createPrefixCSSBlockVisitor({
		name: "pseudo-classes",
		enter: (path) => {
			return prefixPseudoInCSSBlock(path, pseudoClassesFeatures);
		},
	}),
];
