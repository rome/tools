import {CompilerPath} from "@internal/compiler";
import {
	EvalOptions,
	EvalResult,
	tryStaticEvaluation,
} from "./tryStaticEvaluation";

export function tryStaticEvaluationPath(
	path: CompilerPath,
	opts?: EvalOptions,
): EvalResult {
	return tryStaticEvaluation(path.node, path.scope, opts);
}
