import {
	createPrefixCSSBlockVisitor,
	prefixCSSProperty,
} from "@internal/compiler/transforms/compile/css-handler/prefix/utils";

// https://github.com/Fyrd/caniuse/blob/main/features-json/css-text-orientation.json
export default [
	createPrefixCSSBlockVisitor({
		name: "text-orientation",
		enter(path) {
			return prefixCSSProperty({
				path,
				propertyName: "text-orientation",
				browserFeaturesKey: "css-text-orientation",
			});
		},
	}),
];
