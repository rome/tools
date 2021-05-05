import {
	createPrefixVisitor,
	prefixCSSProperty,
} from "@internal/compiler/transforms/compile/css-handler/prefix/utils";

// https://github.com/Fyrd/caniuse/blob/main/features-json/user-select-none.json
export default [
	createPrefixVisitor({
		name: "user-select/none",
		enter(path) {
			return prefixCSSProperty({
				path,
				propertyName: "user-select",
				value: "none",
				browserFeaturesKey: "css-none",
			});
		},
	}),
];
