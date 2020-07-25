import {HTMLText} from "@romefrontend/ast";
import {Builder, Token} from "@romefrontend/formatter";

export default function HTMLText(builder: Builder, node: HTMLText): Token {
	// TODO Escape <
	return node.value;
}
