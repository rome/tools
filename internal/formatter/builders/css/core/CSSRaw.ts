import {CSSRaw} from "@internal/ast";
import {Builder, Token} from "@internal/formatter";

export default function CSSRaw(builder: Builder, node: CSSRaw): Token {
	return node.value;
}
