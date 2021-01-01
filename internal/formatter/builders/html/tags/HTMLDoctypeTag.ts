import {HTMLDoctypeTag} from "@internal/ast";
import {Builder, Token, concat, space} from "@internal/formatter";

export default function HTMLDoctypeTag(
	builder: Builder,
	node: HTMLDoctypeTag,
): Token {
	return concat(["<!DOCTYPE", space, node.value, ">"]);
}
