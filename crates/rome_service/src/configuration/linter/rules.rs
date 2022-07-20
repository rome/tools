//! Generated file, do not edit by hand, see `xtask/codegen`

use crate::RuleConfiguration;
use serde::{Deserialize, Serialize};
#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct Rules {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub js: Option<JsRules>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub jsx: Option<JsxRules>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub regex: Option<RegexRules>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ts: Option<TsRules>,
}
impl Default for Rules {
    fn default() -> Self {
        Self {
            js: Some(JsRules::default()),
            jsx: Some(JsxRules::default()),
            regex: Some(RegexRules::default()),
            ts: Some(TsRules::default()),
        }
    }
}
#[derive(Deserialize, Default, Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase", default, deny_unknown_fields)]
pub struct JsRules {
    #[serde(skip_serializing_if = "RuleConfiguration::is_err")]
    pub no_arguments: RuleConfiguration,
    #[serde(skip_serializing_if = "RuleConfiguration::is_err")]
    pub no_async_promise_executor: RuleConfiguration,
    #[serde(skip_serializing_if = "RuleConfiguration::is_err")]
    pub no_catch_assign: RuleConfiguration,
    #[serde(skip_serializing_if = "RuleConfiguration::is_err")]
    pub no_compare_neg_zero: RuleConfiguration,
    #[serde(skip_serializing_if = "RuleConfiguration::is_err")]
    pub no_dead_code: RuleConfiguration,
    #[serde(skip_serializing_if = "RuleConfiguration::is_err")]
    pub no_debugger: RuleConfiguration,
    #[serde(skip_serializing_if = "RuleConfiguration::is_err")]
    pub no_delete: RuleConfiguration,
    #[serde(skip_serializing_if = "RuleConfiguration::is_err")]
    pub no_double_equals: RuleConfiguration,
    #[serde(skip_serializing_if = "RuleConfiguration::is_err")]
    pub no_empty_pattern: RuleConfiguration,
    #[serde(skip_serializing_if = "RuleConfiguration::is_err")]
    pub no_label_var: RuleConfiguration,
    #[serde(skip_serializing_if = "RuleConfiguration::is_err")]
    pub no_negation_else: RuleConfiguration,
    #[serde(skip_serializing_if = "RuleConfiguration::is_err")]
    pub no_shouty_constants: RuleConfiguration,
    #[serde(skip_serializing_if = "RuleConfiguration::is_err")]
    pub no_sparse_array: RuleConfiguration,
    #[serde(skip_serializing_if = "RuleConfiguration::is_err")]
    pub no_unnecessary_continue: RuleConfiguration,
    #[serde(skip_serializing_if = "RuleConfiguration::is_err")]
    pub no_unsafe_negation: RuleConfiguration,
    #[serde(skip_serializing_if = "RuleConfiguration::is_err")]
    pub no_unused_template_literal: RuleConfiguration,
    #[serde(skip_serializing_if = "RuleConfiguration::is_err")]
    pub use_block_statements: RuleConfiguration,
    #[serde(skip_serializing_if = "RuleConfiguration::is_err")]
    pub use_simplified_logic_expression: RuleConfiguration,
    #[serde(skip_serializing_if = "RuleConfiguration::is_err")]
    pub use_single_case_statement: RuleConfiguration,
    #[serde(skip_serializing_if = "RuleConfiguration::is_err")]
    pub use_single_var_declarator: RuleConfiguration,
    #[serde(skip_serializing_if = "RuleConfiguration::is_err")]
    pub use_template: RuleConfiguration,
    #[serde(skip_serializing_if = "RuleConfiguration::is_err")]
    pub use_valid_typeof: RuleConfiguration,
    #[serde(skip_serializing_if = "RuleConfiguration::is_err")]
    pub use_while: RuleConfiguration,
}
#[derive(Deserialize, Default, Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase", default, deny_unknown_fields)]
pub struct JsxRules {
    #[serde(skip_serializing_if = "RuleConfiguration::is_err")]
    pub no_comment_text: RuleConfiguration,
    #[serde(skip_serializing_if = "RuleConfiguration::is_err")]
    pub no_implicit_boolean: RuleConfiguration,
    #[serde(skip_serializing_if = "RuleConfiguration::is_err")]
    pub use_self_closing_elements: RuleConfiguration,
}
#[derive(Deserialize, Default, Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase", default, deny_unknown_fields)]
pub struct RegexRules {
    #[serde(skip_serializing_if = "RuleConfiguration::is_err")]
    pub no_multiple_spaces_in_regular_expression_literals: RuleConfiguration,
}
#[derive(Deserialize, Default, Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase", default, deny_unknown_fields)]
pub struct TsRules {
    #[serde(skip_serializing_if = "RuleConfiguration::is_err")]
    pub use_shorthand_array_type: RuleConfiguration,
}
