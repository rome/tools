import {
	createPrefixVisitor,
	prefixCSSProperty,
} from "@internal/compiler/transforms/compile/css-handler/prefix/utils";

// https://github.com/Fyrd/caniuse/blob/main/features-json/css-clip-path.json
export default [
	createPrefixVisitor({
		name: "clip-path",
		enter(path) {
			return prefixCSSProperty({
				path,
				propertyName: "clip-path",
				browserFeaturesKey: "css-clip-path",
			});
		},
	}),
];
