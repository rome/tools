use crate::prelude::*;
use rome_formatter::{write, Buffer};

use rome_formatter::{FormatResult, GroupId};
use rome_js_syntax::{JsLanguage, JsSyntaxToken};

use crate::{AsFormat, IntoFormat};
use rome_rowan::{AstNode, AstSeparatedList, AstSeparatedListElementsIterator, SyntaxResult};
use std::iter::FusedIterator;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum TrailingSeparator {
    Allowed,
    Disallowed,
    Mandatory,
}

impl TrailingSeparator {
    pub fn is_allowed(&self) -> bool {
        matches!(self, TrailingSeparator::Allowed)
    }
    pub fn is_mandatory(&self) -> bool {
        matches!(self, TrailingSeparator::Mandatory)
    }
}

impl Default for TrailingSeparator {
    fn default() -> Self {
        TrailingSeparator::Allowed
    }
}

#[derive(Debug, Default, Copy, Clone)]
pub struct FormatSeparatedOptions {
    trailing_separator: TrailingSeparator,
    group_id: Option<GroupId>,
}

impl FormatSeparatedOptions {
    pub fn with_trailing_separator(mut self, separator: TrailingSeparator) -> Self {
        self.trailing_separator = separator;
        self
    }

    pub fn with_group_id(mut self, group_id: Option<GroupId>) -> Self {
        self.group_id = group_id;
        self
    }
}

pub struct FormatSeparatedItem<F, S> {
    node: F,
    separator: S,
    trailing_separator_token: SyntaxResult<Option<JsSyntaxToken>>,
    last: bool,
    options: FormatSeparatedOptions,
}

impl<F, S> Format<JsFormatContext> for FormatSeparatedItem<F, S>
where
    F: Format<JsFormatContext>,
    S: Format<JsFormatContext>,
{
    fn format(&self, f: &mut JsFormatter) -> FormatResult<()> {
        let trailing_separator_factory = format_with(|f| {
            if let Some(group_id) = self.options.group_id {
                write!(f, [if_group_with_id_breaks(&self.separator, group_id)])
            } else {
                write!(f, [if_group_breaks(&self.separator)])
            }
        });

        write!(f, [group_elements(&self.node)])?;

        // Reuse the existing trailing separator or create it if it wasn't in the
        // input source. Only print the last trailing token if the outer group breaks
        if let Some(separator) = self.trailing_separator_token.as_ref()? {
            if self.last {
                if self.options.trailing_separator.is_allowed() {
                    // Use format_replaced instead of wrapping the result of format_token
                    // in order to remove only the token itself when the group doesn't break
                    // but still print its associated trivias unconditionally
                    write!(f, [format_replaced(separator, &trailing_separator_factory)])?;
                } else if self.options.trailing_separator.is_mandatory() {
                    write!(f, [separator.format()])?;
                }
            } else {
                write!(f, [separator.format()])?;
            }
        } else if self.last {
            if self.options.trailing_separator.is_allowed() {
                write!(f, [trailing_separator_factory])?;
            } else if self.options.trailing_separator.is_mandatory() {
                write!(f, [&self.separator])?;
            }
        } else {
            write!(f, [&self.separator])?;
        };

        Ok(())
    }
}

pub struct FormatSeparatedIter<I, Content, Separator>
where
    I: Iterator<Item = (Content, SyntaxResult<Option<JsSyntaxToken>>)>,
{
    inner: std::iter::Peekable<I>,
    separator: Separator,
    options: FormatSeparatedOptions,
}

impl<I, Content, Separator> FormatSeparatedIter<I, Content, Separator>
where
    I: Iterator<Item = (Content, SyntaxResult<Option<JsSyntaxToken>>)>,
    Content: Format<JsFormatContext>,
    Separator: Format<JsFormatContext> + Copy,
{
    pub fn new(inner: I, separator: Separator) -> Self {
        Self::with_options(inner, separator, FormatSeparatedOptions::default())
    }

    pub fn with_options(inner: I, separator: Separator, options: FormatSeparatedOptions) -> Self {
        Self {
            inner: inner.peekable(),
            separator,
            options,
        }
    }
}

