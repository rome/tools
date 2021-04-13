import {
	createPrefixVisitor,
	prefixCSSProperty,
} from "@internal/compiler/transforms/compile/css-handler/prefix/utils";

function borderRadiusRenamer(value: string) {
	if (value.startsWith("-moz-")) {
		if (value.includes("top-left-radius")) {
			return value.replace("top-left-radius", "radius-topleft");
		}
		if (value.includes("top-right-radius")) {
			return value.replace("top-right-radius", "radius-topright");
		}
		if (value.includes("bottom-right-radius")) {
			return value.replace("bottom-right-radius", "radius-bottomright");
		}
		if (value.includes("bottom-left-radius")) {
			return value.replace("bottom-left-radius", "radius-bottomleft");
		}
	}
	return value;
}

// https://github.com/Fyrd/caniuse/blob/main/features-json/border-radius.json
// https://github.com/postcss/autoprefixer/blob/main/data/prefixes.js#L59-L70
export default [
	"border-radius",
	"border-top-left-radius",
	"border-top-right-radius",
	"border-bottom-right-radius",
	"border-bottom-left-radius",
].map((propertyName) =>
	createPrefixVisitor({
		name: propertyName,
		enter(path) {
			return prefixCSSProperty({
				path,
				propertyName,
				browserFeaturesKey: "border-radius",
				rename: borderRadiusRenamer,
			});
		},
	})
);
