import {HTMLString} from "@romejs/ast";
import {Builder, Token} from "@romejs/formatter";

export default function HTMLString(builder: Builder, node: HTMLString): Token {
	return JSON.stringify(node.value);
}
