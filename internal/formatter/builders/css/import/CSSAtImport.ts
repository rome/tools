import {CSSAtImport} from "@internal/ast";
import {Builder, Token, concat} from "@internal/formatter";

export default function CSSAtImport(builder: Builder, node: CSSAtImport): Token {
	return concat([builder.tokenize(node.value, node), ";"]);
}
