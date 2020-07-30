import {HTMLIdentifier} from "@romefrontend/ast";
import {Builder, Token} from "@romefrontend/formatter";

export default function HTMLIdentifier(
	builder: Builder,
	node: HTMLIdentifier,
): Token {
	return node.name;
}
