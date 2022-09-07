///! Utility function that applies some heuristic to format chain member expressions and call expressions
///!
///! We want to transform code that looks like this:
///!
///! ```js
///! something.execute().then().then().catch()
///! ```
///!
///! To something like this:
///!
///! ```js
///! something
///!   .execute()
///!   .then()
///!   .then()
///!   .catch()
///! ```
///!
///! In order to achieve that we use the same heuristic that [Prettier applies](https://github.com/prettier/prettier/blob/main/src/language-js/print/member-chain.js).
///!
///! The process is the following:
///!
///! ### Flattening the AST
///! We flatten the AST. See, the code above is actually nested, where the first member expression (`something`)
///! that we see is actually the last one. This is a oversimplified version of the AST:
///!
///! ```block
///! [
///!     .catch() [
///!         .then() [
///!             .then() [
///!                 .execute() [
///!                     something
///!                 ]
///!             ]
///!         ]
///!     ]
///! ]
///! ```
///! So we need to navigate the AST and make sure that `something` is actually
///! the first one. In a sense, we have to revert the chain of children. We will do that using a recursion.
///!
///! While we navigate the AST and we found particular nodes that we want to track, we also
///! format them. The format of these nodes is different from the standard version.
///!
///! Our formatter makes sure that we don't format twice the same nodes. Let's say for example that
///! we find a `something().execute()`, its AST is like this:
///!
///! ```block
///! JsCallExpression {
///!     callee: JsStaticMember {
///!         object: JsCallExpression {
///!             callee: Reference {
///!                 execute
///!             }
///!         }
///!     }
///! }
///! ```
///!
///! When we track the first [rome_js_syntax::JsCallExpression], we hold basically all the children,
///! that applies for the rest of the nodes. If we decided to format all the children of each node,
///! we will risk to format the last node, the `Reference`, four times.
///!
///! To avoid this, when we encounter particular nodes, we don't format all of its children, but defer
///! the formatting to the child itself.
///!
///! The end result of the flattening, will create an array of something like this:
///!
///! ```block
///! [ Identifier, JsCallExpression, JsStaticMember, JsCallExpression ]
///! ```
///!
///! ### Grouping
///!
///! After the flattening, we start the grouping. We want to group nodes in a way that will help us
///! to apply a deterministic formatting.
///! - first group will be the identifier
///! - the rest of the groups will be  will start StaticMemberExpression followed by the rest of the nodes,
///! right before the end of the next StaticMemberExpression
///!
///! The first group is special, because it holds the reference; it has its own heuristic.
///! Inside the first group we store the first element of the flattened array, then:
///!
///! 1. as many as [rome_js_syntax::JsCallExpression] we can find, this cover cases like
///! `something()()().then()`;
///! 2. as many as [rome_js_syntax::JsComputedMemberExpression] we can find, this cover cases like
///! `something()()[1][3].then()`;
///! 3. as many as consecutive [rome_js_syntax::JsStaticMemberExpression] or [rome_js_syntax::JsComputedMemberExpression], this cover cases like
///! `this.items[0].then()`
///!
///! The rest of the groups are essentially a sequence of `[StaticMemberExpression , CallExpression]`.
///! In order to achieve that, we simply start looping through the rest of the flatten items that we haven't seen.
///!
///! Eventually, we should have something like this:
///!
///! ```block
///! [
///!     [ReferenceIdentifier, CallExpression], // with possible computed expressions in the middle
///!     [StaticMemberExpression, StaticMemberExpression, CallExpression],
///!     [StaticMemberExpression, CallExpression],
///!     [StaticMemberExpression],
///! ]
///! ```
mod chain_member;
mod groups;
mod simple_argument;

use crate::prelude::*;
use crate::utils::member_chain::chain_member::ChainMember;
use crate::utils::member_chain::groups::{
    MemberChainGroup, MemberChainGroups, MemberChainGroupsBuilder,
};
use crate::utils::member_chain::simple_argument::SimpleArgument;
use rome_formatter::{format_args, write, Buffer, Comments, CstFormatContext};
use rome_js_syntax::{JsAnyExpression, JsCallExpression, JsExpressionStatement, JsLanguage};
use rome_rowan::{AstNode, SyntaxResult};

#[derive(Debug, Clone)]
pub(crate) struct MemberChain<'a> {
    calls_count: usize,
    root: &'a JsCallExpression,
    head: MemberChainGroup,
    tail: MemberChainGroups,
}

