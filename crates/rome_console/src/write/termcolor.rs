use std::{
    fmt::{self, Write as _},
    io,
};

use termcolor::{ColorSpec, WriteColor};
use unicode_width::UnicodeWidthChar;

use crate::fmt::MarkupElements;

use super::Write;

/// Adapter struct implementing [Write] over types implementing [WriteColor]
pub struct Termcolor<W>(pub W);

impl<W> Write for Termcolor<W>
where
    W: WriteColor,
{
    fn write_str(&mut self, elements: &MarkupElements, content: &str) -> io::Result<()> {
        with_format(&mut self.0, elements, |writer| {
            let mut adapter = SanitizeAdapter {
                writer,
                error: Ok(()),
            };

            match adapter.write_str(content) {
                Ok(()) => Ok(()),
                Err(..) => {
                    if adapter.error.is_err() {
                        adapter.error
                    } else {
                        // SanitizeAdapter can only fail if the underlying
                        // writer returns an error
                        unreachable!()
                    }
                }
            }
        })
    }

    fn write_fmt(&mut self, elements: &MarkupElements, content: fmt::Arguments) -> io::Result<()> {
        with_format(&mut self.0, elements, |writer| {
            let mut adapter = SanitizeAdapter {
                writer,
                error: Ok(()),
            };

            match adapter.write_fmt(content) {
                Ok(()) => Ok(()),
                Err(..) => {
                    if adapter.error.is_err() {
                        adapter.error
                    } else {
                        Err(io::Error::new(
                            io::ErrorKind::Other,
                            "a Display formatter returned an error",
                        ))
                    }
                }
            }
        })
    }
}

/// Applies the current format in `state` to `writer`, calls `func` to
/// print a piece of text, then reset the printing format
fn with_format<W>(
    writer: &mut W,
    state: &MarkupElements,
    func: impl FnOnce(&mut W) -> io::Result<()>,
) -> io::Result<()>
where
    W: WriteColor,
{
    let mut color = ColorSpec::new();
    state.for_each(&mut |elements| {
        for element in elements {
            element.update_color(&mut color);
        }
    });

    if let Err(err) = writer.set_color(&color) {
        writer.reset()?;
        return Err(err);
    }

    let result = func(writer);
    writer.reset()?;
    result
}

/// Adapter [fmt::Write] calls to [io::Write] with sanitization,
/// implemented as an internal struct to avoid exposing [fmt::Write] on
/// [Termcolor]
struct SanitizeAdapter<W> {
    writer: W,
    error: io::Result<()>,
}

impl<W: io::Write> fmt::Write for SanitizeAdapter<W> {
    fn write_str(&mut self, content: &str) -> fmt::Result {
        let mut buffer = [0; 4];

        for item in content.chars() {
            // Replace non-whitespace, zero-width characters with the Unicode replacement character
            let is_whitespace = item.is_whitespace();
            let is_zero_width = UnicodeWidthChar::width(item).map_or(true, |width| width == 0);
            let item = if !is_whitespace && is_zero_width {
                char::REPLACEMENT_CHARACTER
            } else {
                item
            };

            item.encode_utf8(&mut buffer);
            if let Err(err) = self.writer.write_all(&buffer[..item.len_utf8()]) {
                self.error = Err(err);
                return Err(fmt::Error);
            }
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use std::{fmt::Write, str::from_utf8};

    use super::SanitizeAdapter;

    #[test]
    fn test_sanitize() {
        // Sanitization should leave whitespace control characters (space,
        // tabs, newline, ...) and non-ASCII unicode characters as-is but
        // redact zero-width characters (RTL override, null character, bell,
        // zero-width space, ...)
        const INPUT: &str = "t\tes t\r\n\u{202D}t\0es\x07t\u{202E}\nt\u{200B}es🐛t";
        const OUTPUT: &str = "t\tes t\r\n\u{FFFD}t\u{FFFD}es\u{FFFD}t\u{FFFD}\nt\u{FFFD}es🐛t";

        let mut buffer = Vec::new();

        {
            let mut adapter = SanitizeAdapter {
                writer: &mut buffer,
                error: Ok(()),
            };

            adapter.write_str(INPUT).unwrap();
            adapter.error.unwrap();
        }

        assert_eq!(from_utf8(&buffer).unwrap(), OUTPUT);
    }
}
