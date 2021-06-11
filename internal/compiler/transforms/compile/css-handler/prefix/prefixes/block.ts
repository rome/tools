import {
	createPrefixCSSBlockVisitor,
	prefixCSSProperty,
} from "@internal/compiler/transforms/compile/css-handler/prefix/utils";

// https://github.com/Fyrd/caniuse/blob/main/features-json/css-logical-props.json
export default [
	"border-block-start",
	"border-block-end",
	"margin-block-start",
	"margin-block-end",
	"padding-block-start",
	"padding-block-end",
].map((propertyName) =>
	createPrefixCSSBlockVisitor({
		name: "block",
		enter(path) {
			return prefixCSSProperty({
				path,
				propertyName,
				browserFeaturesKey: "css-logical-props",
			});
		},
	})
);
