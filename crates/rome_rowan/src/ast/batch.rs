use crate::{AstNode, Language, NodeOrToken, SyntaxNode, SyntaxNodeCast};
use std::{collections::BinaryHeap, iter::once};

pub trait BatchMutationExt<L>: AstNode<Language = L>
where
    L: Language,
{
    fn begin(self) -> BatchMutation<L, Self>;
}

impl<L, T> BatchMutationExt<L> for T
where
    L: Language,
    T: AstNode<Language = L>,
{
    fn begin(self) -> BatchMutation<L, Self> {
        BatchMutation {
            root: self,
            changes: BinaryHeap::new(),
        }
    }
}

/// Stores the changes internally used by the [BatchMutation::commit] algorithm.
/// It needs to be sorted by depth, then by range start and range end.
///
/// This is necesasry so we can aggregate all changes to the same node using "peek".
#[derive(Debug)]
struct CommitChange<L: Language> {
    parent_depth: usize,
    parent: Option<SyntaxNode<L>>,
    parent_range: Option<(u32, u32)>,
    new_node_slot: usize,
    new_node: Option<SyntaxNode<L>>,
}

impl<L: Language> PartialEq for CommitChange<L> {
    fn eq(&self, other: &Self) -> bool {
        self.parent == other.parent
    }
}
impl<L: Language> Eq for CommitChange<L> {}

impl<L: Language> PartialOrd for CommitChange<L> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        let self_range = self.parent_range.unwrap_or((0u32, 0u32));
        let other_range = other.parent_range.unwrap_or((0u32, 0u32));

        (self.parent_depth, self_range.0, self_range.1).partial_cmp(&(
            other.parent_depth,
            other_range.0,
            other_range.1,
        ))
    }
}
impl<L: Language> Ord for CommitChange<L> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.parent_depth.cmp(&other.parent_depth)
    }
}

pub struct BatchMutation<L, N>
where
    L: Language,
    N: AstNode<Language = L>,
{
    root: N,
    changes: BinaryHeap<CommitChange<L>>,
}

impl<L, N> BatchMutation<L, N>
where
    L: Language,
    N: AstNode<Language = L>,
{
    pub fn replace_node<T>(&mut self, prev_node: T, next_node: T)
    where
        T: AstNode<Language = L>,
    {
        let prev_node = prev_node.into_syntax();
        let new_node_slot = prev_node.index();
        let parent = prev_node.parent();
        let parent_range: Option<(u32, u32)> = parent.as_ref().map(|p| {
            let range = p.text_range();
            (range.start().into(), range.end().into())
        });
        let parent_depth = parent.as_ref().map(|p| p.ancestors().count()).unwrap_or(0);

        self.changes.push(CommitChange {
            parent_depth,
            parent,
            parent_range,
            new_node_slot,
            new_node: Some(next_node.into_syntax()),
        });
    }

    /// The core of the batch mutation algorithm can be summarized as:
    /// 1 - Iterate all requested changes;
    /// 2 - Insert them into a heap (priority queue) by depth. Deeper changes are done first;
    /// 3 - Loop popping requested changes from the heap, taking the deepest change we have for the moment;
    /// 4 - Each requested change has a "parent", an "index" and the "new node" (or None);
    /// 5 - Clone the current parent's "parent", the "greatparent";
    /// 6 - Detach the current "parent" from the tree;
    /// 7 - Replace the old node at "index" at the current "parent" with the current "new node";
    /// 8 - Insert into the heap the greatparent as the parent and the current "parent" as the "new node";
    ///
    /// This is the simple case. The algorithm also has a more complex case when to changes have a common ancestor,
    /// which can actually be one of the changed nodes.
    ///
    /// To address this case at step 3, when we pop a new change to apply it, we actually aggregate all changes to the current
    /// parent together. This is done by the heap because we also sort by node and it's range.
    ///
    pub fn commit(self) -> N {
        let BatchMutation { root, mut changes } = self;
        // Fill the heap with the requested changes

        while let Some(item) = changes.pop() {
            // If parent is None, we reached the root
            if let Some(current_parent) = item.parent {
                // This must be done before the detachment below
                // because we need nodes that are still valid in the old tree

                let grandparent = current_parent.parent();
                let grandparent_range = grandparent.as_ref().map(|g| {
                    let range = g.text_range();
                    (range.start().into(), range.end().into())
                });
                let currentparent_slot = current_parent.index();

                // Aggregate all modifications to the current parent
                // This works because of the Ord we defined in the [CommitChange] struct

                let mut modifications = vec![(item.new_node_slot, item.new_node)];
                loop {
                    if let Some(next_change_parent) = changes.peek().and_then(|i| i.parent.as_ref())
                    {
                        if *next_change_parent == current_parent {
                            // SAFETY: We can .pop().unwrap() because we .peek() above
                            let next_change = changes.pop().unwrap();
                            modifications.push((next_change.new_node_slot, next_change.new_node));
                            continue;
                        }
                    }
                    break;
                }

                // Now we detach the current parent, make all the modifications
                // and push a pending change to its parent.

                let mut current_parent = current_parent.detach();

                for (index, node) in modifications {
                    let replace_with = node.map(NodeOrToken::Node);
                    current_parent = current_parent.splice_slots(index..=index, once(replace_with));
                }

                changes.push(CommitChange {
                    parent_depth: item.parent_depth - 1,
                    parent: grandparent,
                    parent_range: grandparent_range,
                    new_node_slot: currentparent_slot,
                    new_node: Some(current_parent),
                });
            } else {
                return item.new_node.and_then(|x| x.cast()).unwrap();
            }
        }

        root
    }
}

