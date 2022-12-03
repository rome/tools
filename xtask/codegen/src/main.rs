#[cfg(feature = "aria")]
mod generate_aria;
#[cfg(feature = "schema")]
mod generate_bindings;
#[cfg(feature = "configuration")]
mod generate_configuration;
#[cfg(feature = "schema")]
mod generate_schema;

use case::CaseExt;
use pico_args::Arguments;
use xtask::{project_root, pushd, Mode, Result};

#[cfg(feature = "aria")]
use crate::generate_aria::generate_aria;
#[cfg(feature = "schema")]
use crate::generate_bindings::generate_workspace_bindings;
#[cfg(feature = "configuration")]
use crate::generate_configuration::generate_rules_configuration;
#[cfg(feature = "schema")]
use crate::generate_schema::generate_configuration_schema;
use xtask_codegen::{
    generate_analyzer, generate_ast, generate_formatters, generate_parser_tests, generate_tables,
};

fn main() -> Result<()> {
    let _d = pushd(project_root());

    let mut args = Arguments::from_env();
    let command = args.subcommand()?.unwrap_or_default();
    match command.as_str() {
        "grammar" => {
            generate_grammar(args);
            Ok(())
        }
        "formatter" => {
            generate_formatters();
            Ok(())
        }
        "test" => {
            generate_parser_tests(Mode::Overwrite)?;
            Ok(())
        }
        "unicode" => {
            generate_tables()?;
            Ok(())
        }
        "analyzer" => {
            generate_analyzer()?;
            Ok(())
        }
        "newlintrule" => {
            let path: String = args.value_from_str("--path").unwrap();
            let rule_name: String = args.value_from_str("--name").unwrap();
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
    /// <DESCRIPTION>
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
            Ok(())
        }
        #[cfg(feature = "configuration")]
        "configuration" => {
            generate_rules_configuration(Mode::Overwrite)?;
            Ok(())
        }
        #[cfg(feature = "schema")]
        "schema" => {
            generate_configuration_schema(Mode::Overwrite)?;
            Ok(())
        }
        #[cfg(feature = "schema")]
        "bindings" => {
            generate_workspace_bindings(Mode::Overwrite)?;
            Ok(())
        }
        #[cfg(feature = "aria")]
        "aria" => {
            generate_aria(Mode::Overwrite)?;
            Ok(())
        }
        "all" => {
            generate_tables()?;
            generate_grammar(args);
            generate_parser_tests(Mode::Overwrite)?;
            generate_formatters();
            generate_analyzer()?;
            #[cfg(feature = "configuration")]
            generate_rules_configuration(Mode::Overwrite)?;
            #[cfg(feature = "schema")]
            generate_configuration_schema(Mode::Overwrite)?;
            #[cfg(feature = "schema")]
            generate_workspace_bindings(Mode::Overwrite)?;
            #[cfg(feature = "aria")]
            generate_aria(Mode::Overwrite)?;
            Ok(())
        }
        _ => {
            eprintln!(
                "\
cargo codegen
Run codegen command.
USAGE:
	cargo codegen <SUBCOMMAND> [option]
SUBCOMMANDS:
	aria            Generate aria bindings for lint rules
	analyzer        Generate factory functions for the analyzer and the configuration of the analyzers
	configuration    Generate the part of the configuration that depends on some metadata
	schema          Generate the JSON schema for the Rome configuration file format
	bindings        Generate TypeScript definitions for the JavaScript bindings to the Workspace API
	grammar         Transforms ungram files into AST
	formatter       Generates formatters for each language
	test            Extracts parser inline comments into test files
	unicode         Generates unicode table inside lexer
    newlintrule     Generates a template for an empty lint rule
    all             Run all generators
			"
            );
            Ok(())
        }
    }
}

fn generate_grammar(args: Arguments) {
    let arg_list = args.finish();
    let language_list = arg_list
        .into_iter()
        .filter_map(|arg| arg.to_str().map(|item| item.to_string()))
        .collect::<Vec<_>>();
    let _ = generate_ast(Mode::Overwrite, language_list);
}
