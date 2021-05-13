import {
	createPrefixVisitor,
	prefixCSSProperty,
} from "@internal/compiler/transforms/compile/css-handler/prefix/utils";

export default [
	createPrefixVisitor({
		name: "inline",
		enter(path) {
			return prefixCSSProperty({
				path,
				propertyName: "inline",
				browserFeaturesKey: "",
			});
		},
	}),
];
