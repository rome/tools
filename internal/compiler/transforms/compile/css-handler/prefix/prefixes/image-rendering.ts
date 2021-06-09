import {
	createPrefixVisitor,
	prefixCSSValue,
} from "@internal/compiler/transforms/compile/css-handler/prefix/utils";

function pixelatedRenamer(value: string) {
	if (value.startsWith("-webkit-")) {
		return value.replace("pixelated", "optimize-contrast");
	}
	if (value.startsWith("-moz-")) {
		return value.replace("pixelated", "crisp-edges");
	}
	return value;
}

// https://github.com/Fyrd/caniuse/blob/main/features-json/css-crisp-edges.json
export default [
	createPrefixVisitor({
		name: "image-rendering/pixelated",
		enter(path) {
			return prefixCSSValue({
				path,
				propertyName: "image-rendering",
				value: "pixelated",
				browserFeaturesKey: "css-crisp-edges",
				rename: pixelatedRenamer,
			});
		},
	}),
];
