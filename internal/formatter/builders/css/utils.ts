import {AnyNode} from "@internal/ast";
import {Builder, Token, concat, join, space} from "@internal/formatter";

export function printCommaList(
	builder: Builder,
	nodes: AnyNode[],
	parent: AnyNode,
) {
	const commaSeperatedTokens: Token[] = [];

	const list = [...nodes];
	while (list.length) {
		const commaIndex = list.findIndex((value) => value?.type === "CSSComma");
		const values = list.splice(0, commaIndex > 0 ? commaIndex : nodes.length);
		commaSeperatedTokens.push(
			join(space, values.map((value) => builder.tokenize(value, parent))),
		);
		list.splice(0, 1);
	}
	return join(concat([",", space]), commaSeperatedTokens);
}
