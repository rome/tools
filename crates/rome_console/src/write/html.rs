use std::{fmt, io};

use crate::{fmt::MarkupElements, MarkupElement};

use super::Write;

/// Adapter struct implementing [Write] over types implementing [io::Write],
/// renders markup as UTF-8 strings of HTML code
pub struct HTML<W>(pub W);

impl<W> Write for HTML<W>
where
    W: io::Write,
{
    fn write_str(&mut self, elements: &MarkupElements, content: &str) -> io::Result<()> {
        push_styles(&mut self.0, elements)?;
        self.0.write_all(content.as_bytes())?;
        pop_styles(&mut self.0, elements)
    }

    fn write_fmt(&mut self, elements: &MarkupElements, content: fmt::Arguments) -> io::Result<()> {
        push_styles(&mut self.0, elements)?;
        self.0.write_fmt(content)?;
        pop_styles(&mut self.0, elements)
    }
}

fn push_styles<W: io::Write>(fmt: &mut W, elements: &MarkupElements) -> io::Result<()> {
    let mut result = Ok(());
    elements.for_each(&mut |styles| {
        if result.is_err() {
            return;
        }

        for style in styles {
            let res = match style {
                MarkupElement::Emphasis => {
                    write!(fmt, "<em>")
                }
                MarkupElement::Dim => {
                    write!(fmt, "<span style=\"opacity: 0.8;\">")
                }
                MarkupElement::Italic => {
                    write!(fmt, "<i>")
                }
                MarkupElement::Underline => {
                    write!(fmt, "<u>")
                }
                MarkupElement::Error => {
                    write!(fmt, "<span style=\"color: Tomato;\">")
                }
                MarkupElement::Success => {
                    write!(fmt, "<span style=\"color: MediumSeaGreen;\">")
                }
                MarkupElement::Warn => {
                    write!(fmt, "<span style=\"color: Orange;\">")
                }
                MarkupElement::Info => {
                    write!(fmt, "<span style=\"color: rgb(38, 148, 255);\">")
                }
            };

            if res.is_err() {
                result = res;
                return;
            }
        }
    });

    result
}

fn pop_styles<W: io::Write>(fmt: &mut W, elements: &MarkupElements) -> io::Result<()> {
    let mut result = Ok(());
    elements.for_each(&mut |styles| {
        if result.is_err() {
            return;
        }

        for style in styles {
            let res = match style {
                MarkupElement::Emphasis => {
                    write!(fmt, "</em>")
                }
                MarkupElement::Italic => {
                    write!(fmt, "</i>")
                }
                MarkupElement::Underline => {
                    write!(fmt, "</u>")
                }
                MarkupElement::Dim
                | MarkupElement::Error
                | MarkupElement::Success
                | MarkupElement::Warn
                | MarkupElement::Info => {
                    write!(fmt, "</span>")
                }
            };

            if res.is_err() {
                result = res;
                return;
            }
        }
    });

    result
}
