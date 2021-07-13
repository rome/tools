import {
	createPrefixCSSBlockVisitor,
	prefixCSSProperty,
} from "@internal/compiler/transforms/compile/css-handler/prefix/utils";

export default [
	// https://github.com/Fyrd/caniuse/blob/main/features-json/transforms2d.json
	// https://github.com/postcss/autoprefixer/blob/main/data/prefixes.js#L128-L129
	...["transform", "transform-origin"].map((propertyName) =>
		createPrefixCSSBlockVisitor({
			name: propertyName,
			enter(path) {
				return prefixCSSProperty({
					path,
					propertyName,
					browserFeaturesKey: "transforms2d",
				});
			},
		})
	),

	//https://github.com/postcss/autoprefixer/blob/main/data/prefixes.js#L138-L152
	...[
		"perspective",
		"perspective-origin",
		"transform-style",
		"backface-visibility",
	].map((propertyName) =>
		createPrefixCSSBlockVisitor({
			name: propertyName,
			enter(path) {
				return prefixCSSProperty({
					path,
					propertyName,
					browserFeaturesKey: "transforms3d",
				});
			},
		})
	),
];
