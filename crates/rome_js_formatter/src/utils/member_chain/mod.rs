mod flatten_item;
mod groups;
mod simple_argument;

use crate::prelude::*;
use crate::utils::member_chain::flatten_item::FlattenItem;
use crate::utils::member_chain::groups::{Groups, HeadGroup};
use rome_formatter::{format_args, write, Buffer, Comments, CstFormatContext, PreambleBuffer};
use rome_js_syntax::{
    JsCallExpression, JsComputedMemberExpression, JsExpressionStatement, JsLanguage,
    JsStaticMemberExpression,
};
use rome_js_syntax::{JsSyntaxKind, JsSyntaxNode};
use rome_rowan::{AstNode, SyntaxResult};

/// Utility function that applies some heuristic to format chain member expressions and call expressions
///
/// We want to transform code that looks like this:
///
/// ```js
/// something.execute().then().then().catch()
/// ```
///
/// To something like this:
///
/// ```js
/// something
///   .execute()
///   .then()
///   .then()
///   .catch()
/// ```
///
/// In order to achieve that we use the same heuristic that [Prettier applies].
///
/// The process is the following:
///
/// ### Flattening the AST
/// We flatten the AST. See, the code above is actually nested, where the first member expression (`something`)
/// that we see is actually the last one. This is a oversimplified version of the AST:
///
/// ```block
/// [
///     .catch() [
///         .then() [
///             .then() [
///                 .execute() [
///                     something
///                 ]
///             ]
///         ]
///     ]
/// ]
/// ```
/// So we need to navigate the AST and make sure that `something` is actually
/// the first one. In a sense, we have to revert the chain of children. We will do that using a recursion.
///
/// While we navigate the AST and we found particular nodes that we want to track, we also
/// format them. The format of these nodes is different from the standard version.
///
/// Our formatter makes sure that we don't format twice the same nodes. Let's say for example that
/// we find a `something().execute()`, its AST is like this:
///
/// ```block
/// JsCallExpression {
///     callee: JsStaticMember {
///         object: JsCallExpression {
///             callee: Reference {
///                 execute
///             }
///         }
///     }
/// }
/// ```
///
/// When we track the first [rome_js_syntax::JsCallExpression], we hold basically all the children,
/// that applies for the rest of the nodes. If we decided to format all the children of each node,
/// we will risk to format the last node, the `Reference`, four times.
///
/// To avoid this, when we encounter particular nodes, we don't format all of its children, but defer
/// the formatting to the child itself.
///
/// The end result of the flattening, will create an array of something like this:
///
/// ```block
/// [ Identifier, JsCallExpression, JsStaticMember, JsCallExpression ]
/// ```
///
/// ### Grouping
///
/// After the flattening, we start the grouping. We want to group nodes in a way that will help us
/// to apply a deterministic formatting.
/// - first group will be the identifier
/// - the rest of the groups will be  will start StaticMemberExpression followed by the rest of the nodes,
/// right before the end of the next StaticMemberExpression
///
/// The first group is special, because it holds the reference; it has its own heuristic.
/// Inside the first group we store the first element of the flattened array, then:
///
/// 1. as many as [rome_js_syntax::JsCallExpression] we can find, this cover cases like
/// `something()()().then()`;
/// 2. as many as [rome_js_syntax::JsComputedMemberExpression] we can find, this cover cases like
/// `something()()[1][3].then()`;
/// 3. as many as consecutive [rome_js_syntax::JsStaticMemberExpression] or [rome_js_syntax::JsComputedExpression], this cover cases like
/// `this.items[0].then()`
///
/// The rest of the groups are essentially a sequence of `[StaticMemberExpression , CallExpression]`.
/// In order to achieve that, we simply start looping through the rest of the flatten items that we haven't seen.
///
/// Eventually, we should have something like this:
///
/// ```block
/// [
///     [ReferenceIdentifier, CallExpression], // with possible computed expressions in the middle
///     [StaticMemberExpression, StaticMemberExpression, CallExpression],
///     [StaticMemberExpression, CallExpression],
///     [StaticMemberExpression],
/// ]
/// ```
///
/// [Prettier applies]: https://github.com/prettier/prettier/blob/main/src/language-js/print/member-chain.js
pub fn format_call_expression(syntax_node: &JsSyntaxNode, f: &mut JsFormatter) -> FormatResult<()> {
    let (calls_count, head_group, rest_of_groups) = get_call_expression_groups(syntax_node, f)?;
    write_groups(calls_count, head_group, rest_of_groups, f)
}

fn get_call_expression_groups(
    syntax_node: &JsSyntaxNode,
    f: &mut JsFormatter,
) -> SyntaxResult<(usize, HeadGroup, Groups)> {
    let mut flattened_items = vec![];
    let parent_is_expression_statement = syntax_node.parent().map_or(false, |parent| {
        JsExpressionStatement::can_cast(parent.kind())
    });

    flatten_call_expression(&mut flattened_items, syntax_node, &f.context().comments())?;

    // Count the number of CallExpression in the chain,
    // will be used later to decide on how to format it
    let calls_count = flattened_items
        .iter()
        .filter(|item| item.is_loose_call_expression())
        .count();

    // as explained before, the first group is particular, so we calculate it
    let index_to_split_at = compute_first_group_index(&flattened_items);

    // we have the index where we want to take the first group
    let remaining_groups = flattened_items.split_off(index_to_split_at);
    let first_group = flattened_items;

    let mut head_group = HeadGroup::new(first_group);

    // `flattened_items` now contains only the nodes that should have a sequence of
    // `[ StaticMemberExpression -> AnyNode + JsCallExpression ]`
    let mut rest_of_groups = compute_groups(
        remaining_groups.into_iter(),
        parent_is_expression_statement,
        f,
    );

    // Here we check if the first element of Groups::groups can be moved inside the head.
    // If so, then we extract it and concatenate it together with the head.
    if let Some(group_to_merge) = rest_of_groups.should_merge_with_first_group(&head_group) {
        let group_to_merge = group_to_merge.into_iter().flatten().collect();
        head_group.expand_group(group_to_merge);
    }

    Ok((calls_count, head_group, rest_of_groups))
}

