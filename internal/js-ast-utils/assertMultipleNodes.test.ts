import {test} from "rome";
import {assertMultipleNodes} from "./assertMultipleNodes";
import {AnyNode} from "@internal/ast";
import {template} from "@internal/js-ast-utils";

const node: AnyNode = template.statement`let a;`;
const nodeArray: AnyNode[] = [node, node];

test(
	"returns back the same list of nodes",
	(t) => {
		let returnedArray = assertMultipleNodes(nodeArray);
		t.is(returnedArray.length, nodeArray.length);
		t.looksLike(returnedArray, nodeArray);
	},
);

test(
	"returns an empty list if asserted with undefined",
	(t) => {
		t.looksLike(assertMultipleNodes((undefined as unknown) as AnyNode), []);
	},
);

test(
	"returns a list with node when asserted with a single node",
	(t) => {
		t.looksLike(assertMultipleNodes(node), [node]);
	},
);
