import {
	createPrefixCSSBlockVisitor,
	prefixCSSProperty,
} from "@internal/compiler/transforms/compile/css-handler/prefix/utils";

// https://github.com/Fyrd/caniuse/blob/main/features-json/user-select-none.json
// https://github.com/postcss/autoprefixer/blob/main/data/prefixes.js#L299-L302
export default [
	createPrefixCSSBlockVisitor({
		name: "user-select",
		enter(path) {
			return prefixCSSProperty({
				path,
				propertyName: "user-select",
				browserFeaturesKey: "user-select-none",
			});
		},
	}),
];
