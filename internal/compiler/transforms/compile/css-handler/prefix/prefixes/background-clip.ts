import {
	createPrefixVisitor,
	prefixCSSProperty,
} from "@internal/compiler/transforms/compile/css-handler/prefix/utils";

// https://github.com/Fyrd/caniuse/blob/main/features-json/background-clip-text.json
// https://github.com/postcss/autoprefixer/blob/main/data/prefixes.js#L390
export default [
	createPrefixVisitor({
		name: "background-clip",
		enter(path) {
			return prefixCSSProperty({
				path,
				propertyName: "background-clip",
				browserFeaturesKey: "background-clip-text",
			});
		},
	}),
];
