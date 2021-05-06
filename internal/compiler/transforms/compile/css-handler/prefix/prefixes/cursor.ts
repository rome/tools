import {
	createPrefixVisitor,
	prefixCSSProperty,
} from "@internal/compiler/transforms/compile/css-handler/prefix/utils";

// https://github.com/Fyrd/caniuse/blob/main/features-json/css3-cursors-grab.json
// https://github.com/Fyrd/caniuse/blob/main/features-json/css3-cursors-newer.json
// https://github.com/Fyrd/caniuse/blob/main/features-json/css3-cursors.json
export default [
	createPrefixVisitor({
		name: "cursor",
		enter(path) {
			return prefixCSSProperty({
				path,
				propertyName: "cursor",
				browserFeaturesKey: "css3-cursors",
			});
		},
	}),
];
