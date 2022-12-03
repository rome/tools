use std::{path::PathBuf, str::FromStr};

use case::CaseExt;

pub fn generate_new_lintrule(path: &str, rule_name: &str) {
    let rule_folder = PathBuf::from_str(&path).unwrap();
    match rule_folder.file_stem()
        .and_then(|x| x.to_str()) {
        Some("nursery") => {},
        _ => {
            panic!("all new rules must be at a nursery folder");
        },
    }

    
    let rule_name_upper_camel = rule_name.to_camel();
    let rule_name_snake = rule_name.to_snake();
    let rule_name_lower_camel = rule_name_snake.to_camel_lowercase();
    let code = format!(
        r#"use rome_analyze::{{
context::RuleContext, declare_rule, Rule, RuleDiagnostic,
}};
use crate::semantic_services::Semantic;
use rome_js_syntax::JsCallExpression;

declare_rule! {{
/// Put your description here
///
/// ## Examples
///
/// ### Invalid
///
/// ```js,expect_diagnostic
/// ```
///
/// ## Valid
///
/// ```js
/// ```
///
pub(crate) {rule_name_upper_camel} {{
version: "12.0.0",
name: "{rule_name_lower_camel}",
recommended: false,
}}
}}

impl Rule for {rule_name_upper_camel} {{
type Query = Semantic<JsCallExpression>;
type State = ();
type Signals = Vec<Self::State>;
type Options = ();

fn run(_: &RuleContext<Self>) -> Vec<Self::State> {{
let mut signals = vec![];
signals
}}

fn diagnostic(_: &RuleContext<Self>, _: &Self::State) -> Option<RuleDiagnostic> {{
None
}}
}}
"#
    );
    let file_name = format!("{path}/{rule_name_snake}.rs");
    std::fs::write(file_name, code).unwrap();


    let categories_path = "crates/rome_diagnostics_categories/src/categories.rs";
    let categories = std::fs::read_to_string(categories_path).unwrap();
    let insertion_point = r#"
    
    ;

    // General categories"#;
    debug_assert!(categories.contains(insertion_point));
    let categories = categories.replace(insertion_point, &format!(r#"
    "lint/nursery/{rule_name_lower_camel}": "https://docs.rome.tools/lint/rules/{rule_name_lower_camel}",
    
    ;

    // General categories"#));
    debug_assert!(categories.contains(&rule_name_lower_camel));
    std::fs::write(categories_path, categories).unwrap();

}