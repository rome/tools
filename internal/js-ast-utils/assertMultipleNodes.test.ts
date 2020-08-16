import {test} from "rome";
import {assertMultipleNodes} from "./assertMultipleNodes";
import {AnyNode, AnyNodes } from "@internal/ast";
import { parseJS } from "@internal/js-parser";

const node: AnyNode = parseJS({
    input: "2+3",
    path: "unknown"
}).body[0];
const nodeArray: AnyNodes = [node, node];

test(
	"returns back an array of nodes when asserted with node array",
	(t) => {
		let returnedArray = assertMultipleNodes(nodeArray);
		t.is(returnedArray.length, nodeArray.length);
		t.looksLike(returnedArray, nodeArray);
	},
);

test(
	"returns an empty array if asserted with undefined",
	(t) => {
		t.looksLike(assertMultipleNodes(((undefined as unknown) as AnyNode)), []);
	},
);

test(
	"returns an array with node when asserted with a single node",
	(t) => {
		t.looksLike(assertMultipleNodes(node), [node]);
	},
);
