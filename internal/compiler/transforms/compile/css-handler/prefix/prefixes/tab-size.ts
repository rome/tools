import {
	createPrefixCSSBlockVisitor,
	prefixCSSProperty,
} from "@internal/compiler/transforms/compile/css-handler/prefix/utils";

// https://github.com/Fyrd/caniuse/blob/main/features-json/css3-tabsize.json
export default [
	createPrefixCSSBlockVisitor({
		name: "tab-size",
		enter(path) {
			return prefixCSSProperty({
				path,
				propertyName: "tab-size",
				browserFeaturesKey: "css3-tabsize",
			});
		},
	}),
];