impl<I, Content, Separator> Iterator for FormatSeparatedIter<I, Content, Separator>
where
    I: Iterator<Item = (Content, SyntaxResult<Option<JsSyntaxToken>>)>,
    Content: Format<JsFormatContext>,
    Separator: Format<JsFormatContext> + Copy,
{
    type Item = FormatSeparatedItem<Content, Separator>;

    fn next(&mut self) -> Option<Self::Item> {
        let (content, separator) = self.inner.next()?;

        Some(FormatSeparatedItem {
            node: content,
            separator: self.separator,
            trailing_separator_token: separator,
            last: self.inner.peek().is_none(),
            options: self.options,
        })
    }
}

impl<I, Content, Separator> FusedIterator for FormatSeparatedIter<I, Content, Separator>
where
    I: Iterator<Item = (Content, SyntaxResult<Option<JsSyntaxToken>>)>,
    Content: Format<JsFormatContext>,
    Separator: Format<JsFormatContext> + Copy,
{
}

impl<I, Content, Separator> std::iter::ExactSizeIterator
    for FormatSeparatedIter<I, Content, Separator>
where
    I: Iterator<Item = (Content, SyntaxResult<Option<JsSyntaxToken>>)>,
    Content: Format<JsFormatContext>,
    Separator: Format<JsFormatContext> + Copy,
{
}

pub struct FormatSeparatedListItemIter<N> {
    inner: AstSeparatedListElementsIterator<JsLanguage, N>,
}

impl<N> Iterator for FormatSeparatedListItemIter<N>
where
    N: AstNode<Language = JsLanguage> + IntoFormat<JsFormatContext> + Clone,
{
    type Item = (SyntaxResult<N::Format>, SyntaxResult<Option<JsSyntaxToken>>);

    fn next(&mut self) -> Option<Self::Item> {
        let next = self.inner.next()?;

        let separator = match next.trailing_separator() {
            Ok(sep) => Ok(sep.cloned()),
            Err(err) => Err(err),
        };

        Some((next.node().cloned().into_format(), separator))
    }
}

pub trait FormatSeparatedExtension: AstSeparatedList<Language = JsLanguage>
where
    Self::Node: IntoFormat<JsFormatContext> + Clone,
{
    /// Prints a separated list of nodes
    ///
    /// Trailing separators will be reused from the original list or
    /// created by calling the `separator_factory` function.
    /// The last trailing separator in the list will only be printed
    /// if the outer group breaks.
    ///
    fn format_separated<Separator>(
        &self,
        separator: Separator,
    ) -> FormatSeparatedIter<
        FormatSeparatedListItemIter<Self::Node>,
        SyntaxResult<<Self::Node as IntoFormat<JsFormatContext>>::Format>,
        Separator,
    >
    where
        Separator: Format<JsFormatContext> + Copy;

    /// Prints a separated list of nodes
    ///
    /// Trailing separators will be reused from the original list or
    /// created by calling the `separator_factory` function.
    /// The last trailing separator in the list will only be printed
    /// if the outer group breaks.
    fn format_separated_with_options<Separator>(
        &self,
        separator_factory: Separator,
        options: FormatSeparatedOptions,
    ) -> FormatSeparatedIter<
        FormatSeparatedListItemIter<Self::Node>,
        SyntaxResult<<Self::Node as IntoFormat<JsFormatContext>>::Format>,
        Separator,
    >
    where
        Separator: Format<JsFormatContext> + Copy;
}

impl<T> FormatSeparatedExtension for T
where
    T: AstSeparatedList<Language = JsLanguage>,
    T::Node: IntoFormat<JsFormatContext> + Clone,
{
    fn format_separated<Separator>(
        &self,
        separator: Separator,
    ) -> FormatSeparatedIter<
        FormatSeparatedListItemIter<T::Node>,
        SyntaxResult<<T::Node as IntoFormat<JsFormatContext>>::Format>,
        Separator,
    >
    where
        Separator: Format<JsFormatContext> + Copy,
    {
        let inner = FormatSeparatedListItemIter {
            inner: self.elements(),
        };

        FormatSeparatedIter::new(inner, separator)
    }

    fn format_separated_with_options<Separator>(
        &self,
        separator: Separator,
        options: FormatSeparatedOptions,
    ) -> FormatSeparatedIter<
        FormatSeparatedListItemIter<T::Node>,
        SyntaxResult<<T::Node as IntoFormat<JsFormatContext>>::Format>,
        Separator,
    >
    where
        Separator: Format<JsFormatContext> + Copy,
    {
        let inner = FormatSeparatedListItemIter {
            inner: self.elements(),
        };

        FormatSeparatedIter::with_options(inner, separator, options)
    }
}
