import {
	LintCategories,
	RuleNames,
	lintCategories,
	ruleNames,
} from "@internal/compiler/lint/rules/categories";
import {Consumer} from "@internal/consume";
import {Rules} from "@internal/project/lint";
import {descriptions} from "@internal/diagnostics";

function loadSingleRule(rule: Consumer): boolean {
	return rule.asBoolean();
}

function loadCategory(category: Consumer): Map<RuleNames, boolean> {
	const mappedRules: Map<RuleNames, boolean> = new Map();
	for (const [ruleName, rule] of category.asMap()) {
		if (!ruleNames.has(ruleName as RuleNames)) {
			mappedRules.set(ruleName as RuleNames, loadSingleRule(rule));
		}
	}
	return mappedRules;
}

export function loadRules(lint: Consumer): Rules {
	return lint.get("rules").asMappedObject<boolean | Map<RuleNames, boolean>>((
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
