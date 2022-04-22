//! AST definitions for converting untyped syntax nodes into typed AST nodes.
//!
//! Every field of every AST node is optional, this is to allow the parser to recover
//! from any error and produce an ast from any source code. If you don't want to account for
//! optionals for everything, you can use ...

use std::error::Error;
use std::fmt::{self, Debug, Display, Formatter};
use std::iter::FusedIterator;
use std::marker::PhantomData;
use text_size::TextRange;

use crate::syntax::{SyntaxSlot, SyntaxSlots};
use crate::{Language, SyntaxList, SyntaxNode, SyntaxToken};

/// The main trait to go from untyped `SyntaxNode`  to a typed ast. The
/// conversion itself has zero runtime cost: ast and syntax nodes have exactly
/// the same representation: a pointer to the tree root and a pointer to the
/// node itself.
pub trait AstNode {
    type Language: Language;

    /// Returns `true` if a node with the given kind can be cased to this AST node.
    fn can_cast(kind: <Self::Language as Language>::Kind) -> bool;

    /// Tries to cast the passed syntax node to this AST node.
    ///
    /// # Returns
    ///
    /// [None] if the passed node is of a different kind. [Some] otherwise.
    fn cast(syntax: SyntaxNode<Self::Language>) -> Option<Self>
    where
        Self: Sized;

    /// Returns the underlying syntax node.
    fn syntax(&self) -> &SyntaxNode<Self::Language>;

    /// Cast this node to this AST node
    ///
    /// # Panics
    /// Panics if the underlying node cannot be cast to this AST node
    fn unwrap_cast(syntax: SyntaxNode<Self::Language>) -> Self
    where
        Self: Sized,
    {
        let kind = syntax.kind();
        Self::cast(syntax).unwrap_or_else(|| {
            panic!(
                "Tried to cast node with kind {:?} as `{:?}` but was unable to cast",
                kind,
                std::any::type_name::<Self>()
            )
        })
    }

    /// Returns the string representation of this node without the leading and trailing trivia
    fn text(&self) -> std::string::String {
        self.syntax().text_trimmed().to_string()
    }

    fn range(&self) -> TextRange {
        self.syntax().text_trimmed_range()
    }

    fn clone_subtree(&self) -> Self
    where
        Self: Sized,
    {
        Self::cast(self.syntax().clone_subtree()).unwrap()
    }
}

/// List of homogenous nodes
pub trait AstNodeList {
    type Language: Language;
    type Node: AstNode<Language = Self::Language>;

    /// Returns the underlying syntax list
    fn syntax_list(&self) -> &SyntaxList<Self::Language>;

    fn iter(&self) -> AstNodeListIterator<Self::Language, Self::Node> {
        AstNodeListIterator {
            inner: self.syntax_list().iter(),
            ph: PhantomData,
        }
    }

    #[inline]
    fn len(&self) -> usize {
        self.syntax_list().len()
    }

    /// Returns the first node from this list or None
    #[inline]
    fn first(&self) -> Option<Self::Node> {
        self.iter().next()
    }

    /// Returns the last node from this list or None
    fn last(&self) -> Option<Self::Node> {
        self.iter().last()
    }

    #[inline]
    fn is_empty(&self) -> bool {
        self.syntax_list().is_empty()
    }
}

#[derive(Debug, Clone)]
pub struct AstNodeListIterator<L, N>
where
    L: Language,
{
    inner: SyntaxSlots<L>,
    ph: PhantomData<N>,
}

impl<L: Language, N: AstNode<Language = L>> AstNodeListIterator<L, N> {
    fn slot_to_node(slot: &SyntaxSlot<L>) -> N {
        match slot {
            SyntaxSlot::Empty => panic!("Node isn't permitted to contain empty slots"),
            SyntaxSlot::Node(node) => N::unwrap_cast(node.to_owned()),
            SyntaxSlot::Token(token) => panic!(
                "Expected node of type `{:?}` but found token `{:?}` instead.",
                std::any::type_name::<N>(),
                token
            ),
        }
    }
}

