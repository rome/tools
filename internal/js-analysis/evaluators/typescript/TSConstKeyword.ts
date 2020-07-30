import {Scope} from "../../scopes";
import {AnyNode, TSConstKeyword, tsConstKeyword} from "@internal/ast";

export default function TSConstKeyword(node: AnyNode, scope: Scope) {
	node = tsConstKeyword.assert(node);
	scope;
	throw new Error("unimplemented");
}
