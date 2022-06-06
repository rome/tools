use crate::prelude::*;
use crate::AsFormat;
use rome_formatter::{write, GroupId};
use rome_js_syntax::JsLanguage;
use rome_rowan::{
    AstNode, AstSeparatedElement, AstSeparatedList, AstSeparatedListElementsIterator, Language,
};
use std::iter::FusedIterator;

/// Formats a single element inside of a separated list.
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct FormatSeparatedElement<L: Language, N, Separator> {
    element: AstSeparatedElement<L, N>,
    is_last: bool,
    /// The separator to write if the element has no separator yet.
    separator: Separator,
    options: FormatSeparatedOptions,
}

impl<N, Separator> Format<JsFormatContext> for FormatSeparatedElement<JsLanguage, N, Separator>
where
    for<'a> N: AstNode<Language = JsLanguage> + AsFormat<'a>,
    Separator: Format<JsFormatContext>,
{
    fn fmt(&self, f: &mut Formatter<JsFormatContext>) -> FormatResult<()> {
        let node = self.element.node()?;
        let separator = self.element.trailing_separator()?;

        write!(f, [group_elements(&node.format())])?;

        let format_trailing_separator =
            if_group_breaks(&self.separator).with_group_id(self.options.group_id);

        // Reuse the existing trailing separator or create it if it wasn't in the
        // input source. Only print the last trailing token if the outer group breaks
        if let Some(separator) = separator {
            if self.is_last {
                match self.options.trailing_separator {
                    TrailingSeparator::Allowed => {
                        // Use format_replaced instead of wrapping the result of format_token
                        // in order to remove only the token itself when the group doesn't break
                        // but still print its associated trivias unconditionally
                        write!(f, [format_replaced(separator, &format_trailing_separator)])?;
                    }
                    TrailingSeparator::Mandatory => {
                        write!(f, [separator.format()])?;
                    }
                    TrailingSeparator::Disallowed => {
                        // A trailing separator was present where it wasn't allowed, opt out of formatting
                        return Err(FormatError::MissingRequiredChild);
                    }
                }
            } else {
                write!(f, [separator.format()])?;
            }
        } else if self.is_last {
            match self.options.trailing_separator {
                TrailingSeparator::Allowed => {
                    write!(f, [format_trailing_separator])?;
                }
                TrailingSeparator::Mandatory => {
                    write!(f, [&self.separator])?;
                }
                TrailingSeparator::Disallowed => { /* no op */ }
            }
        } else {
            write!(f, [&self.separator])?;
        };

        Ok(())
    }
}

/// Iterator for formatting separated elements. Prints the separator between each element and
/// inserts a trailing separator if necessary
pub struct FormatSeparatedIter<I, Language, Node, Separator>
where
    Language: rome_rowan::Language,
{
    next: Option<AstSeparatedElement<Language, Node>>,
    inner: I,
    separator: Separator,
    options: FormatSeparatedOptions,
}

impl<I, L, Node, Separator> FormatSeparatedIter<I, L, Node, Separator>
where
    L: Language,
{
    fn new(inner: I, separator: Separator) -> Self {
        Self {
            inner,
            separator,
            next: None,
            options: FormatSeparatedOptions::default(),
        }
    }

    pub fn with_options(mut self, options: FormatSeparatedOptions) -> Self {
        self.options = options;
        self
    }
}

impl<I, N, Separator> Iterator for FormatSeparatedIter<I, JsLanguage, N, Separator>
where
    I: Iterator<Item = AstSeparatedElement<JsLanguage, N>>,
    Separator: Copy,
{
    type Item = FormatSeparatedElement<JsLanguage, N, Separator>;

    fn next(&mut self) -> Option<Self::Item> {
        let element = self.next.take().or_else(|| self.inner.next())?;

        self.next = self.inner.next();
        let is_last = self.next.is_none();

        Some(FormatSeparatedElement {
            element,
            is_last,
            separator: self.separator,
            options: self.options,
        })
    }
}

impl<I, N, Separator> FusedIterator for FormatSeparatedIter<I, JsLanguage, N, Separator>
where
    I: Iterator<Item = AstSeparatedElement<JsLanguage, N>> + FusedIterator,
    Separator: Copy,
{
}

impl<I, N, Separator> ExactSizeIterator for FormatSeparatedIter<I, JsLanguage, N, Separator>
where
    I: Iterator<Item = AstSeparatedElement<JsLanguage, N>> + ExactSizeIterator,
    Separator: Copy,
{
}

/// AST Separated list formatting extension methods
pub trait FormatAstSeparatedListExtension: AstSeparatedList<Language = JsLanguage> {
    /// Prints a separated list of nodes
    ///
    /// Trailing separators will be reused from the original list or
    /// created by calling the `separator_factory` function.
    /// The last trailing separator in the list will only be printed
    /// if the outer group breaks.
    fn format_separated<Separator>(
        &self,
        separator: Separator,
    ) -> FormatSeparatedIter<
        AstSeparatedListElementsIterator<JsLanguage, Self::Node>,
        JsLanguage,
        Self::Node,
        Separator,
    >
    where
        Separator: Format<JsFormatContext> + Copy,
    {
        FormatSeparatedIter::new(self.elements(), separator)
    }
}

impl<T> FormatAstSeparatedListExtension for T where T: AstSeparatedList<Language = JsLanguage> {}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum TrailingSeparator {
    /// A trailing separator is allowed and preferred
    Allowed,

    /// A trailing separator is not allowed
    Disallowed,

    /// A trailing separator is mandatory for the syntax to be correct
    Mandatory,
}

impl Default for TrailingSeparator {
    fn default() -> Self {
        TrailingSeparator::Allowed
    }
}

#[derive(Debug, Default, Copy, Clone, Eq, PartialEq)]
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
