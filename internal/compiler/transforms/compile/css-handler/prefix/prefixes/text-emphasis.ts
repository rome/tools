import {
	createPrefixVisitor,
	prefixCSSProperty,
} from "@internal/compiler/transforms/compile/css-handler/prefix/utils";

// https://github.com/Fyrd/caniuse/blob/main/features-json/text-emphasis.json
export default [
	"text-emphasis",
	"text-emphasis-position",
	"text-emphasis-style",
	"text-emphasis-color",
].map((propertyName) =>
	createPrefixVisitor({
		name: "text-emphasis",
		enter(path) {
			return prefixCSSProperty({
				path,
				propertyName,
				browserFeaturesKey: "text-emphasis",
			});
		},
	})
);
