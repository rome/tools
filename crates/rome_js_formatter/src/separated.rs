use crate::prelude::*;
use crate::AsFormat;
use rome_formatter::{write, GroupId};
use rome_js_syntax::JsLanguage;
use rome_rowan::{
    AstNode, AstSeparatedElement, AstSeparatedList, AstSeparatedListElementsIterator, Language,
    SyntaxResult,
};
use std::iter::FusedIterator;

/// Formats a single element inside of a separated list.
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct FormatSeparatedElement<L: Language, N> {
    element: AstSeparatedElement<L, N>,
    is_last: bool,
    /// The separator to write if the element has no separator yet.
    separator: &'static str,
    options: FormatSeparatedOptions,
}

impl<L: Language, N: AstNode<Language = L>> FormatSeparatedElement<L, N> {
    /// Returns the node belonging to the element.
    pub fn node(&self) -> SyntaxResult<&N> {
        self.element.node()
    }
}

impl<N> Format<JsFormatContext> for FormatSeparatedElement<JsLanguage, N>
where
    for<'a> N: AstNode<Language = JsLanguage> + AsFormat<'a>,
{
    fn fmt(&self, f: &mut Formatter<JsFormatContext>) -> FormatResult<()> {
        let node = self.element.node()?;
        let separator = self.element.trailing_separator()?;

        if !self.options.nodes_grouped {
            node.format().fmt(f)?;
        } else {
            group(&node.format()).fmt(f)?;
        }

        // Reuse the existing trailing separator or create it if it wasn't in the
        // input source. Only print the last trailing token if the outer group breaks
        if let Some(separator) = separator {
            if self.is_last {
                match self.options.trailing_separator {
                    TrailingSeparator::Allowed => {
                        // Use format_replaced instead of wrapping the result of format_token
                        // in order to remove only the token itself when the group doesn't break
                        // but still print its associated trivia unconditionally
                        format_only_if_breaks(separator, &separator.format())
                            .with_group_id(self.options.group_id)
                            .fmt(f)?;
                    }
                    TrailingSeparator::Mandatory => {
                        write!(f, [separator.format()])?;
                    }
                    TrailingSeparator::Disallowed => {
                        // A trailing separator was present where it wasn't allowed, opt out of formatting
                        return Err(FormatDiagnostic::SyntaxError);
                    }
                    TrailingSeparator::Omit => {
                        write!(f, [format_removed(separator)])?;
                    }
                }
            } else {
                write!(f, [separator.format()])?;
            }
        } else if self.is_last {
            match self.options.trailing_separator {
                TrailingSeparator::Allowed => {
                    write!(
                        f,
                        [if_group_breaks(&text(self.separator))
                            .with_group_id(self.options.group_id)]
                    )?;
                }
                TrailingSeparator::Mandatory => {
                    text(self.separator).fmt(f)?;
                }
                TrailingSeparator::Omit | TrailingSeparator::Disallowed => { /* no op */ }
            }
        } else {
            unreachable!(
                "This is a syntax error, separator must be present between every two elements"
            );
        };

        Ok(())
    }
}

/// Iterator for formatting separated elements. Prints the separator between each element and
/// inserts a trailing separator if necessary
pub struct FormatSeparatedIter<I, Language, Node>
where
    Language: rome_rowan::Language,
{
    next: Option<AstSeparatedElement<Language, Node>>,
    inner: I,
    separator: &'static str,
    options: FormatSeparatedOptions,
}

impl<I, L, Node> FormatSeparatedIter<I, L, Node>
where
    L: Language,
{
    fn new(inner: I, separator: &'static str) -> Self {
        Self {
            inner,
            separator,
            next: None,
            options: FormatSeparatedOptions::default(),
        }
    }

    /// Wraps every node inside of a group
    pub fn nodes_grouped(mut self) -> Self {
        self.options.nodes_grouped = true;
        self
    }

    pub fn with_trailing_separator(mut self, separator: TrailingSeparator) -> Self {
        self.options.trailing_separator = separator;
        self
    }

    #[allow(unused)]
    pub fn with_group_id(mut self, group_id: Option<GroupId>) -> Self {
        self.options.group_id = group_id;
        self
    }
}

impl<I, N> Iterator for FormatSeparatedIter<I, JsLanguage, N>
where
    I: Iterator<Item = AstSeparatedElement<JsLanguage, N>>,
{
    type Item = FormatSeparatedElement<JsLanguage, N>;

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

impl<I, N> FusedIterator for FormatSeparatedIter<I, JsLanguage, N> where
    I: Iterator<Item = AstSeparatedElement<JsLanguage, N>> + FusedIterator
{
}

impl<I, N> ExactSizeIterator for FormatSeparatedIter<I, JsLanguage, N> where
    I: Iterator<Item = AstSeparatedElement<JsLanguage, N>> + ExactSizeIterator
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
    fn format_separated(
        &self,
        separator: &'static str,
    ) -> FormatSeparatedIter<
        AstSeparatedListElementsIterator<JsLanguage, Self::Node>,
        JsLanguage,
        Self::Node,
    > {
        FormatSeparatedIter::new(self.elements(), separator)
    }
}

impl<T> FormatAstSeparatedListExtension for T where T: AstSeparatedList<Language = JsLanguage> {}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Default)]
pub enum TrailingSeparator {
    /// A trailing separator is allowed and preferred
    #[default]
    Allowed,

    /// A trailing separator is not allowed
    Disallowed,

    /// A trailing separator is mandatory for the syntax to be correct
    Mandatory,

    /// A trailing separator might be present, but the consumer
    /// decides to remove it
    Omit,
}

#[derive(Debug, Default, Copy, Clone, Eq, PartialEq)]
pub struct FormatSeparatedOptions {
    trailing_separator: TrailingSeparator,
    group_id: Option<GroupId>,
    nodes_grouped: bool,
}
