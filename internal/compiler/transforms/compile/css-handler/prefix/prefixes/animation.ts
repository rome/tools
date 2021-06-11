import {
	createPrefixCSSBlockVisitor,
	prefixCSSProperty,
} from "@internal/compiler/transforms/compile/css-handler/prefix/utils";

// https://github.com/Fyrd/caniuse/blob/main/features-json/css-animation.json
// https://github.com/postcss/autoprefixer/blob/main/data/prefixes.js#L86-L102
export default [
	"animation",
	"animation-name",
	"animation-duration",
	"animation-delay",
	"animation-direction",
	"animation-fill-mode",
	"animation-iteration-count",
	"animation-play-state",
	"animation-timing-function",
].map((propertyName) =>
	createPrefixCSSBlockVisitor({
		name: propertyName,
		enter(path) {
			return prefixCSSProperty({
				path,
				propertyName,
				browserFeaturesKey: "css-animation",
			});
		},
	})
);
