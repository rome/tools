import {
	createPrefixVisitor,
	prefixCSSProperty,
} from "@internal/compiler/transforms/compile/css-handler/prefix/utils";

// https://github.com/Fyrd/caniuse/blob/main/features-json/text-overflow.json
export default [
	createPrefixVisitor({
		name: "text-overflow",
		enter(path) {
			return prefixCSSProperty({
				path,
				propertyName: "text-overflow",
				browserFeaturesKey: "text-overflow",
			});
		},
	}),
];