impl MemberChain<'_> {
    /// It tells if the groups should be break on multiple lines
    pub(crate) fn groups_should_break(
        &self,
        comments: &Comments<JsLanguage>,
    ) -> FormatResult<bool> {
        // Do not allow the group to break if it only contains a single call expression
        if self.calls_count <= 1 {
            return Ok(false);
        }

        // we want to check the simplicity of the call expressions only if we have at least
        // two of them
        // Check prettier: https://github.com/prettier/prettier/blob/main/src/language-js/print/member-chain.js#L389
        let call_expressions_are_not_simple =
            self.calls_count > 2 && self.call_expressions_are_not_simple()?;

        // TODO: add here will_break logic

        let node_has_comments =
            self.tail.has_comments(comments)? || self.head.has_comments(comments);

        let should_break = node_has_comments || call_expressions_are_not_simple;

        Ok(should_break)
    }

    /// We retrieve all the call expressions inside the group and we check if
    /// their arguments are not simple.
    fn call_expressions_are_not_simple(&self) -> SyntaxResult<bool> {
        Ok(self.tail.get_call_expressions().any(|call_expression| {
            call_expression.arguments().map_or(false, |arguments| {
                !arguments
                    .args()
                    .iter()
                    .filter_map(|argument| argument.ok())
                    .all(|argument| SimpleArgument::new(argument).is_simple(0))
            })
        }))
    }
}

impl Format<JsFormatContext> for MemberChain<'_> {
    fn fmt(&self, f: &mut Formatter<JsFormatContext>) -> FormatResult<()> {
        // TODO use Alternatives once available
        write!(f, [&self.head])?;

        if self.groups_should_break(f.context().comments())? {
            write!(
                f,
                [indent(&format_args!(
                    hard_line_break(),
                    format_with(|f| {
                        f.join_with(hard_line_break())
                            .entries(self.tail.iter())
                            .finish()
                    })
                ))]
            )
        } else {
            write!(f, [self.tail])
        }
    }
}

pub(crate) fn get_member_chain<'a>(
    call_expression: &'a JsCallExpression,
    f: &mut JsFormatter,
) -> SyntaxResult<MemberChain<'a>> {
    let mut chain_members = vec![];
    let parent_is_expression_statement =
        call_expression.syntax().parent().map_or(false, |parent| {
            JsExpressionStatement::can_cast(parent.kind())
        });

    let root = flatten_member_chain(
        &mut chain_members,
        call_expression.clone().into(),
        f.context().comments(),
        true,
    )?;

    chain_members.push(root);

    // Count the number of CallExpression in the chain,
    // will be used later to decide on how to format it
    let calls_count = chain_members
        .iter()
        .filter(|item| item.is_loose_call_expression())
        .count();

    // as explained before, the first group is particular, so we calculate it
    let index_to_split_at = compute_first_group_index(&chain_members);

    // we have the index where we want to take the first group
    let remaining_groups = chain_members.split_off(index_to_split_at);
    let first_group = chain_members;

    let mut head_group = MemberChainGroup::from(first_group);

    // `flattened_items` now contains only the nodes that should have a sequence of
    // `[ StaticMemberExpression -> AnyNode + JsCallExpression ]`
    let mut rest_of_groups = compute_groups(
        remaining_groups.into_iter(),
        parent_is_expression_statement,
        f,
    );

    // Here we check if the first element of Groups::groups can be moved inside the head.
    // If so, then we extract it and concatenate it together with the head.
    if let Some(group_to_merge) =
        rest_of_groups.should_merge_with_first_group(&head_group, f.comments())
    {
        let group_to_merge = group_to_merge
            .into_iter()
            .flat_map(|group| group.into_members());
        head_group.expand_group(group_to_merge);
    }

    Ok(MemberChain {
        calls_count,
        root: call_expression,
        head: head_group,
        tail: rest_of_groups,
    })
}

/// Retrieves the index where we want to calculate the first group.
/// The first group gathers inside it all those nodes that are not a sequence of something like:
/// `[ StaticMemberExpression -> AnyNode + JsCallExpression ]`
fn compute_first_group_index(flatten_items: &[ChainMember]) -> usize {
    flatten_items
        .iter()
        .enumerate()
        // the first element will always be part of the first group, so we skip it
        .skip(1)
        // we now find the index, all items before this index will belong to the first group
        .find_map(|(index, item)| {
            let should_skip = match item {
                // This where we apply the first two points explained in the description of the main public function.
                // We want to keep iterating over the items until we have call expressions or computed expressions:
                // - `something()()()()`
                // - `something[1][2][4]`
                // - `something[1]()[3]()`
                // - `something()[2].something.else[0]`
                ChainMember::CallExpression { .. } | ChainMember::ComputedMember { .. } => true,

                // SAFETY: The check `flatten_items[index + 1]` will never panic at runtime because
                // 1. The array will always have at least two items
                // 2. The last element of the array is always a CallExpression
                //
                // Something like `a()` produces these flatten times:
                // ```
                // [
                //      Token("a", 0..1),
                //      CallExpression: [Empty, Empty, Group(List [Token("(", 5..6), Token(")", 2..7)])],
                // ]
                // ```
                //
                // Hence, it will never enter the branch of this `match`.
                //
                // When we have something like `a.b.c()`, the flatten items produced are:
                //
                // ```
                // [
                //      Token("a", 0..1),
                //      StaticMember: [Token(".", 1..2), Token("b", 2..3)],
                //      StaticMember: [Token(".", 3..4), Token("c", 4..5)],
                //      CallExpression: [Empty, Empty, Group(List [Token("(", 5..6), Token(")", 6..7)])],
                // ]
                // ```
                //
                // The loop will match against `StaticMember: [Token(".", 3..4), Token("c", 4..5)],`
                // and the next one is a call expression... the `matches!` fails and the loop is stopped.
                //
                // The last element of the array is always a `CallExpression`, which allows us to avoid the overflow of the array.
                ChainMember::StaticMember { .. } => {
                    let next_flatten_item = &flatten_items[index + 1];
                    matches!(
                        next_flatten_item,
                        ChainMember::StaticMember { .. } | ChainMember::ComputedMember { .. }
                    )
                }
                _ => false,
            };

            if should_skip {
                None
            } else {
                Some(index)
            }
        })
        // If the above returns None this means either all items were skipped
        // or the list was empty. In either case, this means the first group
        // covers the entire list of [FlattenItem]
        .unwrap_or(flatten_items.len())
}

