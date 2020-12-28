import {HTMLCdataTag} from "@internal/ast";
import {Builder, Token, concat} from "@internal/formatter";

export default function HTMLCdataTag(
	builder: Builder,
	node: HTMLCdataTag,
): Token {
	return concat(["<![CDATA[", node.value || "", "]]>"]);
}
