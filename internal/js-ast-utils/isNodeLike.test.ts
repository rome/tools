import {test} from "rome";
import {isNodeLike} from "./isNodeLike";
import {NodeBase} from "@internal/parser-core";

test(
	"returns true for nodeLike ",
	(t) => {
		const nodeLike: NodeBase = {
			type: "a",
		};
		t.true(isNodeLike(nodeLike));
	},
);

test(
	"return false for non nodeLike",
	(t) => {
		t.false(isNodeLike(null));
		t.false(isNodeLike({type: 1}));
		t.false(isNodeLike([]));
	},
);
