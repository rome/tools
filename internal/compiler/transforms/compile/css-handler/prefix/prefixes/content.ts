import {
	createPrefixVisitor,
	prefixCSSProperty,
} from "@internal/compiler/transforms/compile/css-handler/prefix/utils";

// https://github.com/Fyrd/caniuse/blob/main/features-json/css-content-visibility.json
export default [
	createPrefixVisitor({
		name: "content",
		enter(path) {
			return prefixCSSProperty({
				path,
				propertyName: "content",
				browserFeaturesKey: "css-content-visibility",
			});
		},
	}),
];