#[cfg(test)]
pub mod tests {
    use crate::{
        raw_language::{LiteralExpression, RawLanguageKind, RawLanguageRoot, RawSyntaxTreeBuilder},
        AstNode, BatchMutationExt, SyntaxNodeCast,
    };

    /// ```
    /// 0: ROOT@0..1
    ///     0: LITERAL_EXPRESSION@0..1
    ///         0: STRING_TOKEN@0..1 "a" [] []
    /// ```
    fn tree_one(a: &str) -> (RawLanguageRoot, String) {
        let mut builder = RawSyntaxTreeBuilder::new();
        builder
            .start_node(RawLanguageKind::ROOT)
            .start_node(RawLanguageKind::LITERAL_EXPRESSION)
            .token(RawLanguageKind::STRING_TOKEN, a)
            .finish_node()
            .finish_node();
        let root = builder.finish().cast::<RawLanguageRoot>().unwrap();
        let s = format!("{:#?}", root.syntax());
        (root, s)
    }

    /// ```
    /// 0: ROOT@0..1
    ///     0: LITERAL_EXPRESSION@0..1
    ///         0: STRING_TOKEN@0..1 "a" [] []
    ///     1: LITERAL_EXPRESSION@0..1
    ///         0: STRING_TOKEN@0..1 "b" [] []
    /// ```
    fn tree_two(a: &str, b: &str) -> (RawLanguageRoot, String) {
        let mut builder = RawSyntaxTreeBuilder::new();
        builder
            .start_node(RawLanguageKind::ROOT)
            .start_node(RawLanguageKind::LITERAL_EXPRESSION)
            .token(RawLanguageKind::STRING_TOKEN, a)
            .finish_node()
            .start_node(RawLanguageKind::LITERAL_EXPRESSION)
            .token(RawLanguageKind::STRING_TOKEN, b)
            .finish_node()
            .finish_node();
        let root = builder.finish().cast::<RawLanguageRoot>().unwrap();
        let s = format!("{:#?}", root.syntax());
        (root, s)
    }

    fn find(root: &RawLanguageRoot, name: &str) -> LiteralExpression {
        root.syntax()
            .descendants()
            .find(|x| x.kind() == RawLanguageKind::LITERAL_EXPRESSION && x.text_trimmed() == name)
            .unwrap()
            .cast::<LiteralExpression>()
            .unwrap()
    }

    fn clone_detach(root: &RawLanguageRoot, name: &str) -> LiteralExpression {
        root.syntax()
            .descendants()
            .find(|x| x.kind() == RawLanguageKind::LITERAL_EXPRESSION && x.text_trimmed() == name)
            .unwrap()
            .detach()
            .cast::<LiteralExpression>()
            .unwrap()
    }

    #[test]
    pub fn ok_batch_mutation_no_changes() {
        let (before, before_debug) = tree_one("a");

        let batch = before.begin();
        let after = batch.commit();

        assert_eq!(before_debug, format!("{:#?}", after.syntax()));
    }

    #[test]
    pub fn ok_batch_mutation_one_change() {
        let (before, _) = tree_one("a");
        let (expected, expected_debug) = tree_one("b");

        let a = find(&before, "a");
        let b = clone_detach(&expected, "b");

        let mut batch = before.begin();
        batch.replace_node(a, b);
        let root = batch.commit();

        assert_eq!(expected_debug, format!("{:#?}", root.syntax()));
    }

    #[test]
    pub fn ok_batch_mutation_multiple_changes_different_branches() {
        let (before, _) = tree_two("a", "b");
        let (expected, expected_debug) = tree_two("c", "d");

        let a = find(&before, "a");
        let b = find(&before, "b");
        let c = clone_detach(&expected, "c");
        let d = clone_detach(&expected, "d");

        let mut batch = before.begin();
        batch.replace_node(a, c);
        batch.replace_node(b, d);
        let after = batch.commit();

        assert_eq!(expected_debug, format!("{:#?}", after.syntax()));
    }
}
