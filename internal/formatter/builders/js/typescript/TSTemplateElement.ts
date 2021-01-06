import {Builder, Token} from "@internal/formatter";
import {TSTemplateElement} from "@internal/ast";

export default function TSTemplateElement(
	builder: Builder,
	node: TSTemplateElement,
): Token {
	return node.raw;
}
