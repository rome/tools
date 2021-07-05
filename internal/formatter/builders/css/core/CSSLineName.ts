import {CSSLineName} from "@internal/ast";
import {Builder, concat, Token} from "@internal/formatter";

export default function CSSRaw(builder: Builder, node: CSSLineName): Token {
	return concat([
		"[",
		node.value,
		"]",
	]);
}
