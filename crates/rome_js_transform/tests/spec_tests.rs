use rome_analyze::{AnalysisFilter, AnalyzerTransformation, ControlFlow, Never, RuleFilter};
use rome_js_formatter::context::JsFormatOptions;
use rome_js_formatter::format_node;
use rome_js_parser::{parse, JsParserOptions};
use rome_js_syntax::{JsFileSource, JsLanguage};
use rome_rowan::AstNode;
use rome_test_utils::{
    assert_errors_are_absent, create_analyzer_options, diagnostic_to_string,
    has_bogus_nodes_or_empty_slots, register_leak_checker, scripts_from_json,
    write_transformation_snapshot,
};
use std::{ffi::OsStr, fs::read_to_string, path::Path, slice};

tests_macros::gen_tests! {"tests/specs/**/*.{cjs,js,jsx,tsx,ts,json,jsonc}", crate::run_test, "module"}

fn run_test(input: &'static str, _: &str, _: &str, _: &str) {
    register_leak_checker();

    let input_file = Path::new(input);
    let file_name = input_file.file_name().and_then(OsStr::to_str).unwrap();

    let rule_folder = input_file.parent().unwrap();
    let rule = rule_folder.file_name().unwrap().to_str().unwrap();

    if rule == "specs" {
        panic!("the test file must be placed in the {rule}/<group-name>/<rule-name>/ directory");
    }
    if rome_js_transform::metadata()
        .find_rule("transformations", rule)
        .is_none()
    {
        panic!("could not find rule transformations/{rule}");
    }

    let rule_filter = RuleFilter::Rule("transformations", rule);
    let filter = AnalysisFilter {
        enabled_rules: Some(slice::from_ref(&rule_filter)),
        ..AnalysisFilter::default()
    };

    let mut snapshot = String::new();
    let extension = input_file.extension().unwrap_or_default();

    let input_code = read_to_string(input_file)
        .unwrap_or_else(|err| panic!("failed to read {:?}: {:?}", input_file, err));
    let quantity_diagnostics = if let Some(scripts) = scripts_from_json(extension, &input_code) {
        for script in scripts {
            analyze_and_snap(
                &mut snapshot,
                &script,
                JsFileSource::js_script(),
                filter,
                file_name,
                input_file,
                JsParserOptions::default(),
            );
        }

        0
    } else {
        let Ok(source_type) = input_file.try_into() else {
            return;
        };
        analyze_and_snap(
            &mut snapshot,
            &input_code,
            source_type,
            filter,
            file_name,
            input_file,
            JsParserOptions::default(),
        )
    };

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

#[allow(clippy::too_many_arguments)]
pub(crate) fn analyze_and_snap(
    snapshot: &mut String,
    input_code: &str,
    source_type: JsFileSource,
    filter: AnalysisFilter,
    file_name: &str,
    input_file: &Path,
    parser_options: JsParserOptions,
) -> usize {
    let parsed = parse(input_code, source_type, parser_options.clone());
    let root = parsed.tree();

    let mut diagnostics = Vec::new();
    let options = create_analyzer_options(input_file, &mut diagnostics);

    let mut transformations = vec![];
    let (_, errors) = rome_js_transform::transform(&root, filter, &options, source_type, |event| {
        for transformation in event.transformations() {
            check_transformation(
                input_file,
                input_code,
                source_type,
                &transformation,
                parser_options.clone(),
            );
            let node = transformation.mutation.commit();

            let formatted = format_node(JsFormatOptions::new(source_type), &node).unwrap();

            transformations.push(formatted.print().unwrap().as_code().to_string());
        }
        ControlFlow::<Never>::Continue(())
    });

    for error in errors {
        diagnostics.push(diagnostic_to_string(file_name, input_code, error));
    }

    write_transformation_snapshot(
        snapshot,
        input_code,
        transformations.as_slice(),
        source_type.file_extension(),
    );

    diagnostics.len()
}

fn check_transformation(
    path: &Path,
    source: &str,
    source_type: JsFileSource,
    transformation: &AnalyzerTransformation<JsLanguage>,
    options: JsParserOptions,
) {
    let (_, text_edit) = transformation.mutation.as_text_edits().unwrap_or_default();

    let output = text_edit.new_string(source);

    let new_tree = transformation.mutation.clone().commit();

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
    let re_parse = parse(&output, source_type, options);
    assert_errors_are_absent(re_parse.tree().syntax(), re_parse.diagnostics(), path);
}
