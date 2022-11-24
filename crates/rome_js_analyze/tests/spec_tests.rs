use json_comments::StripComments;
use rome_analyze::{
    AnalysisFilter, AnalyzerAction, AnalyzerOptions, ControlFlow, Never, RuleFilter, RuleKey,
};
use rome_console::{
    fmt::{Formatter, Termcolor},
    markup, Markup,
};
use rome_diagnostics::advice::CodeSuggestionAdvice;
use rome_diagnostics::location::FileId;
use rome_diagnostics::termcolor::NoColor;
use rome_diagnostics::{DiagnosticExt, Error, PrintDiagnostic, Severity};
use rome_js_parser::{
    parse,
    test_utils::{assert_errors_are_absent, has_bogus_nodes_or_empty_slots},
};
use rome_js_syntax::{JsLanguage, SourceType};
use similar::TextDiff;
use std::{
    ffi::OsStr, fmt::Write, fs::read_to_string, os::raw::c_int, path::Path, slice, sync::Once,
};

tests_macros::gen_tests! {"tests/specs/**/*.{cjs,js,jsx,tsx,ts,json,jsonc}", crate::run_test, "module"}
tests_macros::gen_tests! {"tests/suppression/**/*.{cjs,js,jsx,tsx,ts,json,jsonc}", crate::run_suppression_test, "module"}

fn scripts_from_json(extension: &OsStr, input_code: &str) -> Option<Vec<String>> {
    if extension == "json" || extension == "jsonc" {
        let input_code = StripComments::new(input_code.as_bytes());
        let scripts: Vec<String> = serde_json::from_reader(input_code).ok()?;
        Some(scripts)
    } else {
        None
    }
}

fn run_test(input: &'static str, _: &str, _: &str, _: &str) {
    register_leak_checker();

    let input_file = Path::new(input);
    let file_name = input_file.file_name().and_then(OsStr::to_str).unwrap();
    let input_code = read_to_string(input_file)
        .unwrap_or_else(|err| panic!("failed to read {:?}: {:?}", input_file, err));

    let (group, rule) = parse_test_path(input_file);

    let rule_filter = RuleFilter::Rule(group, rule);
    let filter = AnalysisFilter {
        enabled_rules: Some(slice::from_ref(&rule_filter)),
        ..AnalysisFilter::default()
    };

    let mut snapshot = String::new();
    let extension = input_file.extension().unwrap_or_default();

    if let Some(scripts) = scripts_from_json(extension, &input_code) {
        for script in scripts {
            write_analysis_to_snapshot(
                &mut snapshot,
                &script,
                SourceType::js_script(),
                filter,
                file_name,
                input_file,
                CheckActionType::Lint,
            )
        }
    } else {
        let Ok(source_type) = input_file.try_into() else {
            return;
        };
        write_analysis_to_snapshot(
            &mut snapshot,
            &input_code,
            source_type,
            filter,
            file_name,
            input_file,
            CheckActionType::Lint,
        );
    }

    insta::with_settings!({
        prepend_module_to_snapshot => false,
        snapshot_path => input_file.parent().unwrap(),
    }, {
        insta::assert_snapshot!(file_name, snapshot, file_name);
    });
}

enum CheckActionType {
    Suppression,
    Lint,
}

impl CheckActionType {
    const fn is_suppression(&self) -> bool {
        matches!(self, Self::Suppression)
    }
}

