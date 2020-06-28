import {AnyNode, TSTupleElement, tsTupleElement} from "@romejs/ast";

export default function TSTupleElement(node: AnyNode) {
	node = tsTupleElement.assert(node);
	throw new Error("unimplemented");
}
