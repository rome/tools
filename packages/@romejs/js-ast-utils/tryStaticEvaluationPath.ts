import {Path} from "@romejs/compiler";
import tryStaticEvaluation, {
	EvalOptions,
	EvalResult,
} from "./tryStaticEvaluation";

export default function tryStaticEvaluationPath(
	path: Path,
	opts?: EvalOptions,
): EvalResult {
	return tryStaticEvaluation(path.node, path.scope, opts);
}
