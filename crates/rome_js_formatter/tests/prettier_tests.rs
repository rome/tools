use parking_lot::{const_mutex, Mutex};
use rome_rowan::{TextRange, TextSize};
use similar::{utils::diff_lines, Algorithm, ChangeTag, TextDiff};
use std::fs::remove_file;
use std::{
    env,
    ffi::OsStr,
    fmt::Write,
    fs::{read_to_string, write},
    ops::Range,
    os::raw::c_int,
    path::Path,
    str::FromStr,
    sync::Once,
};

use rome_diagnostics::{file::SimpleFiles, termcolor, Emitter};
use rome_formatter::IndentStyle;
use rome_js_formatter::context::JsFormatContext;
use rome_js_parser::parse;
use rome_js_syntax::SourceType;
use serde::Serialize;

use crate::check_reformat::CheckReformatParams;

#[derive(serde::Serialize)]
struct TestInfo {
    test_file: String,
}

mod check_reformat;

tests_macros::gen_tests! {"tests/specs/prettier/{js,typescript}/**/*.{js,ts,jsx,tsx}", crate::test_snapshot, "script"}

const PRETTIER_IGNORE: &str = "prettier-ignore";
const ROME_IGNORE: &str = "rome-ignore format: prettier ignore";

fn test_snapshot(input: &'static str, _: &str, _: &str, _: &str) {
    countme::enable(true);

    if input.contains("flow") || input.contains("prepare_tests") {
        return;
    }

    let input_file = Path::new(input);
    let file_name = input_file.file_name().and_then(OsStr::to_str).unwrap();
    let mut input_code = read_to_string(input_file)
        .unwrap_or_else(|err| panic!("failed to read {:?}: {:?}", input_file, err));

    let (_, range_start_index, range_end_index) = strip_placeholders(&mut input_code);
    let parse_input = input_code.replace(PRETTIER_IGNORE, ROME_IGNORE);

    // Prettier testing suite uses JSX tags inside JS files.
    // As there's no way to know in advance which files have JSX syntax, we
    // change the source type only here
    let source_type = if input_file.extension().unwrap() == "js" {
        SourceType::jsx()
    } else if file_name.contains("jsx") && input_file.extension() == Some(OsStr::new("ts")) {
        SourceType::tsx()
    } else {
        input_file.try_into().unwrap()
    };

    let parsed = parse(&parse_input, 0, source_type);

    let has_errors = parsed.has_errors();
    let syntax = parsed.syntax();

    let context = JsFormatContext::default().with_indent_style(IndentStyle::Space(2));

    let result = match (range_start_index, range_end_index) {
        (Some(start), Some(end)) => {
            // Skip the reversed range tests as its impossible
            // to create a reversed TextRange anyway
            if end < start {
                return;
            }

            rome_js_formatter::format_range(
                context.clone(),
                &syntax,
                TextRange::new(
                    TextSize::try_from(start).unwrap(),
                    TextSize::try_from(end).unwrap(),
                ),
            )
        }
        _ => rome_js_formatter::format_node(context.clone(), &syntax)
            .map(|formatted| formatted.print()),
    };

    let formatted = result.expect("formatting failed");
    let formatted = match (range_start_index, range_end_index) {
        (Some(_), Some(_)) => {
            let range = formatted
                .range()
                .expect("the result of format_range should have a range");

            let formatted = formatted.as_code();
            let mut output_code = parse_input.clone();
            output_code.replace_range(Range::<usize>::from(range), formatted);
            output_code
        }
        _ => {
            let result = formatted.into_code();

            if !has_errors {
                check_reformat::check_reformat(CheckReformatParams {
                    root: &syntax,
                    text: &result,
                    source_type,
                    file_name,
                    format_context: context.clone(),
                });
            }

            result
        }
    };

    let formatted = formatted.replace(ROME_IGNORE, PRETTIER_IGNORE);

    let root_path = Path::new(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/tests/specs/prettier/"
    ));

    let relative_file_name = input_file
        .strip_prefix(root_path)
        .unwrap_or_else(|_| {
            panic!(
                "failed to strip prefix {:?} from {:?}",
                root_path, input_file
            )
        })
        .to_str()
        .unwrap();

    let input_extension = input_file.extension().and_then(OsStr::to_str);

    let prettier_snapshot_path = input_extension
        .map(|ext| input_file.with_extension(format!("{}.prettier-snap", ext)))
        .filter(|path| path.exists());

    let prettier_diff = match prettier_snapshot_path {
        Some(prettier_snapshot_path) => {
            let mut prettier_snapshot = read_to_string(prettier_snapshot_path).unwrap();

            strip_placeholders(&mut prettier_snapshot);

            DiffReport::get().report(relative_file_name, &formatted, &prettier_snapshot);

            if formatted == prettier_snapshot {
                // The output matches prettier's output. There's no need for a snapshot that duplicates the output.
                // Delete the snapshot file if it already exists, otherwise return early to not create a new snapshot.
                if let Some(input_extension) = input_extension {
                    let snapshot_file_name =
                        input_file.with_extension(format!("{}.snap", input_extension));

                    if snapshot_file_name.exists() && snapshot_file_name.is_file() {
                        remove_file(snapshot_file_name).ok(); // not the end of the world if it fails
                    }

                    let new_snapshot_file_name =
                        input_file.with_extension(format!("{}.snap.new", input_extension));
                    if new_snapshot_file_name.exists() && new_snapshot_file_name.is_file() {
                        remove_file(new_snapshot_file_name).ok(); // not the end of the world if it fails
                    }
                }

                return;
            } else {
                let mut prettier_differences = Vec::new();

                TextDiff::from_lines(&prettier_snapshot, &formatted)
                    .unified_diff()
                    .header("Prettier", "Rome")
                    .to_writer(&mut prettier_differences)
                    .unwrap();

                Some(String::from_utf8(prettier_differences).expect("Input file to be in UTF8"))
            }
        }
        None => None,
    };

    let mut snapshot = String::new();
    writeln!(snapshot).unwrap();
    writeln!(snapshot, "# Input").unwrap();
    writeln!(snapshot).unwrap();
    writeln!(snapshot, "```js").unwrap();
    snapshot.push_str(&input_code);
    writeln!(snapshot, "```").unwrap();
    writeln!(snapshot).unwrap();
    writeln!(snapshot).unwrap();

    if let Some(prettier_diff) = prettier_diff {
        writeln!(snapshot, "# Prettier differences").unwrap();
        writeln!(snapshot).unwrap();
        writeln!(snapshot, "```diff").unwrap();
        snapshot.push_str(&prettier_diff);
        writeln!(snapshot, "```").unwrap();
        writeln!(snapshot).unwrap();
    }

    writeln!(snapshot, "# Output").unwrap();
    writeln!(snapshot).unwrap();
    writeln!(snapshot, "```js").unwrap();
    snapshot.push_str(&formatted);
    writeln!(snapshot, "```").unwrap();
    writeln!(snapshot).unwrap();
    writeln!(snapshot).unwrap();

    if has_errors {
        let mut files = SimpleFiles::new();
        files.add(file_name.into(), parse_input);

        let mut buffer = termcolor::Buffer::no_color();
        let mut emitter = Emitter::new(&files);

        for error in parsed.diagnostics() {
            emitter
                .emit_with_writer(error, &mut buffer)
                .expect("failed to emit diagnostic");
        }

        writeln!(snapshot, "# Errors").unwrap();
        writeln!(snapshot, "```").unwrap();
        writeln!(
            snapshot,
            "{}",
            std::str::from_utf8(buffer.as_slice()).expect("non utf8 in error buffer")
        )
        .unwrap();
        writeln!(snapshot, "```").unwrap();
        writeln!(snapshot).unwrap();
    }

    let max_width = context.line_width().value() as usize;
    let mut lines_exceeding_max_width = formatted
        .lines()
        .enumerate()
        .filter(|(_, line)| line.len() > max_width)
        .peekable();

    if lines_exceeding_max_width.peek().is_some() {
        writeln!(
            snapshot,
            "# Lines exceeding max width of {max_width} characters"
        )
        .unwrap();
        writeln!(snapshot, "```").unwrap();

        for (index, line) in lines_exceeding_max_width {
            let line_number = index + 1;
            writeln!(snapshot, "{line_number:>5}: {line}").unwrap();
        }
        writeln!(snapshot, "```").unwrap();
    }

    let info = TestInfo {
        test_file: relative_file_name.to_owned(),
    };

    insta::with_settings!({
        prepend_module_to_snapshot => false,
        snapshot_path => input_file.parent().unwrap(),
        omit_expression => true,
        info => &info
    }, {
        insta::assert_snapshot!(file_name, snapshot);
    });
}

