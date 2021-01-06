import {Scope} from "../../scopes";
import {AnyNode, TSTemplateElement, tsTemplateElement} from "@internal/ast";

export default function TSTemplateElement(node: AnyNode, scope: Scope) {
	node = tsTemplateElement.assert(node);
	scope;
	throw new Error("unimplemented");
}
