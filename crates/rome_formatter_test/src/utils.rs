use crate::diff_report::DiffReport;
use similar::TextDiff;
use std::ffi::OsStr;
use std::fs::{read_to_string, remove_file};
use std::path::Path;

/// Find and replace the cursor, range start and range end placeholders in a
/// Prettier snapshot tests and return their indices in the resulting string
pub fn strip_prettier_placeholders(
    input_code: &mut String,
) -> (Option<usize>, Option<usize>, Option<usize>) {
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

pub enum PrettierDiff {
    Diff(String),
    Same,
}

pub fn get_prettier_diff(
    input_file: &Path,
    relative_file_name: &'static str,
    formatted: &str,
) -> PrettierDiff {
    let input_extension = input_file.extension().and_then(OsStr::to_str);

    let prettier_snapshot_path = input_extension
        .map(|ext| input_file.with_extension(format!("{}.prettier-snap", ext)))
        .filter(|path| path.exists());

    let prettier_snapshot_path = prettier_snapshot_path.expect("failed to find prettier snapshot");

    let mut prettier_snapshot = read_to_string(prettier_snapshot_path).unwrap();

    strip_prettier_placeholders(&mut prettier_snapshot);

    DiffReport::get().report(relative_file_name, formatted, &prettier_snapshot);

    if formatted == prettier_snapshot {
        // The output matches prettier's output. There's no need for a snapshot that duplicates the output.
        // Delete the snapshot file if it already exists, otherwise return early to not create a new snapshot.
        if let Some(input_extension) = input_extension {
            let snapshot_file_name = input_file.with_extension(format!("{}.snap", input_extension));

            if snapshot_file_name.exists() && snapshot_file_name.is_file() {
                remove_file(snapshot_file_name).ok(); // not the end of the world if it fails
            }

            let new_snapshot_file_name =
                input_file.with_extension(format!("{}.snap.new", input_extension));
            if new_snapshot_file_name.exists() && new_snapshot_file_name.is_file() {
                remove_file(new_snapshot_file_name).ok(); // not the end of the world if it fails
            }
        }

        PrettierDiff::Same
    } else {
        let mut prettier_differences = Vec::new();

        TextDiff::from_lines(prettier_snapshot.as_str(), formatted)
            .unified_diff()
            .header("Prettier", "Rome")
            .to_writer(&mut prettier_differences)
            .unwrap();

        PrettierDiff::Diff(
            String::from_utf8(prettier_differences).expect("Input file to be in UTF8"),
        )
    }
}
