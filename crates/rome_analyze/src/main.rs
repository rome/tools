use core::slice;

use rome_analyze::AnalysisFilter;
use rome_js_parser::parse;
use rome_js_syntax::SourceType;

fn main() {
    let input_code = r#"
    let b = [1, ,2];
    
    
    "#;
    println!("{}", input_code);
    let source_type = SourceType::js_module();
    let parsed = parse(&input_code, 0, source_type);
    let root = parsed.tree();
    let filter = AnalysisFilter {
        rules: Some(slice::from_ref(&"noSparseArray")),
        ..AnalysisFilter::default()
    };

    let mut diagnostics = Vec::new();
    let mut code_fixes = Vec::new();

    rome_analyze::analyze(0, &root, filter, |event| {
        if let Some(mut diag) = event.diagnostic() {
            if let Some(action) = event.action() {
                diag.suggestions.push(action.into());
            }

            diagnostics.push(diag);
            return;
        }

        if let Some(action) = event.action() {
            code_fixes.push(action);
        }
    });
}

// fn diagnostic_to_string(name: &str, source: &str, diag: Diagnostic) -> String {
//     let file = SimpleFile::new(name.into(), source.into());
//     let text = markup_to_string(markup! {
//         {diag.display(&file)}
//     });

//     text
// }

// fn code_fix_to_string(source: &str, action: AnalyzerAction) -> String {
//     let output = action.root.syntax().to_string();

//     markup_to_string(markup! {
//         {Diff { mode: DiffMode::Unified, left: source, right: &output }}
//     })
// }
