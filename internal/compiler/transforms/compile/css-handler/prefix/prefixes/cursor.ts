import {
	createPrefixVisitor,
	prefixCSSValue,
} from "@internal/compiler/transforms/compile/css-handler/prefix/utils";

export default [
	// https://github.com/Fyrd/caniuse/blob/main/features-json/css3-cursors-newer.json
	...["zoom-in", "zoom-out"].map((value) =>
		createPrefixVisitor({
			name: `cursor/${value}`,
			enter(path) {
				return prefixCSSValue({
					path,
					propertyName: "cursor",
					value,
					browserFeaturesKey: "css3-cursors-newer",
				});
			},
		})
	),
	// https://github.com/Fyrd/caniuse/blob/main/features-json/css3-cursors-grab.json
	...["grab", "grabbing"].map((value) =>
		createPrefixVisitor({
			name: `cursor/${value}`,
			enter(path) {
				return prefixCSSValue({
					path,
					propertyName: "cursor",
					value,
					browserFeaturesKey: "css3-cursors-grab",
				});
			},
		})
	),
];
