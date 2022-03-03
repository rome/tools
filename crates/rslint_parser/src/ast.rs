//! AST definitions for converting untyped syntax nodes into typed AST nodes.
//!
//! Every field of every AST node is optional, this is to allow the parser to recover
//! from any error and produce an ast from any source code. If you don't want to account for
//! optionals for everything, you can use ...

#[macro_use]
mod expr_ext;
mod generated;
mod modifier_ext;
mod stmt_ext;
mod union_ext;

use crate::{syntax_node::*, util::SyntaxNodeExt, JsSyntaxKind, SyntaxList, TextRange};
use std::fmt::{Debug, Formatter};
use std::iter::FusedIterator;
use std::marker::PhantomData;
use thiserror::Error;

pub(crate) use self::generated::syntax_factory::JsSyntaxFactory;
pub use self::modifier_ext::Modifiers;
pub use self::{expr_ext::*, generated::nodes::*, stmt_ext::*};

/// The main trait to go from untyped `SyntaxNode`  to a typed ast. The
/// conversion itself has zero runtime cost: ast and syntax nodes have exactly
/// the same representation: a pointer to the tree root and a pointer to the
/// node itself.
pub trait AstNode {
    fn can_cast(kind: JsSyntaxKind) -> bool
    where
        Self: Sized;

    fn cast(syntax: SyntaxNode) -> Option<Self>
    where
        Self: Sized;

    fn syntax(&self) -> &SyntaxNode;

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

/// Like `AstNode`, but wraps tokens rather than interior nodes.
pub trait AstToken {
    fn can_cast(token: JsSyntaxKind) -> bool
    where
        Self: Sized;

    fn cast(syntax: SyntaxToken) -> Option<Self>
    where
        Self: Sized;

    fn syntax(&self) -> &SyntaxToken;

