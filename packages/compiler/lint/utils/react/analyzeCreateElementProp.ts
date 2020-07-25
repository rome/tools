import {Scope} from "@romefrontend/compiler";
import {AnyNode} from "@romefrontend/ast";
import {EvalResult} from "@romefrontend/js-ast-utils/tryStaticEvaluation";
import {getCreateElementProp} from ".";
import {tryStaticEvaluation} from "@romefrontend/js-ast-utils";

export default function analyzeCreateElementProp(
	node: AnyNode,
	scope: Scope,
	propName: string,
): EvalResult["value"] {
	const prop = getCreateElementProp(node, scope, propName);
	if (!prop) {
		return;
	}
	return tryStaticEvaluation(prop.value, scope).value;
}
