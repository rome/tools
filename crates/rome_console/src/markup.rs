use core::fmt;
use std::io;

use rslint_errors::termcolor::{Color, ColorSpec, WriteColor};

/// Manages the state of an object implementing [WriteColor], tracking the
/// successive styles being applied to the printer, and restoring the previous
/// style when the current one goes out of scope
pub(crate) struct MarkupPrinter<W: WriteColor> {
    inner: W,
    colors: Vec<ColorSpec>,
}

impl<W: WriteColor> MarkupPrinter<W> {
    pub(crate) fn new(writer: W) -> Self {
        Self {
            inner: writer,
            colors: Vec::new(),
        }
    }

    /// Push a new color state to the stack, created from mutating the previous state
    fn push_color(&mut self, func: impl FnOnce(&mut ColorSpec)) -> io::Result<()> {
        // Clone the previous state, or create a default ColorSpec if the stack is empty
        let mut color = match self.colors.last() {
            Some(color) => color.clone(),
            None => ColorSpec::new(),
        };

        func(&mut color);
        self.inner.set_color(&color)?;
        self.colors.push(color);
        Ok(())
    }

    /// Pop the most recent color state from the stack, restoring the previous state
    fn pop_color(&mut self) -> io::Result<()> {
        self.colors.pop();
        match self.colors.last() {
            Some(color) => self.inner.set_color(color),
            None => self.inner.reset(),
        }
    }
}

// MarkupPrinter implements Drop to ensure the color style of the inner printer
// is properly reset after a printing operation completes
impl<W: WriteColor> Drop for MarkupPrinter<W> {
    fn drop(&mut self) {
        // Any error happing here is ignored since there isn't a good way to
        // propagate them from `drop`, and panicking here isn't great either
        self.inner.reset().ok();
    }
}

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

/// Implementation of a single "markup node": can be either a piece of text, or
/// an element containing zero or more other nodes
///
/// Text nodes are formatted lazily by storing an [fmt::Arguments] struct, this
/// means [MarkupNode] shares the same restriction as the values returned by
/// [format_args] and can't be stored in a `let` binding for instance
#[derive(Copy, Clone, Debug)]
pub enum MarkupNode<'fmt> {
    Text(fmt::Arguments<'fmt>),
    Element {
        kind: MarkupElement,
        children: &'fmt [MarkupNode<'fmt>],
    },
}

impl<'fmt> From<fmt::Arguments<'fmt>> for MarkupNode<'fmt> {
    fn from(args: fmt::Arguments<'fmt>) -> Self {
        Self::Text(args)
    }
}

impl<'fmt> MarkupNode<'fmt> {
    /// Print a [MarkupNode] to the provided [MarkupPrinter]
    pub(crate) fn print(&self, fmt: &mut MarkupPrinter<impl WriteColor>) -> io::Result<()> {
        match self {
            // If the node just contains text, print it out directly
            MarkupNode::Text(text) => write!(fmt.inner, "{}", text),

            // If the node is a MarkupElement, apply the associated style before printing the children
            MarkupNode::Element { kind, children } => {
                match kind {
                    // Text Styles
                    MarkupElement::Emphasis => {
                        fmt.push_color(|color| {
                            color.set_bold(true);
                        })?;
                    }
                    MarkupElement::Dim => {
                        fmt.push_color(|color| {
                            color.set_dimmed(true);
                        })?;
                    }
                    MarkupElement::Italic => {
                        fmt.push_color(|color| {
                            color.set_italic(true);
                        })?;
                    }
                    MarkupElement::Underline => {
                        fmt.push_color(|color| {
                            color.set_underline(true);
                        })?;
                    }

                    // Text Colors
                    MarkupElement::Error => {
                        fmt.push_color(|color| {
                            color.set_fg(Some(Color::Red));
                        })?;
                    }
                    MarkupElement::Success => {
                        fmt.push_color(|color| {
                            color.set_fg(Some(Color::Green));
                        })?;
                    }
                    MarkupElement::Warn => {
                        fmt.push_color(|color| {
                            color.set_fg(Some(Color::Yellow));
                        })?;
                    }
                    MarkupElement::Info => {
                        fmt.push_color(|color| {
                            color.set_fg(Some(Color::Blue));
                        })?;
                    }
                }

                // If a child node returns an error while printing, we want to
                // abort immediately and return the error (like if we had
                // written `for child in children { child.print(fmt); }`) but
                // since the console is stateful we need to clean up the style
                // that was applied above before returning.
                // This code iterates over the children, aborting if one of
                // them returns an error but the error is only stored in
                // `result` to be returned later, so the cleanup code can still
                // run in-between
                let mut iter = children.iter();
                let result = loop {
                    match iter.next() {
                        Some(child) => match child.print(fmt) {
                            Ok(()) => continue,
                            Err(err) => break Err(err),
                        },
                        None => break Ok(()),
                    }
                };

                match kind {
                    MarkupElement::Emphasis
                    | MarkupElement::Dim
                    | MarkupElement::Italic
                    | MarkupElement::Underline
                    | MarkupElement::Error
                    | MarkupElement::Success
                    | MarkupElement::Warn
                    | MarkupElement::Info => {
                        fmt.pop_color()?;
                    }
                }

                result
            }
        }
    }
}
