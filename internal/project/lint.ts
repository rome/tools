import {PathPattern} from "@internal/path-match";
import {Recommendable} from "@internal/project/types";
import {LintRuleName, LintRules} from "@internal/compiler/lint/rules/categories";

type Category = Recommendable & LintRules;

export type Rules = Category | boolean;

export interface LintConfig {
	globals: string[];
	ignore: PathPattern[];
	requireSuppressionExplanations: boolean;
	disabledRules: LintRuleName[];
	rules?: Rules;
}
