import {
	PrefixConfig,
	createPrefixVisitor,
	matchBrowser,
	prefixCSSValue,
} from "@internal/compiler/transforms/compile/css-handler/prefix/utils";

// https://caniuse.com/mdn-css_properties_display_flex
// https://caniuse.com/mdn-css_properties_display_inline-flex
// https://ptb2.me/flexbox/
function getFlexPrefixes(prefixConfig: PrefixConfig) {
	const prefixes = new Set<string>();
	if (
		matchBrowser(prefixConfig.target, "chrome 21-28") ||
		matchBrowser(prefixConfig.target, "safari 6.1-8") ||
		matchBrowser(prefixConfig.target, "ios 7-8.4") ||
		matchBrowser(prefixConfig.target, "opera 15")
	) {
		prefixes.add("-webkit-");
	}

	return prefixes;
}

export default [
	createPrefixVisitor({
		name: "display/flex",
		enter(path, prefixConfig) {
			return prefixCSSValue(
				path,
				"display",
				"flex",
				() => getFlexPrefixes(prefixConfig),
			);
		},
	}),
	createPrefixVisitor({
		name: "display/inline-flex",
		enter(path, prefixConfig) {
			return prefixCSSValue(
				path,
				"display",
				"inline-flex",
				() => getFlexPrefixes(prefixConfig),
			);
		},
	}),
];
