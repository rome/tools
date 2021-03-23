import {
	createPrefixVisitor,
	prefixCSSProperty,
} from "@internal/compiler/transforms/compile/css-handler/prefix/utils";

// https://github.com/Fyrd/caniuse/blob/main/features-json/transforms2d.json
export default [
	createPrefixVisitor({
		name: "transform",
		enter(path) {
			return prefixCSSProperty({
				path,
				propertyName: "transform",
				browserFeaturesKey: "transforms2d",
			});
		},
	}),
];
