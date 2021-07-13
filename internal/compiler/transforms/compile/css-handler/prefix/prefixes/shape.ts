import {
	createPrefixCSSBlockVisitor,
	prefixCSSProperty,
} from "@internal/compiler/transforms/compile/css-handler/prefix/utils";

// https://github.com/Fyrd/caniuse/blob/main/features-json/css-shapes.json
export default ["shape-margin", "shape-outside", "shape-image-threshold"].map((
	propertyName,
) =>
	createPrefixCSSBlockVisitor({
		name: propertyName,
		enter(path) {
			return prefixCSSProperty({
				path,
				propertyName,
				browserFeaturesKey: "css-shapes",
			});
		},
	})
);