impl<L: Language, N: AstNode<Language = L>> Iterator for AstNodeListIterator<L, N> {
    type Item = N;

    fn next(&mut self) -> Option<Self::Item> {
        Some(Self::slot_to_node(&self.inner.next()?))
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        (self.inner.len(), Some(self.inner.len()))
    }

    fn last(self) -> Option<Self::Item>
    where
        Self: Sized,
    {
        Some(Self::slot_to_node(&self.inner.last()?))
    }

    fn nth(&mut self, n: usize) -> Option<Self::Item> {
        Some(Self::slot_to_node(&self.inner.nth(n)?))
    }
}

impl<L: Language, N: AstNode<Language = L>> ExactSizeIterator for AstNodeListIterator<L, N> {}

impl<L: Language, N: AstNode<Language = L>> FusedIterator for AstNodeListIterator<L, N> {}

#[derive(Clone)]
pub struct AstSeparatedElement<L: Language, N> {
    node: SyntaxResult<N>,
    trailing_separator: SyntaxResult<Option<SyntaxToken<L>>>,
}

impl<L: Language, N: AstNode<Language = L>> AstSeparatedElement<L, N> {
    pub fn node(&self) -> SyntaxResult<&N> {
        match &self.node {
            Ok(node) => Ok(&node),
            Err(err) => Err(*err),
        }
    }

    pub fn trailing_separator(&self) -> SyntaxResult<Option<&SyntaxToken<L>>> {
        match &self.trailing_separator {
            Ok(Some(sep)) => Ok(Some(sep)),
            Ok(_) => Ok(None),
            Err(err) => Err(*err),
        }
    }
}

impl<L: Language, N: Debug> Debug for AstSeparatedElement<L, N> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match &self.node {
            Ok(node) => N::fmt(node, f)?,
            Err(_) => f.write_str("missing element")?,
        };
        match &self.trailing_separator {
            Ok(Some(separator)) => {
                f.write_str(",\n")?;
                Debug::fmt(&separator, f)
            }
            Err(_) => f.write_str(",\nmissing separator"),
            Ok(None) => Ok(()),
        }
    }
}

/// List of nodes where every two nodes are separated by a token.
/// For example, the elements of an array where every two elements are separated by a comma token.
/// The list expects that the underlying syntax node has a slot for every node and separator
/// even if they are missing from the source code. For example, a list for `a b` where the `,` separator
/// is missing contains the slots `Node(a), Empty, Node(b)`. This also applies for missing nodes:
/// the list for `, b,` must have the slots `Empty, Token(,), Node(b), Token(,)`.
pub trait AstSeparatedList {
    type Language: Language;
    type Node: AstNode<Language = Self::Language>;

    /// Returns the underlying syntax list
    fn syntax_list(&self) -> &SyntaxList<Self::Language>;

    /// Returns an iterator over all nodes with their trailing separator
    fn elements(&self) -> AstSeparatedListElementsIterator<Self::Language, Self::Node> {
        AstSeparatedListElementsIterator::new(self.syntax_list())
    }

    /// Returns an iterator over all separator tokens
    fn separators(&self) -> AstSeparatorIterator<Self::Language, Self::Node> {
        AstSeparatorIterator {
            inner: self.elements(),
        }
    }

    /// Returns an iterator over all nodes
    fn iter(&self) -> AstSeparatedListNodesIterator<Self::Language, Self::Node> {
        AstSeparatedListNodesIterator {
            inner: self.elements(),
        }
    }

    #[inline]
    fn is_empty(&self) -> bool {
        self.len() == 0
    }

    fn len(&self) -> usize {
        (self.syntax_list().len() + 1) / 2
    }

    fn trailing_separator(&self) -> Option<SyntaxToken<Self::Language>> {
        match self.syntax_list().last()? {
            SyntaxSlot::Token(token) => Some(token),
            _ => None,
        }
    }
}

pub struct AstSeparatorIterator<L: Language, N> {
    inner: AstSeparatedListElementsIterator<L, N>,
}

