import {
	createPrefixCSSBlockVisitor,
	prefixCSSProperty,
} from "@internal/compiler/transforms/compile/css-handler/prefix/utils";

// https://github.com/Fyrd/caniuse/blob/main/features-json/css-backdrop-filter.json
// https://github.com/postcss/autoprefixer/blob/main/data/prefixes.js#L245-L247
export default [
	createPrefixCSSBlockVisitor({
		name: "backdrop-filter",
		enter(path) {
			return prefixCSSProperty({
				path,
				propertyName: "backdrop-filter",
				browserFeaturesKey: "css-backdrop-filter",
			});
		},
	}),
];
