import {
	createPrefixVisitor,
	prefixCSSProperty,
} from "@internal/compiler/transforms/compile/css-handler/prefix/utils";

// https://github.com/Fyrd/caniuse/blob/main/features-json/css-text-align-last.json
export default [
	createPrefixVisitor({
		name: "text-align-last",
		enter(path) {
			return prefixCSSProperty({
				path,
				propertyName: "text-align-last",
				browserFeaturesKey: "css-text-align-last",
			});
		},
	}),
];
