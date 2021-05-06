import {
	createPrefixVisitor,
	prefixCSSProperty,
} from "@internal/compiler/transforms/compile/css-handler/prefix/utils";

// https://github.com/Fyrd/caniuse/blob/main/features-json/css-writing-mode.json
export default [
	createPrefixVisitor({
		name: "writing-mode",
		enter(path) {
			return prefixCSSProperty({
				path,
				propertyName: "writing-mode",
				browserFeaturesKey: "css-writing-mode",
			});
		},
	}),
];
