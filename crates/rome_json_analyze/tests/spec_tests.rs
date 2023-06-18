use rome_analyze::{
    AnalysisFilter, AnalyzerAction, AnalyzerOptions, ControlFlow, Never, RuleFilter,
};
use rome_console::{
    fmt::{Formatter, Termcolor},
    markup, Markup,
};
use rome_deserialize::json::deserialize_from_json_str;
use rome_diagnostics::advice::CodeSuggestionAdvice;
use rome_diagnostics::termcolor::{Buffer, NoColor};
use rome_diagnostics::{DiagnosticExt, Error, PrintDiagnostic, Severity};
use rome_json_parser::{parse_json, JsonParse};
use rome_json_syntax::{JsonLanguage, JsonSyntaxNode};
use rome_rowan::SyntaxKind;
use rome_rowan::SyntaxSlot;
use rome_service::configuration::to_analyzer_configuration;
use rome_service::settings::WorkspaceSettings;
use rome_service::Configuration;
use similar::TextDiff;
use std::{
    ffi::OsStr, fmt::Write, fs::read_to_string, os::raw::c_int, path::Path, slice, sync::Once,
};

tests_macros::gen_tests! {"tests/specs/**/*.{json}", crate::run_test, "module"}

fn run_test(input: &'static str, _: &str, _: &str, _: &str) {
    register_leak_checker();

    let input_file = Path::new(input);
    let file_name = input_file.file_name().and_then(OsStr::to_str).unwrap();

    let (group, rule) = parse_test_path(input_file);
    if rule == "specs" || rule == "suppression" {
        panic!("the test file must be placed in the {rule}/<group-name>/<rule-name>/ directory");
    }
    if group == "specs" || group == "suppression" {
        panic!("the test file must be placed in the {group}/{rule}/<rule-name>/ directory");
    }

    if rome_json_analyze::metadata()
        .find_rule(group, rule)
        .is_none()
    {
        panic!("could not find rule {group}/{rule}");
    }

    let rule_filter = RuleFilter::Rule(group, rule);
    let filter = AnalysisFilter {
        enabled_rules: Some(slice::from_ref(&rule_filter)),
        ..AnalysisFilter::default()
    };

    let mut snapshot = String::new();

    let input_code = read_to_string(input_file)
        .unwrap_or_else(|err| panic!("failed to read {:?}: {:?}", input_file, err));

    let quantity_diagnostics =
        write_analysis_to_snapshot(&mut snapshot, &input_code, filter, file_name, input_file);

    insta::with_settings!({
        prepend_module_to_snapshot => false,
        snapshot_path => input_file.parent().unwrap(),
    }, {
        insta::assert_snapshot!(file_name, snapshot, file_name);
    });

    if input_code.contains("/* should not generate diagnostics */") && quantity_diagnostics > 0 {
        panic!("This test should not generate diagnostics");
    }
}

pub(crate) fn write_analysis_to_snapshot(
    snapshot: &mut String,
    input_code: &str,
    filter: AnalysisFilter,
    file_name: &str,
    input_file: &Path,
) -> usize {
    let parsed = parse_json(input_code);
    let root = parsed.tree();

    let mut diagnostics = Vec::new();
    let mut code_fixes = Vec::new();
    let mut options = AnalyzerOptions::default();
    // We allow a test file to configure its rule using a special
    // file with the same name as the test but with extension ".options.json"
    // that configures that specific rule.
    let options_file = input_file.with_extension("options.json");
    if let Ok(json) = std::fs::read_to_string(options_file.clone()) {
        let deserialized = deserialize_from_json_str::<Configuration>(json.as_str());
        if deserialized.has_errors() {
            diagnostics.extend(
                deserialized
                    .into_diagnostics()
                    .into_iter()
                    .map(|diagnostic| {
                        diagnostic_to_string(
                            options_file.file_stem().unwrap().to_str().unwrap(),
                            &json,
                            diagnostic,
                        )
                    })
                    .collect::<Vec<_>>(),
            );
            None
        } else {
            let configuration = deserialized.into_deserialized();
            let mut settings = WorkspaceSettings::default();
            settings.merge_with_configuration(configuration).unwrap();
            let configuration =
                to_analyzer_configuration(&settings.linter, &settings.languages, |_| vec![]);
            options = AnalyzerOptions {
                configuration,
                ..AnalyzerOptions::default()
            };

            Some(json)
        }
    } else {
        None
    };

    let (_, errors) =
        rome_json_analyze::analyze(&root.value().unwrap(), filter, &options, |event| {
            if let Some(mut diag) = event.diagnostic() {
                for action in event.actions() {
                    if !action.is_suppression() {
                        check_code_action(input_file, input_code, &action);
                        diag = diag.add_code_suggestion(CodeSuggestionAdvice::from(action));
                    }
                }

                let error = diag.with_severity(Severity::Warning);
                diagnostics.push(diagnostic_to_string(file_name, input_code, error));
                return ControlFlow::Continue(());
            }

            for action in event.actions() {
                if !action.is_suppression() {
                    check_code_action(input_file, input_code, &action);
                    code_fixes.push(code_fix_to_string(input_code, action));
                }
            }

            ControlFlow::<Never>::Continue(())
        });

    for error in errors {
        diagnostics.push(diagnostic_to_string(file_name, input_code, error));
    }

    writeln!(snapshot, "# Input").unwrap();
    writeln!(snapshot, "```js").unwrap();
    writeln!(snapshot, "{}", input_code).unwrap();
    writeln!(snapshot, "```").unwrap();
    writeln!(snapshot).unwrap();

    if !diagnostics.is_empty() {
        writeln!(snapshot, "# Diagnostics").unwrap();
        for diagnostic in &diagnostics {
            writeln!(snapshot, "```").unwrap();
            writeln!(snapshot, "{}", diagnostic).unwrap();
            writeln!(snapshot, "```").unwrap();
            writeln!(snapshot).unwrap();
        }
    }

    if !code_fixes.is_empty() {
        writeln!(snapshot, "# Actions").unwrap();
        for action in code_fixes {
            writeln!(snapshot, "```diff").unwrap();
            writeln!(snapshot, "{}", action).unwrap();
            writeln!(snapshot, "```").unwrap();
            writeln!(snapshot).unwrap();
        }
    }

    diagnostics.len()
}

