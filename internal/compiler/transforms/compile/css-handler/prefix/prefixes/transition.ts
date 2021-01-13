import {signals} from "@internal/compiler";
import {
	CSSDeclaration,
	cssBlock,
	cssDeclaration,
} from "@internal/ast";
import {
	createPrefixVisitor,
	matchBrowser,
	nodeHasPrefix
} from "@internal/compiler/transforms/compile/css-handler/prefix/utils";


export default createPrefixVisitor({
	name: "css-prefix",
	enter(path, prefixConfig) {

		const prefixes = new Set<string>(); // https://caniuse.com/?search=transition
		if (matchBrowser(prefixConfig.target, "firefox 4-15")) prefixes.add("-moz-");

		if (matchBrowser(prefixConfig.target, "chrome 4-25")
			|| matchBrowser(prefixConfig.target, "safari 3.1-8")
			|| matchBrowser(prefixConfig.target, "ios 3.2-8.4")
			|| matchBrowser(prefixConfig.target, "android 2.1-4.4.4")) prefixes.add("-webkit-");

		if (matchBrowser(prefixConfig.target, "opera 10.5")
			|| matchBrowser(prefixConfig.target, "op_mob 12")) prefixes.add("-o-");

		const {node} = path;
		if (node.type === "CSSBlock") {
			if (node.value && node.value.length > 0) {
				const transitionIndex = node.value.findIndex((n) =>
					n.type === "CSSDeclaration" && n.name === "transition"
				);
				if (transitionIndex > -1) {
					const transition = node.value[transitionIndex] as CSSDeclaration;
					const newDeclarations = [];
					for (const prefix of prefixes) {
						const hasPrefix = nodeHasPrefix(node, prefix);
						if (!hasPrefix) {
							newDeclarations.push(
								cssDeclaration.create({
									name: `${prefix}${transition.name}`,
									value: transition.value,
									important: transition.important,
								}),
							);
						}
					}
					if (newDeclarations.length > 0) {
						const block = cssBlock.create({
							...node,
							value: [
								...node.value.slice(0, transitionIndex),
								transition,
								...newDeclarations,
								...node.value.slice(transitionIndex + 1, node.value.length),
							],
						});
						return signals.replace(block);
					}
				}
			}
		}

		return signals.retain;
	},
});
