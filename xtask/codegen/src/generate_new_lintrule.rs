use std::{path::PathBuf, str::FromStr};

use case::CaseExt;

pub fn generate_new_lintrule(path: &str, rule_name: &str) {
    let rule_folder = PathBuf::from_str(path).unwrap();
    match rule_folder.file_stem().and_then(|x| x.to_str()) {
        Some("nursery") => {}
        _ => {
            panic!("all new rules must be at a nursery folder");
        }
    }

    let rule_name_upper_camel = rule_name.to_camel();
    let rule_name_snake = rule_name.to_snake();
    let rule_name_lower_camel = rule_name_snake.to_camel_lowercase();

    // Generate rule code
    let code = format!(
        r#"use crate::semantic_services::Semantic;
use rome_analyze::{{
    context::RuleContext, declare_rule, Rule, RuleDiagnostic,
}};
use rome_console::markup;
use rome_js_semantic::{{Reference, ReferencesExtensions}};
use rome_js_syntax::JsIdentifierBinding;

declare_rule! {{
    /// Succinct description of the rule.
    ///
    /// Put context and details about the rule.
    /// As a starting point, you can take the description of the corresponding _ESLint_ rule (if any).
    ///
    /// Try to stay consistent with the descriptions of implemented rules.
    ///
    /// Add a link to the corresponding ESLint rule (if any):
    ///
    /// Source: https://eslint.org/docs/latest/rules/rule-name
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```js,expect_diagnostic
    /// var a = 1;
    /// a = 2;
    /// ```
    ///
    /// ## Valid
    ///
    /// ```js
    /// var a = 1;
    /// ```
    ///
    pub(crate) {rule_name_upper_camel} {{
        version: "next",
        name: "{rule_name_lower_camel}",
        recommended: false,
    }}
}}

impl Rule for {rule_name_upper_camel} {{
    type Query = Semantic<JsIdentifierBinding>;
    type State = Reference;
    type Signals = Vec<Self::State>;
    type Options = ();

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {{
        let binding = ctx.query();
        let model = ctx.model();

        binding.all_references(model).collect()
    }}

    fn diagnostic(_: &RuleContext<Self>, reference: &Self::State) -> Option<RuleDiagnostic> {{
        Some(
            RuleDiagnostic::new(
                rule_category!(),
                reference.syntax().text_trimmed_range(),
                markup! {{
                    "Variable is read here."
                }},
            )
            .note(markup! {{
                "This note will give you more information."
            }}),
        )
    }}
}}
"#
    );
    let file_name = format!("{path}/{rule_name_snake}.rs");
    std::fs::write(file_name, code).unwrap();

    let categories_path = "crates/rome_diagnostics_categories/src/categories.rs";
    let categories = std::fs::read_to_string(categories_path).unwrap();

    if !categories.contains(&rule_name_lower_camel) {
        let insertion_point = r#"    // Insert new nursery rule here"#;
        debug_assert!(categories.contains(insertion_point));

        let categories = categories.replace(insertion_point, &format!(
        r#""lint/nursery/{rule_name_lower_camel}": "https://docs.rome.tools/lint/rules/{rule_name_lower_camel}",
{insertion_point}"#));
        debug_assert!(categories.contains(&rule_name_lower_camel));

        std::fs::write(categories_path, categories).unwrap();
    }

    // Generate test code
    let tests_path = format!("crates/rome_js_analyze/tests/specs/nursery/{rule_name_lower_camel}");
    let _ = std::fs::create_dir_all(tests_path);

    let test_file =
        format!("crates/rome_js_analyze/tests/specs/nursery/{rule_name_lower_camel}/valid.js");
    if std::fs::File::open(&test_file).is_err() {
        let _ = std::fs::write(
            test_file,
            "/* should not generate diagnostics */\n\n var a = 1;",
        );
    }

    let test_file =
        format!("crates/rome_js_analyze/tests/specs/nursery/{rule_name_lower_camel}/invalid.js");
    if std::fs::File::open(&test_file).is_err() {
        let _ = std::fs::write(test_file, "\n\n var a = 1;\na = 2;\n a = 3;");
    }
}
