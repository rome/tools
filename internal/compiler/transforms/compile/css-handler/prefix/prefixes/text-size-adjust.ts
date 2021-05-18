import {
	createPrefixVisitor,
	prefixCSSProperty,
} from "@internal/compiler/transforms/compile/css-handler/prefix/utils";

// https://github.com/Fyrd/caniuse/blob/main/features-json/text-size-adjust.json
export default [
	createPrefixVisitor({
		name: "text-size-adjust",
		enter(path) {
			return prefixCSSProperty({
				path,
				propertyName: "text-size-adjust",
				browserFeaturesKey: "text-size-adjust",
			});
		},
	}),
];
