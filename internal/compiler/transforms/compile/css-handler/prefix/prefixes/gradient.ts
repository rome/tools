import {
	createPrefixVisitor,
	prefixCSSValue,
} from "@internal/compiler/transforms/compile/css-handler/prefix/utils";

// https://github.com/Fyrd/caniuse/blob/main/features-json/css-gradients.json
export default [
	"background",
	"background-image",
	"border-image",
	"mask",
	"list-style",
	"list-style-image",
	"content",
	"mask-image",
].flatMap((propertyName) =>
	[
		"linear-gradient",
		"repeating-linear-gradient",
		"radial-gradient",
		"repeating-radial-gradient",
	].map((value) =>
		createPrefixVisitor({
			name: `${propertyName}/${value}`,
			enter(path) {
				return prefixCSSValue({
					path,
					propertyName,
					value,
					browserFeaturesKey: "css-gradients",
				});
			},
		})
	)
);
