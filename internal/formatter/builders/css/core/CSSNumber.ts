import {CSSNumber} from "@internal/ast";
import {Builder, Token} from "@internal/formatter";

const SCIENTIFIC_NOTATION = /e/i;

export default function CSSNumber(builder: Builder, node: CSSNumber): Token {
	if (SCIENTIFIC_NOTATION.test(node.raw)) {
		return node.raw;
	}
	return String(node.value);
}