impl<L, N> Iterator for AstSeparatorIterator<L, N>
where
    L: Language,
    N: AstNode<Language = L>,
{
    type Item = SyntaxResult<SyntaxToken<L>>;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            let element = self.inner.next()?;

            match element.trailing_separator {
                Ok(Some(separator)) => return Some(Ok(separator)),
                Err(missing) => return Some(Err(missing)),
                _ => {}
            }
        }
    }
}

#[derive(Debug, Clone)]
pub struct AstSeparatedListElementsIterator<L: Language, N> {
    slots: SyntaxSlots<L>,
    ph: PhantomData<N>,
}

impl<L: Language, N: AstNode<Language = L>> AstSeparatedListElementsIterator<L, N> {
    fn new(list: &SyntaxList<L>) -> Self {
        Self {
            slots: list.iter(),
            ph: PhantomData,
        }
    }
}

impl<L: Language, N: AstNode<Language = L>> Iterator for AstSeparatedListElementsIterator<L, N> {
    type Item = AstSeparatedElement<L, N>;

    fn next(&mut self) -> Option<Self::Item> {
        let slot = self.slots.next()?;

        let node = match slot {
            // The node for this element is missing if the next child is a token instead of a node.
            SyntaxSlot::Token(token) => panic!("Malformed list, node expected but found token {:?} instead. You must add missing markers for missing elements.", token),
            // Missing element
            SyntaxSlot::Empty => Err(SyntaxError::MissingRequiredChild),
            SyntaxSlot::Node(node) => Ok(N::unwrap_cast(node))
        };

        let separator = match self.slots.next() {
            Some(SyntaxSlot::Empty) => Err(
                SyntaxError::MissingRequiredChild,
            ),
            Some(SyntaxSlot::Token(token)) => Ok(Some(token)),
            // End of list, no trailing separator
            None => Ok(None),
            Some(SyntaxSlot::Node(node)) => panic!("Malformed separated list, separator expected but found node {:?} instead. You must add missing markers for missing separators.", node),
        };

        Some(AstSeparatedElement {
            node,
            trailing_separator: separator,
        })
    }
}

impl<L: Language, N: AstNode<Language = L>> FusedIterator
    for AstSeparatedListElementsIterator<L, N>
{
}

#[derive(Debug, Clone)]
pub struct AstSeparatedListNodesIterator<L: Language, N> {
    inner: AstSeparatedListElementsIterator<L, N>,
}

impl<L: Language, N: AstNode<Language = L>> Iterator for AstSeparatedListNodesIterator<L, N> {
    type Item = SyntaxResult<N>;
    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next().map(|element| element.node)
    }
}

impl<L: Language, N: AstNode<Language = L>> FusedIterator for AstSeparatedListNodesIterator<L, N> {}

/// Specific result used when navigating nodes using AST APIs
pub type SyntaxResult<ResultType> = Result<ResultType, SyntaxError>;

#[derive(Debug, Eq, PartialEq, Clone, Copy)]
pub enum SyntaxError {
    /// Error thrown when a mandatory node is not found
    MissingRequiredChild,
}

impl Display for SyntaxError {
    fn fmt(&self, fmt: &mut Formatter<'_>) -> fmt::Result {
        match self {
            SyntaxError::MissingRequiredChild => fmt.write_str("missing required child"),
        }
    }
}

impl Error for SyntaxError {}

pub mod support {
    use super::{AstNode, SyntaxNode, SyntaxToken};

    use super::{Language, SyntaxError, SyntaxResult};
    use crate::syntax::SyntaxSlot;
    use crate::SyntaxElementChildren;
    use std::fmt::{Debug, Formatter};

    pub fn node<L: Language, N: AstNode<Language = L>>(
        parent: &SyntaxNode<L>,
        slot_index: usize,
    ) -> Option<N> {
        match parent.slots().nth(slot_index)? {
            SyntaxSlot::Empty => None,
            SyntaxSlot::Node(node) => Some(N::unwrap_cast(node)),
            SyntaxSlot::Token(token) => panic!(
                "expected a node in the slot {} but found token {:?}",
                slot_index, token
            ),
        }
    }

