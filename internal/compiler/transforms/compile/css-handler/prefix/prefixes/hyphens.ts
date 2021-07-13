import {
	createPrefixCSSBlockVisitor,
	prefixCSSProperty,
} from "@internal/compiler/transforms/compile/css-handler/prefix/utils";

// https://github.com/Fyrd/caniuse/blob/main/features-json/css-hyphens.json
export default [
	createPrefixCSSBlockVisitor({
		name: "hyphens",
		enter(path) {
			return prefixCSSProperty({
				path,
				propertyName: "hyphens",
				browserFeaturesKey: "css-hyphens",
			});
		},
	}),
];
