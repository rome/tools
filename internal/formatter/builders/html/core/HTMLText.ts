import {HTMLText} from "@internal/ast";
import {Builder, Token} from "@internal/formatter";

export default function HTMLText(builder: Builder, node: HTMLText): Token {
	// TODO Escape <
	return node.value;
}
