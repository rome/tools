import {CSSHash} from "@internal/ast";
import {Builder, Token, concat} from "@internal/formatter";

export default function CSSHash(builder: Builder, node: CSSHash): Token {
	return concat(["#", node.value]);
}
