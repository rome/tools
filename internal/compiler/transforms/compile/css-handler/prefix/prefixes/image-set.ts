import {
	createPrefixVisitor,
	prefixCSSValue,
} from "@internal/compiler/transforms/compile/css-handler/prefix/utils";

export default [
	[
		"background",
		"background-image",
		"border-image",
		"cursor",
		"mask",
		"mask-image",
		"list-style",
		"list-style-image",
		"content",
	].map((value) =>
		createPrefixVisitor({
			name: `image-set/${value}`,
			enter(path) {
				return prefixCSSValue({
					path,
					propertyName: "image-set",
					value,
					browserFeaturesKey: "css-image-set",
				});
			},
		})
	),
];
