import {
	createPrefixVisitor,
	prefixCSSValue,
} from "@internal/compiler/transforms/compile/css-handler/prefix/utils";

function flexRenamer(value: string) {
	return value.startsWith("-moz-") ? value.replace("flex", "box") : value;
}

// https://github.com/Fyrd/caniuse/blob/main/features-json/flexbox.json
export default ["flex", "inline-flex"].map((value) =>
	createPrefixVisitor({
		name: `display/${value}`,
		enter(path) {
			return prefixCSSValue({
				path,
				propertyName: "display",
				value,
				browserFeaturesKey: "flexbox",
				rename: flexRenamer,
			});
		},
	})
);
