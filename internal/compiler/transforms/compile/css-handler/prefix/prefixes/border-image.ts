import {
	createPrefixCSSBlockVisitor,
	prefixCSSProperty,
} from "@internal/compiler/transforms/compile/css-handler/prefix/utils";

// https://github.com/Fyrd/caniuse/blob/main/features-json/border-image.json
export default [
	createPrefixCSSBlockVisitor({
		name: "border-image",
		enter(path) {
			return prefixCSSProperty({
				path,
				propertyName: "border-image",
				browserFeaturesKey: "border-image",
			});
		},
	}),
];
