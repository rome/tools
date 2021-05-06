import {
	createPrefixVisitor,
	prefixCSSProperty,
} from "@internal/compiler/transforms/compile/css-handler/prefix/utils";

// https://github.com/Fyrd/caniuse/blob/main/features-json/css-appearance.json
export default [
	createPrefixVisitor({
		name: "appearance",
		enter(path) {
			return prefixCSSProperty({
				path,
				propertyName: "appearance",
				browserFeaturesKey: "css-appearance",
			});
		},
	}),
];
