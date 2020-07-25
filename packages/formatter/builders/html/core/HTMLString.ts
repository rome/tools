import {HTMLString} from "@romefrontend/ast";
import {Builder, Token} from "@romefrontend/formatter";

export default function HTMLString(builder: Builder, node: HTMLString): Token {
	return JSON.stringify(node.value);
}
