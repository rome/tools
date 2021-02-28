import Scope from "../Scope";
import {AnyNode, tsConstructorType} from "@internal/ast";
import {buildTSSignatureScope} from "../utils";
import {createScopeEvaluator} from "./index";

export default createScopeEvaluator({
	enter(node: AnyNode, parent: AnyNode, scope: Scope) {
		return buildTSSignatureScope(tsConstructorType.assert(node), scope);
	},
});
