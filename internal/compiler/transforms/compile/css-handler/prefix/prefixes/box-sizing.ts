import {
	createPrefixVisitor,
	prefixCSSProperty,
} from "@internal/compiler/transforms/compile/css-handler/prefix/utils";

// https://github.com/Fyrd/caniuse/blob/main/features-json/css3-boxsizing.json
// https://github.com/postcss/autoprefixer/blob/main/data/prefixes.js#L209-L211
export default [
	createPrefixVisitor({
		name: "box-sizing",
		enter(path) {
			return prefixCSSProperty({
				path,
				propertyName: "box-sizing",
				browserFeaturesKey: "css3-boxsizing",
			});
		},
	}),
];