    pub fn required_node<L: Language, N: AstNode<Language = L>>(
        parent: &SyntaxNode<L>,
        slot_index: usize,
    ) -> SyntaxResult<N> {
        self::node(parent, slot_index).ok_or(SyntaxError::MissingRequiredChild)
    }

    pub fn elements<L: Language>(parent: &SyntaxNode<L>) -> SyntaxElementChildren<L> {
        parent.children_with_tokens()
    }

    pub fn list<L: Language, N: AstNode<Language = L>>(
        parent: &SyntaxNode<L>,
        slot_index: usize,
    ) -> N {
        required_node(parent, slot_index)
            .unwrap_or_else(|_| panic!("expected a list in slot {}", slot_index))
    }

    pub fn token<L: Language>(parent: &SyntaxNode<L>, slot_index: usize) -> Option<SyntaxToken<L>> {
        match parent.slots().nth(slot_index)? {
            SyntaxSlot::Empty => None,
            SyntaxSlot::Token(token) => Some(token),
            SyntaxSlot::Node(node) => panic!(
                "expected a token in the slot {} but found node {:?}",
                slot_index, node
            ),
        }
    }

    pub fn required_token<L: Language>(
        parent: &SyntaxNode<L>,
        slot_index: usize,
    ) -> SyntaxResult<SyntaxToken<L>> {
        token(parent, slot_index).ok_or(SyntaxError::MissingRequiredChild)
    }

    /// New-type wrapper to flatten the debug output of syntax result fields when printing [AstNode]s.
    /// Omits the [Ok] if the node is present and prints `missing (required)` if the child is missing
    pub struct DebugSyntaxResult<N>(pub SyntaxResult<N>);