/// Retrieves the index where we want to calculate the first group.
/// The first group gathers inside it all those nodes that are not a sequence of something like:
/// `[ StaticMemberExpression -> AnyNode + JsCallExpression ]`
fn compute_first_group_index(flatten_items: &[FlattenItem]) -> usize {
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
                FlattenItem::CallExpression(_) | FlattenItem::ComputedMember(_) => true,

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
                FlattenItem::StaticMember(_) => {
                    let next_flatten_item = &flatten_items[index + 1];
                    matches!(
                        next_flatten_item,
                        FlattenItem::StaticMember(_) | FlattenItem::ComputedMember(_)
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
    flatten_items: impl Iterator<Item = FlattenItem>,
    in_expression_statement: bool,
    f: &JsFormatter,
) -> Groups {
    let mut has_seen_call_expression = false;
    let mut groups = Groups::new(in_expression_statement, f.context().tab_width());
    for item in flatten_items {
        let has_trailing_comments = item.as_syntax().has_trailing_comments();

        match item {
            FlattenItem::StaticMember(_) => {
                // if we have seen a JsCallExpression, we want to close the group.
                // The resultant group will be something like: [ . , then, () ];
                // `.` and `then` belong to the previous StaticMemberExpression,
                // and `()` belong to the call expression we just encountered

                if has_seen_call_expression {
                    groups.close_group();
                    groups.start_or_continue_group(item);
                    has_seen_call_expression = false;
                } else {
                    groups.start_or_continue_group(item);
                }
            }
            FlattenItem::CallExpression(_) => {
                let is_loose_call_expression = item.is_loose_call_expression();
                groups.start_or_continue_group(item);
                if is_loose_call_expression {
                    has_seen_call_expression = true;
                }
            }
            FlattenItem::ComputedMember(_) => {
                groups.start_or_continue_group(item);
            }
            FlattenItem::Node(_) => groups.continue_group(item),
        }

        // Close the group immediately if the node had any trailing comments to
        // ensure those are printed in a trailing position for the token they
        // were originally commenting
        if has_trailing_comments {
            groups.close_group();
        }
    }

    // closing possible loose groups
    groups.close_group();

    groups
}

/// Formats together the first group and the rest of groups
fn write_groups(
    calls_count: usize,
    head_group: HeadGroup,
    groups: Groups,
    f: &mut JsFormatter,
) -> FormatResult<()> {
    // TODO use Alternatives once available
    write!(f, [head_group])?;

    if groups.groups_should_break(calls_count, &head_group)? {
        write!(
            f,
            [indent(&format_args!(
                hard_line_break(),
                format_with(|f| { groups.write_joined_with_hard_line_breaks(f) })
            ))]
        )
    } else {
        // TODO This line suffix boundary shouldn't be needed but currently is because comments
        // can move over node boundaries. Follow up when re-working member chain formatting
        let mut buffer = PreambleBuffer::new(f, line_suffix_boundary());

        write!(buffer, [format_with(|f| { groups.write(f) })])
    }
}

/// This function tries to flatten the AST. It stores nodes and its formatted version
/// inside an vector of [FlattenItem]. The first element of the vector is the last one.
fn flatten_call_expression(
    queue: &mut Vec<FlattenItem>,
    node: &JsSyntaxNode,
    comments: &Comments<JsLanguage>,
) -> SyntaxResult<()> {
    if comments.is_suppressed(node) {
        queue.push(FlattenItem::Node(node.clone()))
    }

    match node.kind() {
        JsSyntaxKind::JS_CALL_EXPRESSION => {
            let call_expression = JsCallExpression::cast(node.clone()).unwrap();
            let callee = call_expression.callee()?;
            flatten_call_expression(queue, callee.syntax(), comments)?;

            queue.push(FlattenItem::CallExpression(call_expression));
        }
        JsSyntaxKind::JS_STATIC_MEMBER_EXPRESSION => {
            let static_member = JsStaticMemberExpression::cast(node.clone()).unwrap();
            let object = static_member.object()?;
            flatten_call_expression(queue, object.syntax(), comments)?;

            queue.push(FlattenItem::StaticMember(static_member));
        }

        JsSyntaxKind::JS_COMPUTED_MEMBER_EXPRESSION => {
            let computed_expression = JsComputedMemberExpression::cast(node.clone()).unwrap();
            let object = computed_expression.object()?;
            flatten_call_expression(queue, object.syntax(), comments)?;

            queue.push(FlattenItem::ComputedMember(computed_expression));
        }

        _ => {
            queue.push(FlattenItem::Node(node.clone()));
        }
    }

    Ok(())
}

/// Here we check if the length of the groups exceeds the cutoff or there are comments
/// This function is the inverse of the prettier function
/// [Prettier applies]: https://github.com/prettier/prettier/blob/a043ac0d733c4d53f980aa73807a63fc914f23bd/src/language-js/print/member-chain.js#L342
pub fn is_member_call_chain(
    expression: &JsCallExpression,
    f: &mut JsFormatter,
) -> SyntaxResult<bool> {
    let (_, _, groups) = get_call_expression_groups(expression.syntax(), f)?;
    groups.is_member_call_chain()
}
