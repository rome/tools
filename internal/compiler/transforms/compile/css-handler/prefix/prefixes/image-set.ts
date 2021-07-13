import {
	createPrefixCSSBlockVisitor,
	prefixCSSValue,
} from "@internal/compiler/transforms/compile/css-handler/prefix/utils";

// https://github.com/Fyrd/caniuse/blob/main/features-json/css-image-set.json
export default [
	"background",
	"background-image",
	"border-image",
	"cursor",
	"mask",
	"mask-image",
	"list-style",
	"list-style-image",
	"content",
].map((propertyName) =>
	createPrefixCSSBlockVisitor({
		name: `${propertyName}/image-set`,
		enter(path) {
			return prefixCSSValue({
				path,
				propertyName,
				value: "image-set",
				browserFeaturesKey: "css-image-set",
			});
		},
	})
);
