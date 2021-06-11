import {
	createPrefixCSSBlockVisitor,
	prefixCSSProperty,
} from "@internal/compiler/transforms/compile/css-handler/prefix/utils";

// https://github.com/Fyrd/caniuse/blob/main/features-json/css-masks.json
export default [
	"mask-clip",
	"mask-composite",
	"mask-image",
	"mask-origin",
	"mask-repeat",
	"mask-border-repeat",
	"mask-border-source",
	"mask",
	"mask-position",
	"mask-size",
	"mask-border",
	"mask-border-outset",
	"mask-border-width",
	"mask-border-slice",
].map((propertyName) =>
	createPrefixCSSBlockVisitor({
		name: "mask",
		enter(path) {
			return prefixCSSProperty({
				path,
				propertyName,
				browserFeaturesKey: "css-masks",
			});
		},
	})
);
