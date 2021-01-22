import {CSSFunction} from "@internal/ast";
import {Builder, Token, concat} from "@internal/formatter";
import {printCommaList} from "../utils";

export default function CSSFunction(builder: Builder, node: CSSFunction): Token {
	return concat([
		node.name,
		"(",
		printCommaList(builder, node.params, node),
		")",
	]);
}
