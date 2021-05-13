import {
	createPrefixVisitor,
	prefixCSSProperty,
} from "@internal/compiler/transforms/compile/css-handler/prefix/utils";

function flexRenamer(value: string) {
	return value.startsWith("-moz-") ? value.replace("flex", "box") : value;
}

// https://github.com/Fyrd/caniuse/blob/main/features-json/flexbox.json
export default ["flex", "flex-grow", "flex-shrink", "flex-basis"].map((property) =>
	createPrefixVisitor({
		name: property,
		enter(path) {
			return prefixCSSProperty({
				path,
				propertyName: property,
				browserFeaturesKey: "flexbox",
				rename: flexRenamer,
			});
		},
	})
);
