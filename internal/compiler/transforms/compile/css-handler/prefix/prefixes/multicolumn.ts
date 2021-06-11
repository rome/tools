import {
	createPrefixCSSBlockVisitor,
	prefixCSSProperty,
} from "@internal/compiler/transforms/compile/css-handler/prefix/utils";

// https://github.com/postcss/autoprefixer/blob/main/data/prefixes.js#L292-L293
// https://github.com/postcss/autoprefixer/blob/main/lib/hacks/break-props.js
function breakRenamer(value: string) {
	return value.replace("break", "column-break");
}

export default [
	...["break-before", "break-after", "break-inside"].map((propertyName) =>
		createPrefixCSSBlockVisitor({
			name: propertyName,
			enter(path) {
				return prefixCSSProperty({
					path,
					propertyName,
					browserFeaturesKey: "multicolumn",
					rename: breakRenamer,
				});
			},
		})
	),

	// https://github.com/postcss/autoprefixer/blob/main/data/prefixes.js#L272-L286
	...[
		"columns",
		"column-width",
		"column-gap",
		"column-rule",
		"column-rule-color",
		"column-rule-width",
		"column-count",
		"column-rule-style",
		"column-span",
		"column-fill",
	].map((propertyName) =>
		createPrefixCSSBlockVisitor({
			name: propertyName,
			enter(path) {
				return prefixCSSProperty({
					path,
					propertyName,
					browserFeaturesKey: "multicolumn",
				});
			},
		})
	),
];
