import {
	createPrefixVisitor,
	prefixCSSProperty,
} from "@internal/compiler/transforms/compile/css-handler/prefix/utils";

// https://github.com/Fyrd/caniuse/blob/main/features-json/css-grid.json
export default ["grid-template-rows", "grid-template-columns"].map((name) =>
	createPrefixVisitor({
		name,
		enter(path) {
			return prefixCSSProperty({
				path,
				propertyName: name,
				browserFeaturesKey: "css-grid",
			});
		},
	})
);
