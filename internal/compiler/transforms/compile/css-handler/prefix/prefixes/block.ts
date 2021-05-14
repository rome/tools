import {
	createPrefixVisitor,
	prefixCSSProperty,
} from "@internal/compiler/transforms/compile/css-handler/prefix/utils";

export default [
	createPrefixVisitor({
		name: "block",
		enter(path) {
			return prefixCSSProperty({
				path,
				propertyName: "block",
				browserFeaturesKey: "",
			});
		},
	}),
];
