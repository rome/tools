import {AnyNode, TSTupleElement, tsTupleElement} from "@internal/ast";

export default function TSTupleElement(node: AnyNode) {
	node = tsTupleElement.assert(node);
	throw new Error("unimplemented");
}
