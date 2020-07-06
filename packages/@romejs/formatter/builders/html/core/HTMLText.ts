import {HTMLText} from "@romejs/ast";
import {Builder, Token} from "@romejs/formatter";

export default function HTMLText(builder: Builder, node: HTMLText): Token {
	// TODO Escape <
	return node.value;
}
