import {
	createPrefixVisitor,
	prefixCSSProperty,
} from "@internal/compiler/transforms/compile/css-handler/prefix/utils";

export default [
	createPrefixVisitor({
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
