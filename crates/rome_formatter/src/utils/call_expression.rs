use crate::formatter_traits::{FormatOptionalTokenAndNode, FormatTokenAndNode};
use crate::{
    concat_elements, format_elements, group_elements, indent, join_elements, soft_line_break,
    FormatElement, FormatResult, Formatter, ToFormatElement,
};
use rome_js_syntax::{AstNode, JsSyntaxKind, SyntaxNode, SyntaxNodeExt};
use rome_js_syntax::{
    JsCallExpression, JsComputedMemberExpression, JsImportCallExpression, JsStaticMemberExpression,
};
use std::fmt::Debug;
use std::slice;

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
pub fn format_call_expression(
    syntax_node: &SyntaxNode,
    formatter: &Formatter,
) -> FormatResult<FormatElement> {
    let mut flattened_items = vec![];

    flatten_call_expression(&mut flattened_items, syntax_node.clone(), formatter)?;

    // Count the number of CallExpression in the chain,
    // will be used later to decide on how to format it
    let calls_count = flattened_items
        .iter()
        .filter(|item| item.is_loose_call_expression())
        .count();

    // as explained before, the first group is particular, so we calculate it
    let index_to_split_at = compute_first_group_index(&flattened_items);
    let mut flattened_items = flattened_items.into_iter();
    // we have the index where we want to take the first group
    let first_group = concat_elements(
        (&mut flattened_items)
            .take(index_to_split_at)
            .map(FlattenItem::into),
    );
    // `flattened_items` now contains only the nodes that should have a sequence of
    // `[ StaticMemberExpression -> AnyNode + JsCallExpression ]`
    let rest_of_groups = compute_groups(flattened_items)?;
    Ok(format_groups(calls_count, first_group, rest_of_groups))
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
                FlattenItem::CallExpression(_, _) | FlattenItem::ComputedExpression(_, _) => true,

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
                FlattenItem::StaticMember(_, _) => {
                    let next_flatten_item = &flatten_items[index + 1];
                    matches!(
                        next_flatten_item,
                        FlattenItem::StaticMember(_, _) | FlattenItem::ComputedExpression(_, _)
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
fn compute_groups(flatten_items: impl Iterator<Item = FlattenItem>) -> FormatResult<Groups> {
    let mut has_seen_call_expression = false;
    let mut groups = Groups::default();
    for item in flatten_items {
        let has_trailing_comments = item.syntax().has_trailing_comments();

        match item {
            FlattenItem::StaticMember(_, _) => {
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
            FlattenItem::CallExpression(_, _) => {
                let is_loose_call_expression = item.is_loose_call_expression();
                groups.start_or_continue_group(item);
                if is_loose_call_expression {
                    has_seen_call_expression = true;
                }
            }
            FlattenItem::ComputedExpression(_, _) => {
                groups.start_or_continue_group(item);
            }
            FlattenItem::Node(_, _) => groups.continue_group(item),
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

    Ok(groups)
}

/// Formats together the first group and the rest of groups
fn format_groups(
    calls_count: usize,
    first_formatted_group: FormatElement,
    groups: Groups,
) -> FormatElement {
    if groups.groups_should_break(calls_count) {
        group_elements(format_elements![
            first_formatted_group,
            indent(format_elements![
                soft_line_break(),
                groups.into_joined_groups()
            ]),
        ])
    } else {
        format_elements![first_formatted_group, groups.into_concatenated_groups()]
    }
}

/// This function tries to flatten the AST. It stores nodes and its formatted version
/// inside an vector of [FlattenItem]. The first element of the vector is the last one.
fn flatten_call_expression(
    queue: &mut Vec<FlattenItem>,
    node: SyntaxNode,
    formatter: &Formatter,
) -> FormatResult<()> {
    match node.kind() {
        JsSyntaxKind::JS_CALL_EXPRESSION => {
            let call_expression = JsCallExpression::cast(node).unwrap();
            let callee = call_expression.callee()?;
            flatten_call_expression(queue, callee.syntax().clone(), formatter)?;
            let formatted = vec![
                call_expression
                    .optional_chain_token()
                    .format_or_empty(formatter)?,
                call_expression
                    .type_arguments()
                    .format_or_empty(formatter)?,
                call_expression.arguments().format(formatter)?,
            ];

            queue.push(FlattenItem::CallExpression(call_expression, formatted));
        }
        JsSyntaxKind::JS_STATIC_MEMBER_EXPRESSION => {
            let static_member = JsStaticMemberExpression::cast(node).unwrap();
            let object = static_member.object()?;
            flatten_call_expression(queue, object.syntax().clone(), formatter)?;
            let formatted = vec![
                static_member.operator().format(formatter)?,
                static_member.member().format(formatter)?,
            ];
            queue.push(FlattenItem::StaticMember(static_member, formatted));
        }

        JsSyntaxKind::JS_COMPUTED_MEMBER_EXPRESSION => {
            let computed_expression = JsComputedMemberExpression::cast(node).unwrap();
            let object = computed_expression.object()?;
            flatten_call_expression(queue, object.syntax().clone(), formatter)?;
            let formatted = vec![
                computed_expression
                    .optional_chain_token()
                    .format_or_empty(formatter)?,
                computed_expression.l_brack_token().format(formatter)?,
                computed_expression.member().format(formatter)?,
                computed_expression.r_brack_token().format(formatter)?,
            ];

            queue.push(FlattenItem::ComputedExpression(
                computed_expression,
                formatted,
            ));
        }

        _ => {
            let formatted = node.to_format_element(formatter)?;
            queue.push(FlattenItem::Node(node, formatted));
        }
    }

    Ok(())
}

#[derive(Clone)]
/// Data structure that holds the node with its formatted version
pub(crate) enum FlattenItem {
    /// Holds onto a [rome_js_syntax::JsStaticMemberExpression]
    StaticMember(JsStaticMemberExpression, Vec<FormatElement>),
    /// Holds onto a [rome_js_syntax::JsCallExpression]
    CallExpression(JsCallExpression, Vec<FormatElement>),
    /// Holds onto a [rome_js_syntax::JsComputedMemberExpression]
    ComputedExpression(JsComputedMemberExpression, Vec<FormatElement>),
    /// Any other node that are not  [rome_js_syntax::JsCallExpression] or [rome_js_syntax::JsStaticMemberExpression]
    /// Are tracked using this variant
    Node(SyntaxNode, FormatElement),
}

impl FlattenItem {
    /// checks if the current node is a [rome_js_syntax::JsCallExpression] or a [rome_js_syntax::JsImportExpression]
    pub fn is_loose_call_expression(&self) -> bool {
        match self {
            FlattenItem::CallExpression(_, _) => true,
            FlattenItem::Node(node, _) => JsImportCallExpression::can_cast(node.kind()),
            _ => false,
        }
    }

    fn as_format_elements(&self) -> &[FormatElement] {
        match self {
            FlattenItem::StaticMember(_, elements) => elements,
            FlattenItem::CallExpression(_, elements) => elements,
            FlattenItem::ComputedExpression(_, elements) => elements,
            FlattenItem::Node(_, element) => slice::from_ref(element),
        }
    }

    fn syntax(&self) -> &SyntaxNode {
        match self {
            FlattenItem::StaticMember(node, _) => node.syntax(),
            FlattenItem::CallExpression(node, _) => node.syntax(),
            FlattenItem::ComputedExpression(node, _) => node.syntax(),
            FlattenItem::Node(node, _) => node,
        }
    }
}

impl Debug for FlattenItem {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            FlattenItem::StaticMember(_, formatted) => write!(f, "StaticMember: {:?}", formatted),
            FlattenItem::CallExpression(_, formatted) => {
                write!(f, "CallExpression: {:?}", formatted)
            }
            FlattenItem::ComputedExpression(_, formatted) => {
                write!(f, "ComputedExpression: {:?}", formatted)
            }
            FlattenItem::Node(_, formatted) => write!(f, "{:?}", formatted),
        }
    }
}

impl From<FlattenItem> for FormatElement {
    fn from(flatten_item: FlattenItem) -> Self {
        match flatten_item {
            FlattenItem::StaticMember(_, formatted) => concat_elements(formatted),
            FlattenItem::CallExpression(_, formatted) => concat_elements(formatted),
            FlattenItem::ComputedExpression(_, formatted) => concat_elements(formatted),
            FlattenItem::Node(_, formatted) => formatted,
        }
    }
}

#[derive(Default, Clone)]
/// Handles creation of groups while scanning the flatten items
struct Groups {
    /// keeps track of the groups created
    groups: Vec<Vec<FlattenItem>>,
    /// keeps track of the current group that is being created/updated
    current_group: Vec<FlattenItem>,
}

impl Groups {
    /// starts a new group
    pub fn start_group<I: Into<FlattenItem>>(&mut self, flatten_item: I) {
        debug_assert!(self.current_group.is_empty());
        self.current_group.push(flatten_item.into());
    }

    /// continues of starts a new group
    pub fn start_or_continue_group<I: Into<FlattenItem>>(&mut self, flatten_item: I) {
        if self.current_group.is_empty() {
            self.start_group(flatten_item);
        } else {
            self.continue_group(flatten_item);
        }
    }

    /// adds the passed element to the current group
    pub fn continue_group<I: Into<FlattenItem>>(&mut self, flatten_item: I) {
        debug_assert!(!self.current_group.is_empty());
        self.current_group.push(flatten_item.into());
    }

    /// clears the current group, and adds a new group to the groups
    pub fn close_group(&mut self) {
        if !self.current_group.is_empty() {
            let mut elements = vec![];
            std::mem::swap(&mut elements, &mut self.current_group);
            self.groups.push(elements);
        }
    }

    /// It tells if the groups should be break on multiple lines
    pub fn groups_should_break(&self, calls_count: usize) -> bool {
        // Do not allow the group to break if it only contains a single call expression
        if calls_count <= 1 {
            return false;
        }

        // This emulates a simplified version of the similar logic found in the
        // printer to force groups to break if they contain any "hard line
        // break" (these not only include hard_line_break elements but also
        // empty_line or tokens containing the "\n" character): The idea is
        // that since any of these will force the group to break when it gets
        // printed, the formatter needs to emit a group element for the call
        // chain in the first place or it will not be printed correctly
        let has_line_breaks = self
            .groups
            .iter()
            .flat_map(|group| group.iter())
            .flat_map(|item| item.as_format_elements())
            .any(|element| element.has_hard_line_breaks());

        if has_line_breaks {
            return true;
        }

        // Otherwise, use a simple complexity threshold to
        // determine whether the group should be allow to break
        self.groups.len() > 3
    }

    fn into_formatted_groups(self) -> Vec<FormatElement> {
        self.groups
            .into_iter()
            .map(|group| concat_elements(group.into_iter().map(|flatten_item| flatten_item.into())))
            .collect()
    }

    /// Concatenate groups, without fancy formatting
    pub fn into_concatenated_groups(self) -> FormatElement {
        let formatted_groups = self.into_formatted_groups();
        concat_elements(formatted_groups)
    }

    /// Format groups on multiple lines
    pub fn into_joined_groups(self) -> FormatElement {
        let formatted_groups = self.into_formatted_groups();
        join_elements(soft_line_break(), formatted_groups)
    }
}
