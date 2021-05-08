import {
	createPrefixVisitor,
	prefixCSSValue,
} from "@internal/compiler/transforms/compile/css-handler/prefix/utils";

// https://github.com/Fyrd/caniuse/blob/main/features-json/css-gradients.json
export default [
	...[
		"background",
		"background-image",
		"border-image",
		"mask",
		"list-style",
		"list-style-image",
		"content",
		"mask-image",
	].map((propertyName) => {
		return createPrefixVisitor({
			name: `${propertyName}/linear-gradient`,
			enter(path) {
				return prefixCSSValue({
					path,
					propertyName,
					value: "linear-gradient",
					browserFeaturesKey: "css-gradients",
				});
			},
		});
	}),

	...[
		"background",
		"background-image",
		"border-image",
		"mask",
		"list-style",
		"list-style-image",
		"content",
		"mask-image",
	].map((propertyName) => {
		return createPrefixVisitor({
			name: `${propertyName}/repeating-linear-gradient`,
			enter(path) {
				return prefixCSSValue({
					path,
					propertyName,
					value: "repeating-linear-gradient",
					browserFeaturesKey: "css-gradients",
				});
			},
		});
	}),

	...[
		"background",
		"background-image",
		"border-image",
		"mask",
		"list-style",
		"list-style-image",
		"content",
		"mask-image",
	].map((propertyName) => {
		return createPrefixVisitor({
			name: `${propertyName}/radial-gradient`,
			enter(path) {
				return prefixCSSValue({
					path,
					propertyName,
					value: "radial-gradient",
					browserFeaturesKey: "css-gradients",
				});
			},
		});
	}),

	...[
		"background",
		"background-image",
		"border-image",
		"mask",
		"list-style",
		"list-style-image",
		"content",
		"mask-image",
	].map((propertyName) => {
		return createPrefixVisitor({
			name: `${propertyName}/repeating-radial-gradient`,
			enter(path) {
				return prefixCSSValue({
					path,
					propertyName,
					value: "repeating-radial-gradient",
					browserFeaturesKey: "css-gradients",
				});
			},
		});
	}),
];
