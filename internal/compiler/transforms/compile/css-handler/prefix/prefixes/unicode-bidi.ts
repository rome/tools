import {
	createPrefixVisitor,
	prefixCSSValue,
} from "@internal/compiler/transforms/compile/css-handler/prefix/utils";

// https://github.com/Fyrd/caniuse/blob/main/features-json/css-unicode-bidi.json
export default ["isolate", "plaintext", "isolate-override"].map((
	propertyName,
	propertyValue,
) =>
	createPrefixVisitor({
		name: `unicode-bidi/${propertyValue}`,
		enter(path) {
			return prefixCSSValue({
				path,
				propertyName,
				value: "unicode-bidi",
				browserFeaturesKey: "css-unicode-bidi",
			});
		},
	})
);
