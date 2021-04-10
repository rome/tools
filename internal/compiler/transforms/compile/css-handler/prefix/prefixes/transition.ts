import {
	createPrefixVisitor,
	prefixCSSProperty,
} from "@internal/compiler/transforms/compile/css-handler/prefix/utils";

// https://github.com/Fyrd/caniuse/blob/main/features-json/css-transitions.json
// https://github.com/postcss/autoprefixer/blob/main/data/prefixes.js#L110-L122
export default [
	"transition",
	"transition-property",
	"transition-duration",
	"transition-delay",
	"transition-timing-function",
].map((propertyName) =>
	createPrefixVisitor({
		name: propertyName,
		enter(path) {
			return prefixCSSProperty({
				path,
				propertyName,
				browserFeaturesKey: "css-transitions",
			});
		},
	})
);
