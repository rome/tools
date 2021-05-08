import {
	createPrefixVisitor,
	prefixCSSProperty,
} from "@internal/compiler/transforms/compile/css-handler/prefix/utils";

function printRenamer(value: string) {
	return value.startsWith("-webkit-") ? value.replace("-webkit-", "-webkit-print-") : value;
}

// https://github.com/Fyrd/caniuse/blob/main/features-json/css-color-adjust.json
export default [
	createPrefixVisitor({
		name: "color-adjust",
		enter(path) {
			return prefixCSSProperty({
				path,
				propertyName: "color-adjust",
				browserFeaturesKey: "css-color-adjust",
				rename: printRenamer,
			});
		},
	}),
];
