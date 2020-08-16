import { test } from "rome";
import { assertSingleNode } from "./assertSingleNode";
import { AnyNode, AnyNodes } from "@internal/ast";
import { parseJS } from "@internal/js-parser";

const node: AnyNode = parseJS({
    input: "let a;",
    path: "unknown"
}).body[0];
const nodeArray: AnyNodes = [node, node];

test(
    "returns back the node when asserted with single node",
    (t) => {
        t.looksLike(assertSingleNode(node), node);
    },
);

test(
    "fails with error when asserted with array of more than 1 nodes ",
    (t)=>{
    
    }
)