/// Find and replace the cursor, range start and range end placeholders in a
/// Prettier snapshot tests and return their indices in the resulting string
fn strip_placeholders(input_code: &mut String) -> (Option<usize>, Option<usize>, Option<usize>) {
    const CURSOR_PLACEHOLDER: &str = "<|>";
    const RANGE_START_PLACEHOLDER: &str = "<<<PRETTIER_RANGE_START>>>";
    const RANGE_END_PLACEHOLDER: &str = "<<<PRETTIER_RANGE_END>>>";

    let mut cursor_index = None;
    let mut range_start_index = None;
    let mut range_end_index = None;

    if let Some(index) = input_code.find(CURSOR_PLACEHOLDER) {
        input_code.replace_range(index..index + CURSOR_PLACEHOLDER.len(), "");
        cursor_index = Some(index);
    }

    if let Some(index) = input_code.find(RANGE_START_PLACEHOLDER) {
        input_code.replace_range(index..index + RANGE_START_PLACEHOLDER.len(), "");
        range_start_index = Some(index);

        if let Some(cursor) = &mut cursor_index {
            if *cursor > index {
                *cursor -= RANGE_START_PLACEHOLDER.len();
            }
        }
    }

    if let Some(index) = input_code.find(RANGE_END_PLACEHOLDER) {
        input_code.replace_range(index..index + RANGE_END_PLACEHOLDER.len(), "");
        range_end_index = Some(index);

        if let Some(cursor) = &mut cursor_index {
            if *cursor > index {
                *cursor -= RANGE_END_PLACEHOLDER.len();
            }
        }
        if let Some(cursor) = &mut range_start_index {
            // Prettier has tests for reversed ranges
            if *cursor > index {
                *cursor -= RANGE_END_PLACEHOLDER.len();
            }
        }
    }

    (cursor_index, range_start_index, range_end_index)
}

