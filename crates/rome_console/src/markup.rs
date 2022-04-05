use std::{
    fmt::{self, Debug},
    io,
};

use termcolor::{Color, ColorSpec};

use crate::fmt::{Display, Formatter, MarkupElements, Write};

/// Enumeration of all the supported markup elements
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum MarkupElement {
    Emphasis,
    Dim,
    Italic,
    Underline,
    Error,
    Success,
    Warn,
    Info,
}

impl MarkupElement {
    /// Mutate a [ColorSpec] object in place to apply this element's associated
    /// style to it
    pub(crate) fn update_color(&self, color: &mut ColorSpec) {
        match self {
            // Text Styles
            MarkupElement::Emphasis => {
                color.set_bold(true);
            }
            MarkupElement::Dim => {
                color.set_dimmed(true);
            }
            MarkupElement::Italic => {
                color.set_italic(true);
            }
            MarkupElement::Underline => {
                color.set_underline(true);
            }

            // Text Colors
            MarkupElement::Error => {
                color.set_fg(Some(Color::Red));
            }
            MarkupElement::Success => {
                color.set_fg(Some(Color::Green));
            }
            MarkupElement::Warn => {
                color.set_fg(Some(Color::Yellow));
            }
            MarkupElement::Info => {
                // Blue is really difficult to see on the standard windows command line
                #[cfg(windows)]
                const BLUE: Color = Color::Cyan;
                #[cfg(not(windows))]
                const BLUE: Color = Color::Blue;

                color.set_fg(Some(BLUE));
            }
        }
    }
}

/// Implementation of a single "markup node": a piece of text with a number of
/// associated styles applied to it
#[derive(Copy, Clone)]
pub struct MarkupNode<'fmt> {
    pub elements: &'fmt [MarkupElement],
    pub content: &'fmt dyn Display,
}

#[derive(Clone, PartialEq, Eq)]
pub struct MarkupNodeBuf {
    pub elements: Vec<MarkupElement>,
    pub content: String,
}

impl Debug for MarkupNodeBuf {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        for element in &self.elements {
            write!(fmt, "<{element:?}>")?;
        }
        write!(fmt, "{:?}", self.content)?;
        for element in self.elements.iter().rev() {
            write!(fmt, "</{element:?}>")?;
        }
        if fmt.alternate() && self.content.contains('\n') {
            writeln!(fmt)?;
        }
        Ok(())
    }
}

/// Root type returned by the `markup` macro: this is simply a container for a
/// list of markup nodes
///
/// Text nodes are formatted lazily by storing an [fmt::Arguments] struct, this
/// means [Markup] shares the same restriction as the values returned by
/// [format_args] and can't be stored in a `let` binding for instance
#[derive(Copy, Clone)]
pub struct Markup<'fmt>(pub &'fmt [MarkupNode<'fmt>]);

impl<'fmt> Markup<'fmt> {
    pub fn to_owned(&self) -> MarkupBuf {
        let mut result = MarkupBuf(Vec::new());
        // SAFETY: The implementation of Write for MarkupBuf bellow always returns Ok
        Formatter::new(&mut result).write_markup(*self).unwrap();
        result
    }
}

#[derive(Clone, Default, PartialEq, Eq)]
pub struct MarkupBuf(pub Vec<MarkupNodeBuf>);

impl MarkupBuf {
    pub(crate) fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
}

impl Write for MarkupBuf {
    fn write_str(&mut self, elements: &MarkupElements, content: &str) -> io::Result<()> {
        let mut styles = Vec::new();
        elements.for_each(&mut |elements| {
            styles.extend_from_slice(elements);
        });

        if let Some(last) = self.0.last_mut() {
            if last.elements == styles {
                last.content.push_str(content);
                return Ok(());
            }
        }

        self.0.push(MarkupNodeBuf {
            elements: styles,
            content: content.into(),
        });

        Ok(())
    }

    fn write_fmt(&mut self, elements: &MarkupElements, content: fmt::Arguments) -> io::Result<()> {
        let mut styles = Vec::new();
        elements.for_each(&mut |elements| {
            styles.extend_from_slice(elements);
        });

        if let Some(last) = self.0.last_mut() {
            if last.elements == styles {
                last.content.push_str(&content.to_string());
                return Ok(());
            }
        }

        self.0.push(MarkupNodeBuf {
            elements: styles,
            content: content.to_string(),
        });
        Ok(())
    }
}

impl Display for MarkupBuf {
    fn fmt(&self, fmt: &mut Formatter) -> io::Result<()> {
        let nodes: Vec<_> = self
            .0
            .iter()
            .map(|node| MarkupNode {
                elements: &node.elements,
                content: &node.content,
            })
            .collect();

        fmt.write_markup(Markup(&nodes))
    }
}

impl Debug for MarkupBuf {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        for node in &self.0 {
            write!(fmt, "{node:?}")?;
        }
        Ok(())
    }
}
