import {PathPattern} from "@internal/path-match";
import {ProjectLintRules} from "@internal/compiler/lint/rules/categories";

// applies the possibility to recommend a category, resulting in these two options:
//
// 1. js: { recommended: true }
// 2. js: { noAccessKey: true }
type DeepRecommend<T> =
	| {
			[P in keyof T]: T[P] & {
				recommended?: undefined;
			}
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
