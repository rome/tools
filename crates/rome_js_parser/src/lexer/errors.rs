use rome_diagnostics::{file::FileId, Diagnostic};

pub fn invalid_digits_after_unicode_escape_sequence(
    file_id: FileId,
    start: usize,
    end: usize,
) -> Diagnostic {
    Diagnostic::error(file_id, "", "invalid digits after unicode escape sequence")
        .primary(start..end, "expected valid unicode escape sequence")
}
