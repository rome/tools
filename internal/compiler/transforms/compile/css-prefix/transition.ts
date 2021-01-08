import {createVisitor, signals} from "@internal/compiler";
import {
	CSSBlock,
	CSSDeclaration,
	cssBlock,
	cssDeclaration,
} from "@internal/ast";

const PREFIXES = new Set(["-webkit-", "-o-"]);

function nodeHasPrefix(
	node: CSSBlock,
	prefix: string,
): undefined | CSSDeclaration {
	if (node.value) {
		return node.value.find((n) =>
			n.type === "CSSDeclaration" && n.name.includes(prefix)
		) as CSSDeclaration;
	}
	return undefined;
}

export default createVisitor({
	name: "css-prefix",
	enter(path) {
		const {node} = path;
		if (node.type === "CSSBlock") {
			if (node.value && node.value.length > 0) {
				const transitionIndex = node.value.findIndex((n) =>
					n.type === "CSSDeclaration" && n.name === "transition"
				);
				if (transitionIndex > -1) {
					const transition = node.value[transitionIndex] as CSSDeclaration;
					const newDeclarations = [];
					for (const prefix of PREFIXES) {
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
