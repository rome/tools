use std::{
    ffi::OsStr, fmt::Write, fs::read_to_string, os::raw::c_int, path::Path, slice, sync::Once,
};

use rome_analyze::{AnalysisFilter, AnalyzerAction, ControlFlow, Never};
use rome_console::{
    diff::{Diff, DiffMode},
    fmt::{Formatter, Termcolor},
    markup, Markup,
};
use rome_diagnostics::{file::SimpleFile, termcolor::NoColor, Diagnostic};
use rome_js_parser::parse;
use rome_rowan::{AstNode, Language};

tests_macros::gen_tests! {"tests/specs/**/*.js", crate::run_test, "module"}

fn run_test(input: &'static str, _: &str, _: &str, _: &str) {
    register_leak_checker();

    let input_file = Path::new(input);
    let file_name = input_file.file_name().and_then(OsStr::to_str).unwrap();
    let input_code = read_to_string(input_file)
        .unwrap_or_else(|err| panic!("failed to read {:?}: {:?}", input_file, err));

    let source_type = input_file.try_into().unwrap();
    let parsed = parse(&input_code, 0, source_type);
    let root = parsed.tree();

    // The test runner for the analyzer is currently designed to have a
    // one-to-one mapping between test case and analyzer rules, so each testing
    // file will be run through the analyzer with only the rule corresponding
    // to the file name (or the name of the parent directory if it's not "specs")
    // enabled, eg. `useWhile.js` and `useWhile/test.js` will be analyzed with
    // just the `useWhile` rule
    let rule_name = input_file
        .parent()
        .and_then(|parent| parent.file_name()?.to_str())
        .filter(|parent| *parent != "specs")
        .or_else(|| input_file.file_stem()?.to_str())
        .unwrap();
    let filter = AnalysisFilter {
        rules: Some(slice::from_ref(&rule_name)),
        ..AnalysisFilter::default()
    };

    let mut diagnostics = Vec::new();
    let mut code_fixes = Vec::new();

    rome_analyze::analyze(0, &root, filter, |event| {
        if let Some(mut diag) = event.diagnostic() {
            if let Some(action) = event.action() {
                diag.suggestions.push(action.into());
            }

            diagnostics.push(diagnostic_to_string(file_name, &input_code, diag));
            return ControlFlow::Continue(());
        }

        if let Some(action) = event.action() {
            code_fixes.push(code_fix_to_string(&input_code, action));
        }

        ControlFlow::<Never>::Continue(())
    });

    let mut snapshot = String::new();

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
            writeln!(snapshot, "```").unwrap();
            writeln!(snapshot, "{}", action).unwrap();
            writeln!(snapshot, "```").unwrap();
            writeln!(snapshot).unwrap();
        }
    }

    insta::with_settings!({
        prepend_module_to_snapshot => false,
        snapshot_path => input_file.parent().unwrap(),
    }, {
        insta::assert_snapshot!(file_name, snapshot, file_name);
    });
}

fn markup_to_string(markup: Markup) -> String {
    let mut buffer = Vec::new();
    let mut write = Termcolor(NoColor::new(&mut buffer));
    let mut fmt = Formatter::new(&mut write);
    fmt.write_markup(markup).unwrap();

    String::from_utf8(buffer).unwrap()
}

fn diagnostic_to_string(name: &str, source: &str, diag: Diagnostic) -> String {
    let file = SimpleFile::new(name.into(), source.into());
    let text = markup_to_string(markup! {
        {diag.display(&file)}
    });

    text
}

fn code_fix_to_string<L>(source: &str, action: AnalyzerAction<L>) -> String
where
    L: Language,
{
    let output = action.root.syntax().to_string();
    markup_to_string(markup! {
        {Diff { mode: DiffMode::Unified, left: source, right: &output }}
    })
}

// Check that all red / green nodes have correctly been released on exit
extern "C" fn check_leaks() {
    if let Some(report) = rome_rowan::check_live() {
        panic!("\n{report}")
    }
}

fn register_leak_checker() {
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
