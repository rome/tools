import Scope from "../Scope";
import {AnyNode, tsMethodSignature} from "@internal/ast";
import {buildTSSignatureScope} from "../utils";
import {createScopeEvaluator} from "./index";

export default createScopeEvaluator({
	enter(node: AnyNode, parent: AnyNode, scope: Scope) {
		return buildTSSignatureScope(tsMethodSignature.assert(node), scope);
	},
});
