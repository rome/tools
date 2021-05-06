import {
	createPrefixVisitor,
	prefixCSSProperty,
} from "@internal/compiler/transforms/compile/css-handler/prefix/utils";

// https://github.com/Fyrd/caniuse/blob/main/features-json/css3-cursors-grab.json
// https://github.com/Fyrd/caniuse/blob/main/features-json/css3-cursors-newer.json
// https://github.com/Fyrd/caniuse/blob/main/features-json/css3-cursors.json
export default ["zoom-in", "zoom-out", "grab", "grabbing"].map((value) => {
	let browserFeaturesKeySufix = "";
	if (value === "zoom-in" || value === "zoom-out") {
		browserFeaturesKeySufix = "newer";
	}
	if (value === "grab" || value === "grabbing") {
		browserFeaturesKeySufix = "grab";
	}
	return [
		createPrefixVisitor({
			name: `cursor/${value}`,
			enter(path) {
				return prefixCSSProperty({
					path,
					propertyName: "cursor",
					value,
					browserFeaturesKey: ["css3-cursors", browserFeaturesKeySufix].join(
						"-",
					),
				});
			},
		}),
	];
});
