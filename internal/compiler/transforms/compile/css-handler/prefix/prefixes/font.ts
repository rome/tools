import {
	createPrefixCSSBlockVisitor,
	prefixCSSProperty,
} from "@internal/compiler/transforms/compile/css-handler/prefix/utils";

export default [
	// https://github.com/Fyrd/caniuse/blob/main/features-json/font-feature.json
	...[
		"font-feature-settings",
		"font-variant-ligatures",
		"font-language-override",
	].map((propertyName) =>
		createPrefixCSSBlockVisitor({
			name: propertyName,
			enter(path) {
				return prefixCSSProperty({
					path,
					propertyName,
					browserFeaturesKey: "font-feature",
				});
			},
		})
	),
	// https://github.com/Fyrd/caniuse/blob/main/features-json/font-kerning.json
	createPrefixCSSBlockVisitor({
		name: "font-kerning",
		enter(path) {
			return prefixCSSProperty({
				path,
				propertyName: "font-kerning",
				browserFeaturesKey: "font-kerning",
			});
		},
	}),
];
