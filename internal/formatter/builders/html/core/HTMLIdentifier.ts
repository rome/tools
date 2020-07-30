import {HTMLIdentifier} from "@internal/ast";
import {Builder, Token} from "@internal/formatter";

export default function HTMLIdentifier(
	builder: Builder,
	node: HTMLIdentifier,
): Token {
	return node.name;
}