#[derive(Debug, PartialEq, Eq)]
enum ReportType {
    Json,
    Markdown,
}

#[derive(Debug, Clone, Default, Serialize)]
struct SingleFileMetricData {
    filename: String,
    single_file_compatibility: f64,
    #[serde(skip)]
    diff: Option<String>,
}

#[derive(Clone, Debug, Default, Serialize)]
struct PrettierCompatibilityMetricData {
    file_based_average_prettier_similarity: f64,
    line_based_average_prettier_similarity: f64,
    files: Vec<SingleFileMetricData>,
}

impl FromStr for ReportType {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "json" => Ok(Self::Json),
            "markdown" => Ok(Self::Markdown),
            _ => Err("Only `json` and `markdown` are supported".to_string()),
        }
    }
}

struct DiffReportItem {
    file_name: &'static str,
    rome_formatted_result: String,
    prettier_formatted_result: String,
}
struct DiffReport {
    state: Mutex<Vec<DiffReportItem>>,
}

impl DiffReport {
    fn get() -> &'static Self {
        static REPORTER: DiffReport = DiffReport {
            state: const_mutex(Vec::new()),
        };

        // Use an atomic Once to register an exit callback the first time any
        // testing thread requests an instance of the Reporter
        static ONCE: Once = Once::new();
        ONCE.call_once(|| {
            // Import the atexit function from libc
            extern "C" {
                fn atexit(f: extern "C" fn()) -> c_int;
            }

            // Trampoline function into the reporter printing logic with the
            // correct extern C ABI
            extern "C" fn print_report() {
                REPORTER.print();
            }

            // Register the print_report function to be called when the process exits
            unsafe {
                atexit(print_report);
            }
        });

