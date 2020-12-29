import {test} from "rome";
import {assertSingleNode} from "./assertSingleNode";
import {AnyNode, AnyNodes} from "@internal/ast";
import {template} from "@internal/js-ast-utils";

const node: AnyNode = template.statement`let a`;
const nodeArray: AnyNodes = [node, node];

test(
	"returns back the node when asserted with a single node or a list with single node",
	(t) => {
		t.looksLike(assertSingleNode(node), node);
		t.looksLike(assertSingleNode([node]), node);
	},
);

test(
	"fails with error when asserted with list of more than 1 nodes or undefined ",
	(t) => {
		t.throws(
			() => {
				assertSingleNode(nodeArray);
			},
			"Expected node list length of 1 but got 2",
		);
		t.throws(
			() => {
				assertSingleNode((undefined as unknown) as AnyNode);
			},
			"Expected node or node list but got undefined",
		);
	},
);
