use crate::buffer::BufferSnapshotId;
use crate::builders::{FillBuilder, JoinBuilder};
use crate::group_id::UniqueGroupIdBuilder;
use crate::prelude::*;
#[cfg(debug_assertions)]
use crate::printed_tokens::PrintedTokens;
use crate::{Arguments, Buffer, GroupId};
use drop_bomb::DebugDropBomb;
use rome_rowan::{Language, SyntaxNode, SyntaxToken};
use std::fmt;

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
        self.state().context()
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
        joiner: &'joiner dyn Format<Context>,
    ) -> JoinBuilder<'a, 'joiner, 'buf, Context> {
        JoinBuilder::with(self, joiner)
    }

    /// Specialized version of [join_with] for joining SyntaxNodes separated by a space, soft
    /// line break or empty line depending on the input file.
    ///
    /// This functions inspects the input source and separates consecutive elements with either
    /// a [soft_line_break_or_space] or [empty_line] depending on how many line breaks were
    /// separating the elements in the original file.
    pub fn join_nodes_with_soft_line<'a>(
        &'a mut self,
    ) -> JoinNodesBuilder<'a, 'buf, Line, Context> {
        JoinNodesBuilder::new(soft_line_break_or_space(), self)
    }

    /// Specialized version of [join_elements] for joining SyntaxNodes separated by one or more
    /// line breaks depending on the input file.
    ///
    /// This functions inspects the input source and separates consecutive elements with either
    /// a [hard_line_break] or [empty_line] depending on how many line breaks were separating the
    /// elements in the original file.
    pub fn join_nodes_with_hardline<'a>(&'a mut self) -> JoinNodesBuilder<'a, 'buf, Line, Context> {
        JoinNodesBuilder::new(hard_line_break(), self)
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
        separator: &'with dyn Format<Context>,
    ) -> FillBuilder<'a, 'with, 'buf, Context> {
        FillBuilder::new(self, separator)
    }
}

impl<Options> Formatter<'_, Options> {
    /// Take a snapshot of the state of the formatter
    #[inline]

    pub fn snapshot(&mut self) -> FormatterSnapshot {
        FormatterSnapshot {
            buffer: self.buffer.snapshot(),
            bomb: DebugDropBomb::new("Snapshot must either be 'released' or restored'."),
            #[cfg(debug_assertions)]
            printed_tokens: self.state().printed_tokens.clone(),
        }
    }

    #[inline]
    /// Restore the state of the formatter to a previous snapshot
    pub fn restore_snapshot(&mut self, mut snapshot: FormatterSnapshot) {
        cfg_if::cfg_if! {
            if #[cfg(debug_assertions)] {
                self.state_mut().printed_tokens = snapshot.printed_tokens;
            }
        }
        snapshot.bomb.defuse();
        self.buffer.restore_snapshot(snapshot.buffer)
    }

    pub fn release_snapshot(&mut self, mut snapshot: FormatterSnapshot) {
        snapshot.bomb.defuse();
        self.buffer.release_snapshot(snapshot.buffer)
    }
}

impl<O> Buffer for Formatter<'_, O> {
    type Context = O;

    fn write_element(&mut self, element: FormatElement) {
        self.buffer.write_element(element)
    }

    fn write_fmt(&mut self, arguments: &Arguments<Self::Context>) -> FormatResult<()> {
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

    fn snapshot(&mut self) -> BufferSnapshotId {
        self.buffer.snapshot()
    }

    fn restore_snapshot(&mut self, snapshot: BufferSnapshotId) {
        self.buffer.restore_snapshot(snapshot)
    }

    fn release_snapshot(&mut self, snapshot: BufferSnapshotId) {
        self.buffer.release_snapshot(snapshot)
    }
}

#[derive(Default)]
pub struct FormatState<O> {
    options: O,
    group_id_builder: UniqueGroupIdBuilder,
    // This is using a RefCell as it only exists in debug mode,
    // the Formatter is still completely immutable in release builds
    #[cfg(debug_assertions)]
    pub printed_tokens: PrintedTokens,
}

impl<O> fmt::Debug for FormatState<O>
where
    O: fmt::Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("FormatContext")
            .field("options", &self.options)
            .finish()
    }
}

impl<O> FormatState<O> {
    pub fn new(options: O) -> Self {
        Self {
            options,
            group_id_builder: Default::default(),
            #[cfg(debug_assertions)]
            printed_tokens: Default::default(),
        }
    }

    /// Returns the [FormatOptions] specifying how to format the current CST
    pub fn context(&self) -> &O {
        &self.options
    }

    /// Creates a new group id that is unique to this document. The passed debug name is used in the
    /// [std::fmt::Debug] of the document if this is a debug build.
    /// The name is unused for production builds and has no meaning on the equality of two group ids.
    pub fn group_id(&self, debug_name: &'static str) -> GroupId {
        self.group_id_builder.group_id(debug_name)
    }

    /// Tracks the given token as formatted
    #[inline]
    pub fn track_token<L: Language>(&mut self, #[allow(unused_variables)] token: &SyntaxToken<L>) {
        cfg_if::cfg_if! {
            if #[cfg(debug_assertions)] {
                self.printed_tokens.track_token(token);
            }
        }
    }

    #[inline]
    pub fn assert_formatted_all_tokens<L: Language>(
        &self,
        #[allow(unused_variables)] root: &SyntaxNode<L>,
    ) {
        cfg_if::cfg_if! {
            if #[cfg(debug_assertions)] {
                self.printed_tokens.assert_all_tracked(root);
            }
        }
    }
}

/// Snapshot of the formatter state  used to handle backtracking if
/// errors are encountered in the formatting process and the formatter
/// has to fallback to printing raw tokens
///
/// In practice this only saves the set of printed tokens in debug
/// mode and compiled to nothing in release mode
#[must_use = "Snapshot must either be 'released' or 'restored'."]
pub struct FormatterSnapshot {
    buffer: BufferSnapshotId,
    bomb: DebugDropBomb,
    #[cfg(debug_assertions)]
    printed_tokens: PrintedTokens,
}
