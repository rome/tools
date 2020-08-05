import {Scope} from "@internal/compiler";
import {AnyNode} from "@internal/ast";
import {EvalResult} from "@internal/js-ast-utils/tryStaticEvaluation";
import getCreateElementProp from "./getCreateElementProp";
import {tryStaticEvaluation} from "@internal/js-ast-utils";

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
