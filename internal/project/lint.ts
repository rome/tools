import {PathPattern} from "@internal/path-match";
import {
	ProjectLintRules,
	a11YRules,
	cssRules,
	htmlRules,
	jsRules,
	jsxRules,
	reactRules,
	regexRules,
	tsRules,
} from "@internal/compiler/lint/rules/categories";

type Recommended = {
	recommended: true;
};
// applies the possibility to recommend a category, resulting in these two options:
//
// 1. js: { recommended: true }
// 2. js: { noAccessKey: true }
// 3. js: false
type DeepRecommend =
	| {
			[Category in keyof ProjectLintRules]:
				| (ProjectLintRules[Category] & {
						recommended?: undefined;
					})
				| boolean
				| Recommended
		}
	| Recommended;

export type Rules =
	| Recommended
	| ({
			recommended?: undefined;
		} & DeepRecommend);

// NOTE: these are loose types that will be provided by the user
// These soft rules are use inside the configuration provided by the user
type SoftRules = {
	a11y?: {[key in a11YRules]?: boolean};
	css?: {[key in cssRules]?: boolean};
	html?: {[key in htmlRules]?: boolean};
	js?: {[key in jsRules]?: boolean};
	jsx?: {[key in jsxRules]?: boolean};
	react?: {[key in reactRules]?: boolean};
	regex?: {[key in regexRules]?: boolean};
	ts?: {[key in tsRules]?: boolean};
};

type LooseRules =
	| {
			[Category in keyof SoftRules]:
				| (SoftRules[Category] & {
						recommended?: undefined;
					})
				| boolean
				| Recommended
		}
	| Recommended;

export type UserProjectRules =
	| Recommended
	| ({
			recommended?: undefined;
		} & LooseRules);

export type LintConfig = {
	globals: string[];
	ignore: PathPattern[];
	requireSuppressionExplanations: boolean;
	rules?: Rules;
};
