import {
	createPrefixCSSBlockVisitor,
	prefixCSSProperty,
} from "@internal/compiler/transforms/compile/css-handler/prefix/utils";

// https://github.com/Fyrd/caniuse/blob/main/features-json/css-boxshadow.json
export default [
	createPrefixCSSBlockVisitor({
		name: "box-shadow",
		enter(path) {
			return prefixCSSProperty({
				path,
				propertyName: "box-shadow",
				browserFeaturesKey: "css-boxshadow",
			});
		},
	}),
];
