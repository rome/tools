import {
	createPrefixVisitor,
	prefixCSSProperty,
} from "@internal/compiler/transforms/compile/css-handler/prefix/utils";

export default [
	createPrefixVisitor({
		name: "text-decoration",
		enter(path) {
			return prefixCSSProperty({
				path,
				propertyName: "text-decoration",
				browserFeaturesKey: "text-decoration",
			});
		},
	}),
];
