import {
	createPrefixVisitor,
	prefixCSSProperty,
} from "@internal/compiler/transforms/compile/css-handler/prefix/utils";

export default [
	createPrefixVisitor({
		name: "scroll-snap-type",
		enter(path) {
			return prefixCSSProperty({
				path,
				propertyName: "scroll-snap-type",
				browserFeaturesKey: "css-snappoints",
			});
		},
	}),
];