        &REPORTER
    }

    fn report(
        &self,
        file_name: &'static str,
        rome_formatted_result: &str,
        prettier_formatted_result: &str,
    ) {
        match env::var("REPORT_PRETTIER") {
            Ok(value) if value == "1" => {
                self.state.lock().push(DiffReportItem {
                    file_name,
                    rome_formatted_result: rome_formatted_result.to_owned(),
                    prettier_formatted_result: prettier_formatted_result.to_owned(),
                });
            }
            _ => {}
        }
    }
    fn print(&self) {
        if let Some(report) = rome_rowan::check_live() {
            panic!("\n{report}")
        }
        // Only create the report file if the REPORT_PRETTIER
        // environment variable is set to 1
        match env::var("REPORT_PRETTIER") {
            Ok(value) if value == "1" => {
                let report_type = match env::var("REPORT_TYPE") {
                    Ok(value) => ReportType::from_str(&value).unwrap(),
                    _ => ReportType::Markdown,
                };
                let report_filename = match env::var("REPORT_FILENAME") {
                    Ok(value) => value,
                    _ => match report_type {
                        ReportType::Json => "report.json".to_string(),
                        ReportType::Markdown => "report.md".to_string(),
                    },
                };
                self.report_prettier(report_type, report_filename);
            }
            _ => {}
        }
    }

    fn report_prettier(&self, report_type: ReportType, report_filename: String) {
        let mut state = self.state.lock();
        state.sort_by_key(|DiffReportItem { file_name, .. }| *file_name);

        let mut report_metric_data = PrettierCompatibilityMetricData::default();
        let mut file_ratio_sum = 0_f64;
        let mut total_lines = 0;
        let mut total_matched_lines = 0;
        let mut file_count = 0;

        for DiffReportItem {
            file_name,
            rome_formatted_result,
            prettier_formatted_result,
        } in state.iter()
        {
            file_count += 1;

            let rome_lines = rome_formatted_result.lines().count();
            let prettier_lines = prettier_formatted_result.lines().count();

            let (matched_lines, ratio, diff) = if rome_formatted_result == prettier_formatted_result
            {
                (rome_lines, 1f64, None)
            } else {
                let mut matched_lines = 0;
                let mut diff = String::new();

                for (tag, line) in diff_lines(
                    Algorithm::default(),
                    prettier_formatted_result,
                    rome_formatted_result,
                ) {
                    if matches!(tag, ChangeTag::Equal) {
                        matched_lines += 1;
                    }

                    let line = line.strip_suffix('\n').unwrap_or(line);
                    writeln!(diff, "{}{}", tag, line).unwrap();
                }

                let ratio = matched_lines as f64 / rome_lines.max(prettier_lines) as f64;

                (matched_lines, ratio, Some(diff))
            };

            total_lines += rome_lines.max(prettier_lines);
            total_matched_lines += matched_lines;
            file_ratio_sum += ratio;

            let single_file_metric_data = SingleFileMetricData {
                diff,
                filename: file_name.to_string(),
                single_file_compatibility: ratio,
            };

            report_metric_data.files.push(single_file_metric_data);
        }

        report_metric_data.file_based_average_prettier_similarity =
            file_ratio_sum / file_count as f64;
        report_metric_data.line_based_average_prettier_similarity =
            total_matched_lines as f64 / total_lines as f64;

        match report_type {
            ReportType::Json => self.report_json(report_filename, report_metric_data),
            ReportType::Markdown => self.report_markdown(report_filename, report_metric_data),
        }
    }

    fn report_markdown(
        &self,
        report_filename: String,
        report_metric_data: PrettierCompatibilityMetricData,
    ) {
        let mut report = String::new();
        for SingleFileMetricData {
            filename,
            single_file_compatibility,
            diff,
        } in report_metric_data.files.iter()
        {
            writeln!(report, "# {}", filename).unwrap();

            if let Some(diff) = diff {
                writeln!(report, "```diff").unwrap();
                writeln!(report, "{diff}").unwrap();
                writeln!(report, "```").unwrap()
            }
            writeln!(
                report,
                "**Prettier Similarity**: {:.2}%",
                single_file_compatibility * 100_f64
            )
            .unwrap();
        }
        // extra two space force markdown render insert a new line
        report = format!(
            "**File Based Average Prettier Similarity**: {:.2}%  \n**Line Based Average Prettier Similarity**: {:.2}%  \nthe definition of similarity you could found here: https://github.com/rome/tools/issues/2555#issuecomment-1124787893\n",
            report_metric_data.file_based_average_prettier_similarity * 100_f64,
            report_metric_data.line_based_average_prettier_similarity * 100_f64
        ) + &report;
        write(report_filename, report).unwrap();
    }

    fn report_json(
        &self,
        report_filename: String,
        report_metric_data: PrettierCompatibilityMetricData,
    ) {
        let json_content = serde_json::to_string(&report_metric_data).unwrap();
        write(report_filename, json_content).unwrap();
    }
}
