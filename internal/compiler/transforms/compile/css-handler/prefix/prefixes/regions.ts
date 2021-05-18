import {
	createPrefixVisitor,
	prefixCSSProperty,
} from "@internal/compiler/transforms/compile/css-handler/prefix/utils";

// https://github.com/Fyrd/caniuse/blob/main/features-json/css-regions.json
export default ["flow-into", "flow-from", "region-fragment"].map((propertyName) =>
	createPrefixVisitor({
		name: propertyName,
		enter(path) {
			return prefixCSSProperty({
				path,
				propertyName,
				browserFeaturesKey: "css-regions",
			});
		},
	})
);
