import {test} from "rome";
import {assertSingleOrMultipleNodes} from "./assertSingleOrMultipleNodes";
import {AnyNode} from "@internal/ast";
import {template} from "@internal/js-ast-utils";

test(
	"returns back the asserted value when asserted with a single or multiple node(s)",
	(t) => {
		const node: AnyNode = template.statement`let a`;
		t.looksLike(assertSingleOrMultipleNodes(node), node);
		t.looksLike(assertSingleOrMultipleNodes([node]), [node]);
		t.looksLike(assertSingleOrMultipleNodes([node, node]), [node, node]);
	},
);

test(
	"fails with error when asserted with undefined or symbol ",
	(t) => {
		t.throws(
			() => {
				assertSingleOrMultipleNodes((Symbol("symbol") as unknown) as AnyNode);
			},
			"No symbols expected here",
		);
		t.throws(
			() => {
				assertSingleOrMultipleNodes((undefined as unknown) as AnyNode);
			},
			"Expected node or node list but got null",
		);
	},
);
