import {
	createPrefixVisitor,
	prefixCSSValue,
} from "@internal/compiler/transforms/compile/css-handler/prefix/utils";

function flexRenamer(value: string) {
	return value === "-moz-flex" ? "-moz-box" : value;
}
function inlineflexRenamer(value: string) {
	return value === "-moz-inline-flex" ? "-moz-inline-box" : value;
}

// https://github.com/Fyrd/caniuse/blob/main/features-json/flexbox.json
export default [
	createPrefixVisitor({
		name: "display/flex",
		enter(path) {
			return prefixCSSValue({
				path,
				propertyName: "display",
				value: "flex",
				browserFeaturesKey: "flexbox",
				rename: flexRenamer,
			});
		},
	}),
	createPrefixVisitor({
		name: "display/inline-flex",
		enter(path) {
			return prefixCSSValue({
				path,
				propertyName: "display",
				value: "inline-flex",
				browserFeaturesKey: "flexbox",
				rename: inlineflexRenamer,
			});
		},
	}),
];
