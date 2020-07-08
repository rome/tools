import {Path} from "@romefrontend/compiler";
import {
	EvalOptions,
	EvalResult,
	tryStaticEvaluation,
} from "./tryStaticEvaluation";

export function tryStaticEvaluationPath(
	path: Path,
	opts?: EvalOptions,
): EvalResult {
	return tryStaticEvaluation(path.node, path.scope, opts);
}
