import {HTMLString} from "@internal/ast";
import {Builder, Token} from "@internal/formatter";

export default function HTMLString(builder: Builder, node: HTMLString): Token {
	return JSON.stringify(node.value);
}
