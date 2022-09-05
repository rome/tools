use std::{borrow::Cow, io};

use rome_console::{
    codespan::{Codespan, Label, LabelStyle, Locus, Severity, SourceFile},
    fmt, markup,
};
use rome_text_edit::TextRange;

use crate::v2::{Location, Path};

/// Prints a code frame advice as a [Codespan]
pub(super) fn print_frame(fmt: &mut fmt::Formatter<'_>, location: Location<'_>) -> io::Result<()> {
    let source_span = location.source_code.as_ref().and_then(|source_code| {
        let span = location.span.as_ref()?;
        Some((&source_code.text, source_code.line_starts, span))
    });

    if let Some((source, line_starts, span)) = source_span {
        // Either re-use the existing line index provided by the diagnostic or create one
        let line_starts = line_starts
            .map(Cow::Borrowed)
            .unwrap_or_else(|| Cow::Owned(SourceFile::line_starts(source).collect()));

        let start = span.start();
        let end = span.end();

        let source_file = SourceFile::new(source, &line_starts);

        let locus = match location.path {
            Path::File(file) => file.path().map(|name| match source_file.location(start) {
                Ok(location) => Locus::FileLocation { name, location },
                Err(_) => Locus::File { name },
            }),
            _ => None,
        };

        fmt.write_markup(markup! {
            {Codespan {
                source_file,
                severity: Severity::Error,
                locus,
                labels: &[Label {
                    style: LabelStyle::Primary,
                    range: TextRange::new(start, end),
                    message: markup!().to_owned(),
                }],
            }}
            "\n"
        })?;
    }

    Ok(())
}
