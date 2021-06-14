import {
	A11YRulesCategoryRules,
	CssRulesCategoryRules,
	HtmlRulesCategoryRules,
	JsRulesCategoryRules,
	JsxRulesCategoryRules,
	LintCategories,
	ReactRulesCategoryRules,
	RegexRulesCategoryRules,
	RuleNames,
	TsRulesCategoryRules,
	lintCategories,
	ruleNames,
} from "@internal/compiler/lint/rules/categories";
import {Consumer} from "@internal/consume";
import {Rules} from "@internal/project/lint";
import {descriptions} from "@internal/diagnostics";

type AllCategoryRules =
	| A11YRulesCategoryRules
	| CssRulesCategoryRules
	| HtmlRulesCategoryRules
	| JsRulesCategoryRules
	| JsxRulesCategoryRules
	| ReactRulesCategoryRules
	| RegexRulesCategoryRules
	| TsRulesCategoryRules;

function loadSingleRule(rule: Consumer): boolean {
	return rule.asBoolean();
}

function loadCategory(category: Consumer): AllCategoryRules {
	let mappedCategory: AllCategoryRules;
	mappedCategory = category.asMappedObject((rule, name) => {
		if (!ruleNames.has(name as RuleNames)) {
			throw rule.unexpected(
				descriptions.PROJECT_CONFIG.RULES_UNKNOWN_RULE_NAME(name, Array.from(ruleNames)),
				{
					target: "key",
					at: "none",
				},
			);
		}

		return loadSingleRule(rule);
	});
	return mappedCategory;
}

export function loadRules(lint: Consumer): Rules {
	return lint.get("rules").asMappedObject<boolean | AllCategoryRules>((
		value,
		key,
	) => {
		if (key === "recommended") {
			return value.asBoolean();
		}
		if (lintCategories.has(key as LintCategories)) {
			if (typeof value.asUnknown() === "boolean") {
				return value.asBoolean();
			}

			return loadCategory(value);
		} else {
			throw value.unexpected(
				descriptions.PROJECT_CONFIG.RULES_UNKNOWN_CATEGORY(
					key,
					Array.from(lintCategories),
				),
				{
					target: "key",
					at: "none",
				},
			);
		}
	});
}
