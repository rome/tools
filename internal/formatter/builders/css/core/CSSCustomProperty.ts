import {CSSCustomProperty} from "@internal/ast";
import {Builder, Token} from "@internal/formatter";

export default function CSSCustomProperty(
	builder: Builder,
	node: CSSCustomProperty,
): Token {
	// custom properties are case insensitive so they are formatted to lower case
	// Source:
	// - https://www.w3.org/TR/css-variables/#apis
	// - https://www.w3.org/TR/css-variables/#serializing-custom-props
	return node.value.toLowerCase();
}
