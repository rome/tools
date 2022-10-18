use crate::ParseDiagnostic;
use rome_diagnostics::file::FileId;

pub fn invalid_digits_after_unicode_escape_sequence(
    file_id: FileId,
    start: usize,
    end: usize,
) -> ParseDiagnostic {
    ParseDiagnostic::new(file_id, "invalid digits after unicode escape sequence")
        .primary(start..end, "expected valid unicode escape sequence")
}