pub(crate) fn write_analysis_to_snapshot(
    snapshot: &mut String,
    input_code: &str,
    source_type: SourceType,
    filter: AnalysisFilter,
    file_name: &str,
    input_file: &Path,
    check_action_type: CheckActionType,
) {
    let parsed = parse(input_code, FileId::zero(), source_type);
    let root = parsed.tree();

    let mut diagnostics = Vec::new();
    let mut code_fixes = Vec::new();
    let mut options = AnalyzerOptions::default();

    // We allow a test file to configure its rule using a special
    // file with the same name as the test but with extension ".options.json"
    // that configures that specific rule.
    let options_file = input_file.with_extension("options.json");
    if let Ok(json) = std::fs::read_to_string(options_file) {
        let v: serde_json::Value = serde_json::from_str(&json).expect("must be a valid JSON");

        //RuleKey needs 'static string, so we must leak them here
        let (group, rule) = parse_test_path(input_file);
        let group = Box::leak(Box::new(group.to_string()));
        let rule = Box::leak(Box::new(rule.to_string()));
        let rule_key = RuleKey::new(group, rule);

        options.configuration.rules.push_rule(rule_key, v);
    }

    rome_js_analyze::analyze(FileId::zero(), &root, filter, &options, |event| {
        if let Some(mut diag) = event.diagnostic() {
            for action in event.actions() {
                if check_action_type.is_suppression() {
                    if action.is_suppression() {
                        check_code_action(input_file, input_code, source_type, &action);
                        diag = diag.add_code_suggestion(CodeSuggestionAdvice::from(action));
                    }
                } else if !action.is_suppression() {
                    check_code_action(input_file, input_code, source_type, &action);
                    diag = diag.add_code_suggestion(CodeSuggestionAdvice::from(action));
                }
            }
            let error = diag.with_severity(Severity::Warning);
            diagnostics.push(diagnostic_to_string(file_name, input_code, error));
            return ControlFlow::Continue(());
        }

        for action in event.actions() {
            if check_action_type.is_suppression() {
                if action.category.matches("quickfix.suppressRule") {
                    check_code_action(input_file, input_code, source_type, &action);
                    code_fixes.push(code_fix_to_string(input_code, action));
                }
            } else if !action.category.matches("quickfix.suppressRule") {
                check_code_action(input_file, input_code, source_type, &action);
                code_fixes.push(code_fix_to_string(input_code, action));
            }
        }

        ControlFlow::<Never>::Continue(())
    });

    writeln!(snapshot, "# Input").unwrap();
    writeln!(snapshot, "```js").unwrap();
    writeln!(snapshot, "{}", input_code).unwrap();
    writeln!(snapshot, "```").unwrap();
    writeln!(snapshot).unwrap();

    if !diagnostics.is_empty() {
        writeln!(snapshot, "# Diagnostics").unwrap();
        for diagnostic in diagnostics {
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
}

/// The test runner for the analyzer is currently designed to have a
/// one-to-one mapping between test case and analyzer rules, so each testing
/// file will be run through the analyzer with only the rule corresponding
/// to the file name (or the name of the parent directory if it's not "specs")
/// enabled, eg. `correctness/useWhile.js` and `correctness/useWhile/test.js` will be analyzed with
/// just the `correctness-ignore lint(correctness/useW/useWhile` rule
fn parse_test_path(file: &Path) -> (&str, &str) {
    let file_stem = file.file_stem().unwrap();

    let ancestor_0 = file.parent().unwrap();
    let name_0 = ancestor_0.file_name().unwrap();

    let ancestor_1 = ancestor_0.parent().unwrap();
    let name_1 = ancestor_1.file_name().unwrap();

    if matches!(name_1.to_str().unwrap(), "specs" | "suppression") {
        (name_0.to_str().unwrap(), file_stem.to_str().unwrap())
    } else {
        (name_1.to_str().unwrap(), name_0.to_str().unwrap())
    }
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
    let error = diag
        .with_file_path((name, FileId::zero()))
        .with_file_source_code(source);
    let text = markup_to_string(markup! {
        {PrintDiagnostic::verbose(&error)}
    });

    text
}

fn check_code_action(
    path: &Path,
    source: &str,
    source_type: SourceType,
    action: &AnalyzerAction<JsLanguage>,
) {
    let (_, text_edit) = action.mutation.as_text_edits().unwrap_or_default();

    let output = text_edit.new_string(source);

    let new_tree = action.mutation.clone().commit();

    // Checks that applying the text edits returned by the BatchMutation
    // returns the same code as printing the modified syntax tree
    assert_eq!(new_tree.to_string(), output);

    if has_bogus_nodes_or_empty_slots(&new_tree) {
        panic!("modified tree has bogus nodes or empty slots:\n{new_tree:#?}")
    }

    // Checks the returned tree contains no missing children node
    if format!("{new_tree:?}").contains("missing (required)") {
        panic!("modified tree has missing children:\n{new_tree:#?}")
    }

    // Re-parse the modified code and panic if the resulting tree has syntax errors
    let re_parse = parse(&output, FileId::zero(), source_type);
    assert_errors_are_absent(&re_parse, path);
}

fn code_fix_to_string(source: &str, action: AnalyzerAction<JsLanguage>) -> String {
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

pub(crate) fn run_suppression_test(input: &'static str, _: &str, _: &str, _: &str) {
    register_leak_checker();

    let input_file = Path::new(input);
    let file_name = input_file.file_name().and_then(OsStr::to_str).unwrap();
    let input_code = read_to_string(input_file)
        .unwrap_or_else(|err| panic!("failed to read {:?}: {:?}", input_file, err));

    let (group, rule) = parse_test_path(input_file);

    let rule_filter = RuleFilter::Rule(group, rule);
    let filter = AnalysisFilter {
        enabled_rules: Some(slice::from_ref(&rule_filter)),
        ..AnalysisFilter::default()
    };

    dbg!(&rule_filter);
    let mut snapshot = String::new();
    write_analysis_to_snapshot(
        &mut snapshot,
        &input_code,
        SourceType::jsx(),
        filter,
        file_name,
        input_file,
        CheckActionType::Suppression,
    );

    insta::with_settings!({
        prepend_module_to_snapshot => false,
        snapshot_path => input_file.parent().unwrap(),
    }, {
        insta::assert_snapshot!(file_name, snapshot, file_name);
    });
}