    impl<N: Debug> Debug for DebugSyntaxResult<N> {
        fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
            match &self.0 {
                Ok(node) => std::fmt::Debug::fmt(node, f),
                Err(SyntaxError::MissingRequiredChild) => f.write_str("missing (required)"),
            }
        }
    }

    /// New-type wrapper to flatten the debug output of optional children when printing [AstNode]s.
    /// Omits the [Some] if the node is present and prints `missing (optional)` if the child is missing
    pub struct DebugOptionalElement<N>(pub Option<N>);

    impl<N: Debug> Debug for DebugOptionalElement<N> {
        fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
            match &self.0 {
                Some(node) => std::fmt::Debug::fmt(node, f),
                None => f.write_str("missing (optional)"),
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::raw_language::{
        LiteralExpression, RawLanguage, RawLanguageKind, RawSyntaxTreeBuilder,
        SeparatedExpressionList,
    };
    use crate::{AstNode, AstSeparatedElement, AstSeparatedList, SyntaxResult};

    /// Creates a ast separated list over a sequence of numbers separated by ",".
    /// The elements are pairs of: (value, separator).
    fn build_list<'a>(
        elements: impl IntoIterator<Item = (Option<i32>, Option<&'a str>)>,
    ) -> SeparatedExpressionList {
        let mut builder = RawSyntaxTreeBuilder::new();

        builder.start_node(RawLanguageKind::SEPARATED_EXPRESSION_LIST);

        for (node, separator) in elements.into_iter() {
            if let Some(node) = node {
                builder.start_node(RawLanguageKind::LITERAL_EXPRESSION);
                builder.token(RawLanguageKind::NUMBER_TOKEN, node.to_string().as_str());
                builder.finish_node();
            }

            if let Some(separator) = separator {
                builder.token(RawLanguageKind::COMMA_TOKEN, separator);
            }
        }

        builder.finish_node();

        let node = builder.finish();

        SeparatedExpressionList::new(node.into_list())
    }

    fn assert_elements<'a>(
        actual: impl Iterator<Item = AstSeparatedElement<RawLanguage, LiteralExpression>>,
        expected: impl IntoIterator<Item = (Option<f64>, Option<&'a str>)>,
    ) {
        let actual = actual.map(|element| {
            (
                element.node.ok().map(|n| n.text().parse::<f64>().unwrap()),
                element
                    .trailing_separator
                    .ok()
                    .flatten()
                    .map(|separator| separator.text().to_string()),
            )
        });

        let expected = expected
            .into_iter()
            .map(|(value, separator)| (value, separator.map(|sep| sep.to_string())))
            .collect::<Vec<_>>();

        assert_eq!(actual.collect::<Vec<_>>(), expected);
    }

    fn assert_nodes(
        actual: impl Iterator<Item = SyntaxResult<LiteralExpression>>,
        expected: impl IntoIterator<Item = f64>,
    ) {
        assert_eq!(
            actual
                .map(|literal| literal.unwrap().text().parse::<f64>().unwrap())
                .collect::<Vec<_>>(),
            expected.into_iter().collect::<Vec<_>>()
        );
    }

    #[test]
    fn empty() {
        let list = build_list(vec![]);

        assert_eq!(list.len(), 0);
        assert!(list.is_empty());
        assert_eq!(list.separators().count(), 0);

        assert_nodes(list.iter(), vec![]);
        assert_elements(list.elements(), vec![]);
        assert_eq!(list.trailing_separator(), None);
    }

    #[test]
    fn separated_list() {
        let list = build_list(vec![
            (Some(1), Some(",")),
            (Some(2), Some(",")),
            (Some(3), Some(",")),
            (Some(4), None),
        ]);

        assert_eq!(list.len(), 4);
        assert!(!list.is_empty());
        assert_eq!(list.separators().count(), 3);

        assert_nodes(list.iter(), vec![1., 2., 3., 4.]);
        assert_elements(
            list.elements(),
            vec![
                (Some(1.), Some(",")),
                (Some(2.), Some(",")),
                (Some(3.), Some(",")),
                (Some(4.), None),
            ],
        );
        assert_eq!(list.trailing_separator(), None);
    }

    #[test]
    fn separated_with_trailing() {
        // list(1, 2, 3, 4,)
        let list = build_list(vec![
            (Some(1), Some(",")),
            (Some(2), Some(",")),
            (Some(3), Some(",")),
            (Some(4), Some(",")),
        ]);

        assert_eq!(list.len(), 4);
        assert!(!list.is_empty());
        assert_nodes(list.iter(), vec![1., 2., 3., 4.]);
        assert_eq!(list.separators().count(), 4);

        assert_elements(
            list.elements(),
            vec![
                (Some(1.), Some(",")),
                (Some(2.), Some(",")),
                (Some(3.), Some(",")),
                (Some(4.), Some(",")),
            ],
        );
        assert!(list.trailing_separator().is_some());
    }

    #[test]
    fn separated_with_two_successive_separators() {
        // list([1,,])
        let list = build_list(vec![(Some(1), Some(",")), (None, Some(","))]);

        assert_eq!(list.len(), 2);
        assert!(!list.is_empty());
        assert_eq!(list.separators().count(), 2);

        assert_elements(
            list.elements(),
            vec![(Some(1.), Some(",")), (None, Some(","))],
        );
    }

    #[test]
    fn separated_with_leading_separator() {
        // list([,3])
        let list = build_list(vec![(None, Some(",")), (Some(3), None)]);

        assert_eq!(list.len(), 2);
        assert!(!list.is_empty());
        assert_eq!(list.separators().count(), 1);

        assert_elements(
            list.elements(),
            vec![
                // missing first element
                (None, Some(",")),
                (Some(3.), None),
            ],
        );
    }

    #[test]
    fn separated_with_two_successive_nodes() {
        // list([1 2,])
        let list = build_list(vec![(Some(1), None), (Some(2), Some(","))]);

        assert_eq!(list.len(), 2);
        assert!(!list.is_empty());
        assert_eq!(list.separators().count(), 2);

        assert_elements(
            list.elements(),
            vec![(Some(1.), None), (Some(2.), Some(","))],
        );
    }
}
