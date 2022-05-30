use crate::builders::{FillBuilder, JoinBuilder};
use crate::prelude::*;
#[cfg(debug_assertions)]
use crate::printed_tokens::PrintedTokens;
use crate::{Arguments, Buffer, FormatContext, FormatState, GroupId};
use rome_rowan::{Language, SyntaxNode, SyntaxToken};

/// Handles the formatting of a CST and stores the options how the CST should be formatted (user preferences).
/// The formatter is passed to the [Format] implementation of every node in the CST so that they
/// can use it to format their children.
pub struct Formatter<'buf, Options> {
    buffer: &'buf mut dyn Buffer<Context = Options>,
}

impl<'buf, Context> Formatter<'buf, Context> {
    /// Creates a new context that uses the given formatter options
    pub fn new(buffer: &'buf mut (dyn Buffer<Context = Context> + 'buf)) -> Self {
        Self { buffer }
    }

    /// Returns the [FormatOptions] specifying how to format the current CST
    pub fn context(&self) -> &Context {
        &self.state().context()
    }

    /// Creates a new group id that is unique to this document. The passed debug name is used in the
    /// [std::fmt::Debug] of the document if this is a debug build.
    /// The name is unused for production builds and has no meaning on the equality of two group ids.
    pub fn group_id(&self, debug_name: &'static str) -> GroupId {
        self.state().group_id(debug_name)
    }

    /// Concatenates multiple [Format].
    ///
    /// ## Examples
    ///
    /// ```rust
    /// use rome_formatter::format;
    /// use rome_formatter::prelude::*;
    ///
    /// let formatted = format!(SimpleFormatContext::default(), [format_with(|f| {
    ///     f.join()
    ///         .entry(&token("a"))
    ///         .entry(&space_token())
    ///         .entry(&token("+"))
    ///         .entry(&space_token())
    ///         .entry(&token("b"))
    ///         .finish()
    /// })]).unwrap();
    ///
    /// assert_eq!(
    ///     "a + b",
    ///     formatted.print().as_code()
    /// )
    /// ```
    pub fn join<'a>(&'a mut self) -> JoinBuilder<'a, '_, 'buf, Context> {
        JoinBuilder::new(self)
    }

    /// Joins the elements by placing a given separator between elements.
    ///
    /// ## Examples
    ///
    /// Joining different tokens by separating them with a comma and a space.
    ///
    /// ```
    /// use rome_formatter::{format, format_args};
    /// use rome_formatter::prelude::*;
    ///
    /// let formatted = format!(SimpleFormatContext::default(), [format_with(|f| {
    ///     f.join_with(&format_args!(token(","), space_token()))
    ///         .entry(&token("1"))
    ///         .entry(&token("2"))
    ///         .entry(&token("3"))
    ///         .entry(&token("4"))
    ///         .finish()
    /// })]).unwrap();
    ///
    /// assert_eq!(
    ///     "1, 2, 3, 4",
    ///     formatted.print().as_code()
    /// );
    /// ```
    pub fn join_with<'a, 'joiner>(
        &'a mut self,
        joiner: &'joiner dyn Format<Context = Context>,
    ) -> JoinBuilder<'a, 'joiner, 'buf, Context> {
        JoinBuilder::with(self, joiner)
    }

    /// Concatenates a list of [FormatElement]s with spaces and line breaks to fit
    /// them on as few lines as possible. Each element introduces a conceptual group. The printer
    /// first tries to print the item in flat mode but then prints it in expanded mode if it doesn't fit.
    ///
    /// ## Examples
    ///
    /// ```rust
    /// use rome_formatter::prelude::*;
    /// use rome_formatter::{format, format_args};
    ///
    /// let formatted = format!(SimpleFormatContext::default(), [format_with(|f| {
    ///     f.fill(&soft_line_break_or_space())
    ///         .entry(&token("aaaaaaaaaaaaaaaaaaaaaaaaaaaaaa"))
    ///         .entry(&token("bbbbbbbbbbbbbbbbbbbbbbbbbbbbbb"))
    ///         .entry(&token("cccccccccccccccccccccccccccccc"))
    ///         .entry(&token("dddddddddddddddddddddddddddddd"))
    ///         .finish()
    /// })]).unwrap();
    ///
    /// assert_eq!(
    ///     "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaa bbbbbbbbbbbbbbbbbbbbbbbbbbbbbb\ncccccccccccccccccccccccccccccc dddddddddddddddddddddddddddddd",
    ///     formatted.print().as_code()
    /// )
    /// ```
    pub fn fill<'a, 'with>(
        &'a mut self,
        separator: &'with dyn Format<Context = Context>,
    ) -> FillBuilder<'a, 'with, 'buf, Context> {
        FillBuilder::new(self, separator)
    }
}

impl<Options> Formatter<'_, Options> {
    // TODO

    // /// Take a snapshot of the state of the formatter
    // #[inline]
    // pub fn snapshot(&self) -> FormatterSnapshot {
    //     FormatterSnapshot {
    //         #[cfg(debug_assertions)]
    //         printed_tokens: self.printed_tokens.borrow().clone(),
    //     }
    // }
    //
    // #[inline]
    // /// Restore the state of the formatter to a previous snapshot
    // pub fn restore(&self, #[allow(unused)] snapshot: FormatterSnapshot) {
    //     cfg_if::cfg_if! {
    //         if #[cfg(debug_assertions)] {
    //             *self.printed_tokens.borrow_mut() = snapshot.printed_tokens;
    //         }
    //     }
    // }
}

impl<O> Buffer for Formatter<'_, O> {
    type Context = O;

    fn write_element(&mut self, element: FormatElement) -> FormatResult<()> {
        self.buffer.write_element(element)
    }

    fn write_fmt(self: &mut Self, arguments: &Arguments<Self::Context>) -> FormatResult<()> {
        for argument in arguments.items() {
            argument.format(self)?;
        }
        Ok(())
    }

    fn state(&self) -> &FormatState<Self::Context> {
        self.buffer.state()
    }

    fn state_mut(&mut self) -> &mut FormatState<Self::Context> {
        self.buffer.state_mut()
    }
}

/// Snapshot of the formatter state  used to handle backtracking if
/// errors are encountered in the formatting process and the formatter
/// has to fallback to printing raw tokens
///
/// In practice this only saves the set of printed tokens in debug
/// mode and compiled to nothing in release mode
pub struct FormatterSnapshot {
    #[cfg(debug_assertions)]
    printed_tokens: PrintedTokens,
}
