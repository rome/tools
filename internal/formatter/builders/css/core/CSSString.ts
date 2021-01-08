import {CSSString} from "@internal/ast";
import {Builder, Token} from "@internal/formatter";

export default function CSSString(builder: Builder, node: CSSString): Token {
	return JSON.stringify(node.value);
}
