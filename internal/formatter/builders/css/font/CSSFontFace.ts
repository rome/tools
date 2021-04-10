import {CSSFontFace} from "@internal/ast";
import {Builder, Token} from "@internal/formatter";

export default function CSSFontFace(builder: Builder, node: CSSFontFace): Token {
	return builder.tokenize(node.value, node);
}
