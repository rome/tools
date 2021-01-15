import {
	PrefixConfig,
	createPrefixVisitor,
	matchBrowser,
	prefixCSSProperty,
} from "@internal/compiler/transforms/compile/css-handler/prefix/utils";

// https://caniuse.com/?search=transform
function getPrefixes(prefixConfig: PrefixConfig) {
	const prefixes = new Set<string>();
	if (matchBrowser(prefixConfig.target, "firefox 3.5-15")) {
		prefixes.add("-moz-");
	}

	if (
		matchBrowser(prefixConfig.target, "chrome 4-35") ||
		matchBrowser(prefixConfig.target, "safari 3.1-8") ||
		matchBrowser(prefixConfig.target, "ios 3.2-8.4") ||
		matchBrowser(prefixConfig.target, "opera 15-22") ||
		matchBrowser(prefixConfig.target, "android 2.1-4.4.4")
	) {
		prefixes.add("-webkit-");
	}

	if (matchBrowser(prefixConfig.target, "opera 11.5")) {
		prefixes.add("-o-");
	}

	return prefixes;
}

export default createPrefixVisitor({
	name: "transform",
	enter(path, prefixConfig) {
		return prefixCSSProperty(path, "transform", () => getPrefixes(prefixConfig));
	},
});
