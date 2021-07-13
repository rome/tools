import {CSSLineName} from "@internal/ast";
import {Builder, Token, concat} from "@internal/formatter";

export default function CSSLineName(builder: Builder, node: CSSLineName): Token {
	return concat(["[", node.value, "]"]);
}
