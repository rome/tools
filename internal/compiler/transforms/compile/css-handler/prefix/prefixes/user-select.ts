import {
	createPrefixVisitor,
	prefixCSSProperty,
} from "@internal/compiler/transforms/compile/css-handler/prefix/utils";

// https://github.com/Fyrd/caniuse/blob/main/features-json/user-select-none.json
export default ["auto", "none", "text", "all"].map((value) => [
	createPrefixVisitor({
		name: `user-select/${value}`,
		enter(path) {
			return prefixCSSProperty({
				path,
				propertyName: "user-select",
				name,
				browserFeaturesKey: "user-select",
			});
		},
	}),
]);
