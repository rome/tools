import {Scope} from "@romejs/compiler";
import {AnyNode} from "@romejs/ast";
import {EvalResult} from "@romejs/js-ast-utils/tryStaticEvaluation";
import {getCreateElementProp} from ".";
import {tryStaticEvaluation} from "@romejs/js-ast-utils";

export default function analyzeCreateElementProp(
	node: AnyNode,
	scope: Scope,
	propName: string,
): EvalResult["value"] {
	const elementType = getCreateElementProp(node, scope, propName);
	if (!elementType) {
		return;
	}
	return tryStaticEvaluation(elementType.value, scope).value;
}
