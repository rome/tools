import {AnyNode, TSTupleElement, tsTupleElement} from "@romefrontend/ast";

export default function TSTupleElement(node: AnyNode) {
	node = tsTupleElement.assert(node);
	throw new Error("unimplemented");
}
