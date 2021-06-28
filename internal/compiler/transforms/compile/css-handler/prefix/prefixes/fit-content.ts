import {
	createPrefixCSSBlockVisitor,
	prefixCSSProperty,
} from "@internal/compiler/transforms/compile/css-handler/prefix/utils";

export default [
	createPrefixCSSBlockVisitor({
		name: "fit-content",
		enter(path) {
			return prefixCSSProperty({
				path,
				propertyName: "fit-content",
				browserFeaturesKey: "",
			});
		},
	}),
];