/// The test runner for the analyzer is currently designed to have a
/// one-to-one mapping between test case and analyzer rules.
/// So each testing file will be run through the analyzer with only the rule
/// corresponding to the directory name. E.g., `style/useWhile/test.js`
/// will be analyzed with just the `style/useWhile` rule.
fn parse_test_path(file: &Path) -> (&str, &str) {
    let rule_folder = file.parent().unwrap();
    let rule_name = rule_folder.file_name().unwrap();

    let group_folder = rule_folder.parent().unwrap();
    let group_name = group_folder.file_name().unwrap();

    (group_name.to_str().unwrap(), rule_name.to_str().unwrap())
}

fn markup_to_string(markup: Markup) -> String {
    let mut buffer = Vec::new();
    let mut write = Termcolor(NoColor::new(&mut buffer));
    let mut fmt = Formatter::new(&mut write);
    fmt.write_markup(markup).unwrap();

    String::from_utf8(buffer).unwrap()
}
#[allow(clippy::let_and_return)]
fn diagnostic_to_string(name: &str, source: &str, diag: Error) -> String {
    let error = diag.with_file_path(name).with_file_source_code(source);
    let text = markup_to_string(markup! {
        {PrintDiagnostic::verbose(&error)}
    });

    text
}

fn check_code_action(path: &Path, source: &str, action: &AnalyzerAction<JsonLanguage>) {
    let (_, text_edit) = action.mutation.as_text_edits().unwrap_or_default();

    let output = text_edit.new_string(source);

    let new_tree = action.mutation.clone().commit();

    // Checks that applying the text edits returned by the BatchMutation
    // returns the same code as printing the modified syntax tree
    assert_eq!(new_tree.to_string(), output);

    if has_bogus_nodes_or_empty_slots(&new_tree) {
        panic!(
            "modified tree has bogus nodes or empty slots:\n{new_tree:#?} \n\n {}",
            new_tree
        )
    }

    // Checks the returned tree contains no missing children node
    if format!("{new_tree:?}").contains("missing (required)") {
        panic!("modified tree has missing children:\n{new_tree:#?}")
    }

    // Re-parse the modified code and panic if the resulting tree has syntax errors
    let re_parse = parse_json(&output);
    assert_errors_are_absent(&re_parse, path);
}

fn code_fix_to_string(source: &str, action: AnalyzerAction<JsonLanguage>) -> String {
    let (_, text_edit) = action.mutation.as_text_edits().unwrap_or_default();

    let output = text_edit.new_string(source);

    let diff = TextDiff::from_lines(source, &output);

    let mut diff = diff.unified_diff();
    diff.context_radius(3);

    diff.to_string()
}

// Check that all red / green nodes have correctly been released on exit
extern "C" fn check_leaks() {
    if let Some(report) = rome_rowan::check_live() {
        panic!("\n{report}")
    }
}
pub(crate) fn register_leak_checker() {
    // Import the atexit function from libc
    extern "C" {
        fn atexit(f: extern "C" fn()) -> c_int;
    }

    // Use an atomic Once to register the check_leaks function to be called
    // when the process exits
    static ONCE: Once = Once::new();
    ONCE.call_once(|| unsafe {
        countme::enable(true);
        atexit(check_leaks);
    });
}

/// This check is used in the parser test to ensure it doesn't emit
/// bogus nodes without diagnostics, and in the analyzer tests to
/// check the syntax trees resulting from code actions are correct
pub fn has_bogus_nodes_or_empty_slots(node: &JsonSyntaxNode) -> bool {
    node.descendants().any(|descendant| {
        let kind = descendant.kind();
        if kind.is_bogus() {
            return true;
        }

        if kind.is_list() {
            return descendant
                .slots()
                .any(|slot| matches!(slot, SyntaxSlot::Empty));
        }

        false
    })
}

/// This function analyzes the parsing result of a file and panic with a
/// detailed message if it contains any error-level diagnostic, bogus nodes,
/// empty list slots or missing required children
pub fn assert_errors_are_absent(program: &JsonParse, path: &Path) {
    let syntax = program.syntax();
    let debug_tree = format!("{:?}", program.tree());
    let has_missing_children = debug_tree.contains("missing (required)");

    if !program.has_errors() && !has_bogus_nodes_or_empty_slots(&syntax) && !has_missing_children {
        return;
    }

    let mut buffer = Buffer::no_color();
    for diagnostic in program.diagnostics() {
        let error = diagnostic
            .clone()
            .with_file_path(path.to_str().unwrap())
            .with_file_source_code(syntax.to_string());
        Formatter::new(&mut Termcolor(&mut buffer))
            .write_markup(markup! {
                {PrintDiagnostic::verbose(&error)}
            })
            .unwrap();
    }

    panic!("There should be no errors in the file {:?} but the following errors where present:\n{}\n\nParsed tree:\n{:#?}",
		   path.display(),
		   std::str::from_utf8(buffer.as_slice()).unwrap(),
		   &syntax
	);
}