    fn text(&self) -> &str {
        self.syntax().text()
    }
}

/// List of homogenous nodes
pub trait AstNodeList<N>
where
    N: AstNode,
{
    /// Returns the underlying syntax list
    fn syntax_list(&self) -> &SyntaxList;

    fn iter(&self) -> AstNodeListIterator<N> {
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
    fn first(&self) -> Option<N> {
        self.iter().next()
    }

    /// Returns the last node from this list or None
    fn last(&self) -> Option<N> {
        self.iter().last()
    }

    #[inline]
    fn is_empty(&self) -> bool {
        self.syntax_list().is_empty()
    }
}

#[derive(Debug, Clone)]
pub struct AstNodeListIterator<N> {
    inner: SyntaxSlots,
    ph: PhantomData<N>,
}

impl<N: AstNode> AstNodeListIterator<N> {
    fn slot_to_node(slot: &SyntaxSlot) -> N {
        match slot {
            SyntaxSlot::Empty => panic!("Node isn't permitted to contain empty slots"),
            SyntaxSlot::Node(node) => node.to(),
            SyntaxSlot::Token(token) => panic!(
                "Expected node of type `{:?}` but found token `{:?}` instead.",
                std::any::type_name::<N>(),
                token
            ),
        }
    }
}

impl<N: AstNode> Iterator for AstNodeListIterator<N> {
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

impl<N: AstNode> ExactSizeIterator for AstNodeListIterator<N> {}

impl<N: AstNode> FusedIterator for AstNodeListIterator<N> {}

#[derive(Clone)]
pub struct AstSeparatedElement<N> {
    node: SyntaxResult<N>,
    trailing_separator: SyntaxResult<Option<SyntaxToken>>,
}

impl<N: AstNode + Clone> AstSeparatedElement<N> {
    pub fn node(&self) -> SyntaxResult<N> {
        self.node.clone()
    }

    pub fn trailing_separator(&self) -> SyntaxResult<Option<SyntaxToken>> {
        self.trailing_separator.clone()
    }
}

impl<N: Debug> Debug for AstSeparatedElement<N> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match &self.node {
            Ok(node) => N::fmt(node, f)?,
            Err(_) => f.write_str("missing element")?,
        };
        match &self.trailing_separator {
            Ok(Some(separator)) => {
                f.write_str(",\n")?;
                separator.fmt(f)
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
pub trait AstSeparatedList<N>
where
    N: AstNode,
{
    /// Returns the underlying syntax list
    fn syntax_list(&self) -> &SyntaxList;

    /// Returns an iterator over all nodes with their trailing separator
    fn elements(&self) -> AstSeparatedListElementsIterator<N> {
        AstSeparatedListElementsIterator::new(self.syntax_list())
    }

    /// Returns an iterator over all separator tokens
    fn separators(&self) -> AstSeparatorIterator<N> {
        AstSeparatorIterator {
            inner: self.elements(),
        }
    }

    /// Returns an iterator over all nodes
    fn iter(&self) -> AstSeparatedListNodesIterator<N> {
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

    fn trailing_separator(&self) -> Option<SyntaxToken> {
        match self.syntax_list().last()? {
            SyntaxSlot::Token(token) => Some(token),
            _ => None,
        }
    }
}

pub struct AstSeparatorIterator<N> {
    inner: AstSeparatedListElementsIterator<N>,
}

impl<N> Iterator for AstSeparatorIterator<N>
where
    N: AstNode,
{
    type Item = SyntaxResult<SyntaxToken>;

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
pub struct AstSeparatedListElementsIterator<N> {
    slots: SyntaxSlots,
    parent: SyntaxNode,
    ph: PhantomData<N>,
}

impl<N: AstNode> AstSeparatedListElementsIterator<N> {
    fn new(list: &SyntaxList) -> Self {
        Self {
            slots: list.iter(),
            parent: list.node().clone(),
            ph: PhantomData,
        }
    }
}

impl<N: AstNode> Iterator for AstSeparatedListElementsIterator<N> {
    type Item = AstSeparatedElement<N>;

    fn next(&mut self) -> Option<Self::Item> {
        let slot = self.slots.next()?;

        let node = match slot {
            // The node for this element is missing if the next child is a token instead of a node.
            SyntaxSlot::Token(token) => panic!("Malformed list, node expected but found token {:?} instead. You must add missing markers for missing elements.", token),
            // Missing element
            SyntaxSlot::Empty => Err(SyntaxError::MissingRequiredChild(
                    self.parent.clone(),
                )),
            SyntaxSlot::Node(node) => Ok(node.to::<N>())
        };

        let separator = match self.slots.next() {
            Some(SyntaxSlot::Empty) => Err(
                SyntaxError::MissingRequiredChild(self.parent.clone()),
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

impl<N: AstNode> FusedIterator for AstSeparatedListElementsIterator<N> {}

#[derive(Debug, Clone)]
pub struct AstSeparatedListNodesIterator<N> {
    inner: AstSeparatedListElementsIterator<N>,
}

impl<N: AstNode> Iterator for AstSeparatedListNodesIterator<N> {
    type Item = SyntaxResult<N>;
    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next().map(|element| element.node)
    }
}

impl<N: AstNode> FusedIterator for AstSeparatedListNodesIterator<N> {}

/// Specific result used when navigating nodes using AST APIs
pub type SyntaxResult<ResultType> = Result<ResultType, SyntaxError>;

#[derive(Debug, Eq, PartialEq, Clone, Error)]
pub enum SyntaxError {
    /// Error thrown when a mandatory node is not found
    #[error("missing required child")]
    MissingRequiredChild(SyntaxNode),
}

mod support {
    use super::{AstNode, SyntaxNode, SyntaxToken};
    use crate::ast::DebugSyntaxElement;
    use crate::{SyntaxElementChildren, SyntaxNodeExt};
    use crate::{SyntaxError, SyntaxResult};
    use rome_rowan::SyntaxSlot;
    use std::fmt::{Debug, Formatter};

    pub(super) fn node<N: AstNode>(parent: &SyntaxNode, slot_index: usize) -> Option<N> {
        match parent.slots().nth(slot_index)? {
            SyntaxSlot::Empty => None,
            SyntaxSlot::Node(node) => Some(node.to()),
            SyntaxSlot::Token(token) => panic!(
                "expected a node in the slot {} but found token {:?}",
                slot_index, token
            ),
        }
    }

    pub(super) fn required_node<N: AstNode>(
        parent: &SyntaxNode,
        slot_index: usize,
    ) -> SyntaxResult<N> {
        self::node(parent, slot_index)
            .ok_or_else(|| SyntaxError::MissingRequiredChild(parent.clone()))
    }

    pub(super) fn elements(parent: &SyntaxNode) -> SyntaxElementChildren {
        parent.children_with_tokens()
    }

    pub(super) fn list<L: AstNode>(parent: &SyntaxNode, slot_index: usize) -> L {
        required_node(parent, slot_index)
            .unwrap_or_else(|_| panic!("expected a list in slot {}", slot_index))
    }

    pub(super) fn token(parent: &SyntaxNode, slot_index: usize) -> Option<SyntaxToken> {
        match parent.slots().nth(slot_index)? {
            SyntaxSlot::Empty => None,
            SyntaxSlot::Token(token) => Some(token),
            SyntaxSlot::Node(node) => panic!(
                "expected a token in the slot {} but found node {:?}",
                slot_index, node
            ),
        }
    }

    pub(super) fn required_token(
        parent: &SyntaxNode,
        slot_index: usize,
    ) -> SyntaxResult<SyntaxToken> {
        token(parent, slot_index).ok_or_else(|| SyntaxError::MissingRequiredChild(parent.clone()))
    }

    /// New-type wrapper to flatten the debug output of syntax result fields when printing [AstNode]s.
    /// Omits the [Ok] if the node is present and prints `missing (required)` if the child is missing
    pub(super) struct DebugSyntaxResult<N>(pub(super) SyntaxResult<N>);

    impl<N: Debug> Debug for DebugSyntaxResult<N> {
        fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
            match &self.0 {
                Ok(node) => std::fmt::Debug::fmt(node, f),
                Err(SyntaxError::MissingRequiredChild(_)) => f.write_str("missing (required)"),
            }
        }
    }

    /// New-type wrapper to flatten the debug output of optional children when printing [AstNode]s.
    /// Omits the [Some] if the node is present and prints `missing (optional)` if the child is missing
    pub(super) struct DebugOptionalElement<N>(pub(super) Option<N>);

    impl<N: Debug> Debug for DebugOptionalElement<N> {
        fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
            match &self.0 {
                Some(node) => std::fmt::Debug::fmt(node, f),
                None => f.write_str("missing (optional)"),
            }
        }
    }

    #[derive(Clone)]
    pub(super) struct DebugSyntaxElementChildren(pub(super) SyntaxElementChildren);

    impl Debug for DebugSyntaxElementChildren {
        fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
            f.debug_list()
                .entries(self.clone().0.map(DebugSyntaxElement))
                .finish()
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::ast::{
        AstSeparatedElement, AstSeparatedList, JsNumberLiteralExpression, JsSyntaxFactory,
    };
    use crate::{AstNode, JsLanguage, JsSyntaxKind, SyntaxNode, SyntaxResult};
    use rome_rowan::{SyntaxList, TreeBuilder};

    struct TestList {
        syntax_list: SyntaxList<JsLanguage>,
    }

    impl TestList {
        fn new(list: SyntaxList<JsLanguage>) -> Self {
            Self { syntax_list: list }
        }
    }

    impl AstSeparatedList<JsNumberLiteralExpression> for TestList {
        fn syntax_list(&self) -> &crate::SyntaxList {
            &self.syntax_list
        }
    }

    impl AstNode for TestList {
        fn can_cast(_: JsSyntaxKind) -> bool {
            unimplemented!()
        }

        fn cast(_: SyntaxNode) -> Option<Self>
        where
            Self: Sized,
        {
            unimplemented!()
        }

        fn syntax(&self) -> &SyntaxNode {
            self.syntax_list.node()
        }
    }

    /// Creates a ast separated list over a sequence of numbers separated by ",".
    /// The elements are pairs of: (value, separator).
    fn build_list<'a>(
        elements: impl IntoIterator<Item = (Option<i32>, Option<&'a str>)>,
    ) -> TestList {
        let mut builder: TreeBuilder<JsLanguage, JsSyntaxFactory> = TreeBuilder::new();

        builder.start_node(JsSyntaxKind::JS_ARRAY_ELEMENT_LIST);

        for (node, separator) in elements.into_iter() {
            if let Some(node) = node {
                builder.start_node(JsSyntaxKind::JS_NUMBER_LITERAL_EXPRESSION);
                builder.token(JsSyntaxKind::JS_NUMBER_LITERAL, node.to_string().as_str());
                builder.finish_node();
            }

            if let Some(separator) = separator {
                builder.token(JsSyntaxKind::COMMA, separator);
            }
        }

        builder.finish_node();

        let node = builder.finish();

        TestList::new(node.into_list())
    }

    fn assert_elements<'a>(
        actual: impl Iterator<Item = AstSeparatedElement<JsNumberLiteralExpression>>,
        expected: impl IntoIterator<Item = (Option<f64>, Option<&'a str>)>,
    ) {
        let actual = actual.map(|element| {
            (
                element.node.ok().map(|n| n.as_number().unwrap()),
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
        actual: impl Iterator<Item = SyntaxResult<JsNumberLiteralExpression>>,
        expected: impl IntoIterator<Item = f64>,
    ) {
        assert_eq!(
            actual
                .map(|literal| literal.unwrap().as_number().unwrap())
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
