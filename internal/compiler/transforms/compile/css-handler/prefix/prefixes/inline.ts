import {
	createPrefixVisitor,
	prefixCSSProperty,
} from "@internal/compiler/transforms/compile/css-handler/prefix/utils";

// https://github.com/Fyrd/caniuse/blob/main/features-json/css-logical-props.json
export default [
	"border-inline-start",
	"border-inline-end",
	"margin-inline-start",
	"margin-inline-end",
	"padding-inline-start",
	"padding-inline-end",
].map((propertyName) =>
	createPrefixVisitor({
		name: "inline",
		enter(path) {
			return prefixCSSProperty({
				path,
				propertyName,
				browserFeaturesKey: "css-logical-props",
			});
		},
	})
);
