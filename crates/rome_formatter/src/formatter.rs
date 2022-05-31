use crate::group_id::UniqueGroupIdBuilder;
use crate::prelude::*;
#[cfg(debug_assertions)]
use crate::printed_tokens::PrintedTokens;
use crate::GroupId;
use rome_rowan::{Language, SyntaxNode, SyntaxToken};
#[cfg(debug_assertions)]
use std::cell::RefCell;

/// Handles the formatting of a CST and stores the options how the CST should be formatted (user preferences).
/// The formatter is passed to the [Format] implementation of every node in the CST so that they
/// can use it to format their children.
#[derive(Default)]
pub struct Formatter<Context> {
    /// Yields various information that belong to the current instance of the formatter
    context: Context,
    group_id_builder: UniqueGroupIdBuilder,
    // This is using a RefCell as it only exists in debug mode,
    // the Formatter is still completely immutable in release builds
    #[cfg(debug_assertions)]
    pub printed_tokens: RefCell<PrintedTokens>,
}

impl<Context> Formatter<Context> {
    /// Creates a new context that uses the given formatter options
    pub fn new(options: Context) -> Self {
        Self {
            context: options,
            group_id_builder: Default::default(),
            #[cfg(debug_assertions)]
            printed_tokens: Default::default(),
        }
    }

    /// Returns the [FormatOptions] specifying how to format the current CST
    pub fn context(&self) -> &Context {
        &self.context
    }

    /// Creates a new group id that is unique to this document. The passed debug name is used in the
    /// [std::fmt::Debug] of the document if this is a debug build.
    /// The name is unused for production builds and has no meaning on the equality of two group ids.
    pub fn group_id(&self, debug_name: &'static str) -> GroupId {
        self.group_id_builder.group_id(debug_name)
    }

    /// Tracks the given token as formatted
    #[inline]
    pub fn track_token<L: Language>(&self, #[allow(unused_variables)] token: &SyntaxToken<L>) {
        cfg_if::cfg_if! {
            if #[cfg(debug_assertions)] {
                self.printed_tokens.borrow_mut().track_token(token);
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
                let printed_tokens = self.printed_tokens.borrow();
                printed_tokens.assert_all_tracked(root);
            }
        }
    }

    /// Formats all items of the iterator and returns the formatted result
    ///
    /// Returns the [Err] of the first item that failed to format.
    #[inline]
    pub fn format_all<T: Format<Context = Context>>(
        &self,
        nodes: impl IntoIterator<Item = T>,
    ) -> FormatResult<impl Iterator<Item = FormatElement>> {
        let mut result = Vec::new();

        for node in nodes {
            match node.format(self) {
                Ok(formatted) => {
                    result.push(formatted);
                }
                Err(err) => return Err(err),
            }
        }

        Ok(result.into_iter())
    }
}

impl<Context> Formatter<Context> {
    /// Take a snapshot of the state of the formatter
    #[inline]
    pub fn snapshot(&self) -> FormatterSnapshot {
        FormatterSnapshot {
            #[cfg(debug_assertions)]
            printed_tokens: self.printed_tokens.borrow().clone(),
        }
    }

    #[inline]
    /// Restore the state of the formatter to a previous snapshot
    pub fn restore(&self, #[allow(unused)] snapshot: FormatterSnapshot) {
        cfg_if::cfg_if! {
            if #[cfg(debug_assertions)] {
                *self.printed_tokens.borrow_mut() = snapshot.printed_tokens;
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
pub struct FormatterSnapshot {
    #[cfg(debug_assertions)]
    printed_tokens: PrintedTokens,
}
