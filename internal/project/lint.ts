import {PathPattern} from "@internal/path-match";
import {ProjectLintRules} from "@internal/compiler/lint/rules/categories";

// applies the possibility to recommend a category, resulting in these two options:
//
// 1. js: { recommended: true }
// 2. js: { noAccessKey: true }
type DeepRecommend<LintRule> =
	| {
			[Category in keyof LintRule]:
				| (LintRule[Category] & {
						recommended?: undefined;
					})
				| boolean
		}
	| {
			recommended: true;
		};

export type Rules =
	| {
			recommended: true;
		}
	| ({
			recommended?: undefined;
		} & DeepRecommend<ProjectLintRules>);

export type LintConfig = {
	globals: string[];
	ignore: PathPattern[];
	requireSuppressionExplanations: boolean;
	rules?: Rules;
};
