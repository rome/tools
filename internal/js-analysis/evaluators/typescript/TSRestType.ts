import {Scope} from "../../scopes";
import {AnyNode, TSRestType, tsRestType} from "@internal/ast";

export default function TSRestType(node: AnyNode, scope: Scope) {
	node = tsRestType.assert(node);
	scope;
	throw new Error("unimplemented");
}
