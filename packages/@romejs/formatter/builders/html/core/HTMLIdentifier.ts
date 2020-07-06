import {HTMLIdentifier} from "@romejs/ast";
import {Builder, Token} from "@romejs/formatter";

export default function HTMLIdentifier(
	builder: Builder,
	node: HTMLIdentifier,
): Token {
	return node.name;
}
