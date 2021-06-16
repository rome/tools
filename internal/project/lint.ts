import {PathPattern} from "@internal/path-match";
import {
	A11YRules,
	CssRules,
	HtmlRules,
	JsRules,
	JsxRules,
	ProjectLintRules,
	ReactRules,
	RegexRules,
	TsRules,
} from "@internal/compiler/lint/rules/categories";

type Recommended = {
	recommended: true;
};
// applies the possibility to recommend a category, resulting in these two options:
//
// 1. js: { recommended: true }
// 2. js: { noAccessKey: true }
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
	a11y?: {[key in A11YRules]?: boolean};
	css?: {[key in CssRules]?: boolean};
	html?: {[key in HtmlRules]?: boolean};
	js?: {[key in JsRules]?: boolean};
	jsx?: {[key in JsxRules]?: boolean};
	react?: {[key in ReactRules]?: boolean};
	regex?: {[key in RegexRules]?: boolean};
	ts?: {[key in TsRules]?: boolean};
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