/// computes groups coming after the first group
fn compute_groups(
    flatten_items: impl Iterator<Item = ChainMember>,
    in_expression_statement: bool,
    f: &JsFormatter,
) -> MemberChainGroups {
    let mut has_seen_call_expression = false;
    let mut groups_builder =
        MemberChainGroupsBuilder::new(in_expression_statement, f.options().tab_width());
    for item in flatten_items {
        let has_trailing_comments = f.comments().has_trailing_comments(item.syntax());

        match item {
            ChainMember::StaticMember { .. } => {
                // if we have seen a JsCallExpression, we want to close the group.
                // The resultant group will be something like: [ . , then, () ];
                // `.` and `then` belong to the previous StaticMemberExpression,
                // and `()` belong to the call expression we just encountered

                if has_seen_call_expression {
                    groups_builder.close_group();
                    groups_builder.start_or_continue_group(item);
                    has_seen_call_expression = false;
                } else {
                    groups_builder.start_or_continue_group(item);
                }
            }
            ChainMember::CallExpression { .. } => {
                let is_loose_call_expression = item.is_loose_call_expression();
                groups_builder.start_or_continue_group(item);
                if is_loose_call_expression {
                    has_seen_call_expression = true;
                }
            }
            ChainMember::ComputedMember { .. } => {
                groups_builder.start_or_continue_group(item);
            }
            ChainMember::Node(_) => groups_builder.continue_group(item),
        }

        // Close the group immediately if the node had any trailing comments to
        // ensure those are printed in a trailing position for the token they
        // were originally commenting
        if has_trailing_comments {
            groups_builder.close_group();
        }
    }

    // closing possible loose groups
    groups_builder.close_group();

    groups_builder.finish()
}

/// This function tries to flatten the AST. It stores nodes and its formatted version
/// inside an vector of [FlattenItem]. The first element of the vector is the last one.
fn flatten_member_chain(
    queue: &mut Vec<ChainMember>,
    node: JsAnyExpression,
    comments: &Comments<JsLanguage>,
    root: bool,
) -> SyntaxResult<ChainMember> {
    use JsAnyExpression::*;

    if comments.is_suppressed(node.syntax()) {
        return Ok(ChainMember::Node(node.into_syntax()));
    }

    let member = match node {
        JsCallExpression(call_expression) => {
            let callee = call_expression.callee()?;
            let left = flatten_member_chain(queue, callee, comments, false)?;
            queue.push(left);

            ChainMember::CallExpression {
                expression: call_expression,
                root,
            }
        }

        JsStaticMemberExpression(static_member) => {
            let object = static_member.object()?;
            let left = flatten_member_chain(queue, object, comments, false)?;
            queue.push(left);

            ChainMember::StaticMember {
                expression: static_member,
                root,
            }
        }

        JsComputedMemberExpression(computed_expression) => {
            let object = computed_expression.object()?;

            let left = flatten_member_chain(queue, object, comments, false)?;
            queue.push(left);

            ChainMember::ComputedMember {
                expression: computed_expression,
                root,
            }
        }
        expression => ChainMember::Node(expression.into_syntax()),
    };

    Ok(member)
}

/// Here we check if the length of the groups exceeds the cutoff or there are comments
/// This function is the inverse of the prettier function
/// [Prettier applies]: https://github.com/prettier/prettier/blob/a043ac0d733c4d53f980aa73807a63fc914f23bd/src/language-js/print/member-chain.js#L342
pub fn is_member_call_chain(
    expression: &JsCallExpression,
    f: &mut JsFormatter,
) -> SyntaxResult<bool> {
    let chain = get_member_chain(expression, f)?;

    chain.tail.is_member_call_chain(f.context().comments())
}
