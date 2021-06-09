import {CSSAtImport} from "@internal/ast";
import {Builder, Token} from "@internal/formatter";

export default function CSSAtImport(builder: Builder, node: CSSAtImport): Token {
	return JSON.stringify(node.value);
}
