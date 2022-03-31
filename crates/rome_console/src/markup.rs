use termcolor::{Color, ColorSpec};

use crate::fmt::Display;

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

/// Root type returned by the `markup` macro: this is simply a container for a
/// list of markup nodes
///
/// Text nodes are formatted lazily by storing an [fmt::Arguments] struct, this
/// means [Markup] shares the same restriction as the values returned by
/// [format_args] and can't be stored in a `let` binding for instance
pub struct Markup<'fmt>(pub &'fmt [MarkupNode<'fmt>]);
