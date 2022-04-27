use crate::format_traits::FormatOptional;
use crate::utils::SimpleArgument;
use crate::{
    concat_elements, format_elements, group_elements, hard_line_break, indent, join_elements,
    soft_line_break, Format, FormatElement, FormatResult, Formatter,
};
use rome_js_syntax::{
    JsAnyCallArgument, JsAnyExpression, JsCallExpression, JsComputedMemberExpression,
    JsExpressionStatement, JsIdentifierExpression, JsImportCallExpression, JsNewExpression,
    JsStaticMemberExpression, JsThisExpression,
};
use rome_js_syntax::{JsSyntaxKind, JsSyntaxNode};
use rome_rowan::{AstNode, AstSeparatedList, SyntaxResult};
use std::fmt::Debug;
use std::{mem, slice};

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
    syntax_node: &JsSyntaxNode,
    formatter: &Formatter,
) -> FormatResult<FormatElement> {
    let mut flattened_items = vec![];
    let parent_is_expression_statement = syntax_node.parent().map_or(false, |parent| {
        JsExpressionStatement::can_cast(parent.kind())
    });

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
    let first_group: Vec<_> = (&mut flattened_items).take(index_to_split_at).collect();

    let mut head_group = HeadGroup::new(first_group);

    // `flattened_items` now contains only the nodes that should have a sequence of
    // `[ StaticMemberExpression -> AnyNode + JsCallExpression ]`
    let mut rest_of_groups =
        compute_groups(flattened_items, parent_is_expression_statement, formatter)?;

    // Here we check if the first element of Groups::groups can be moved inside the head.
    // If so, then we extract it and concatenate it together with the head.
    if let Some(group_to_merge) = rest_of_groups.should_merge_with_first_group(&head_group) {
        let group_to_merge = group_to_merge.into_iter().flatten().collect();
        head_group.expand_group(group_to_merge);
    }

    format_groups(calls_count, head_group, rest_of_groups)
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
fn compute_groups(
    flatten_items: impl Iterator<Item = FlattenItem>,
    in_expression_statement: bool,
    formatter: &Formatter,
) -> FormatResult<Groups> {
    let mut has_seen_call_expression = false;
    let mut groups = Groups::new(formatter, in_expression_statement);
    for item in flatten_items {
        let has_trailing_comments = item.as_syntax().has_trailing_comments();

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
    head_group: HeadGroup,
    groups: Groups,
) -> FormatResult<FormatElement> {
    if groups.groups_should_break(calls_count, &head_group)? {
        Ok(format_elements![
            head_group.into_format_element(),
            group_elements(indent(format_elements![
                hard_line_break(),
                groups.into_joined_hard_line_groups()
            ]),)
        ])
    } else {
        let head_formatted = head_group.into_format_element();
        let (one_line, _) = groups.into_format_elements();

        // TODO: this is not the definitive solution, as there are few restrictions due to how the printer works:
        // - groups that contains other groups with hard lines break all the groups
        // - conditionally print one single line is subject to how the printer works (by default, multiline)
        Ok(format_elements![head_formatted, one_line])
    }
}

/// This function tries to flatten the AST. It stores nodes and its formatted version
/// inside an vector of [FlattenItem]. The first element of the vector is the last one.
fn flatten_call_expression(
    queue: &mut Vec<FlattenItem>,
    node: JsSyntaxNode,
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
                static_member.operator_token().format(formatter)?,
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
            let formatted = node.format(formatter)?;
            queue.push(FlattenItem::Node(node, formatted));
        }
    }

    Ok(())
}

#[derive(Debug)]
struct HeadGroup {
    items: Vec<FlattenItem>,
}

impl HeadGroup {
    fn new(items: Vec<FlattenItem>) -> Self {
        Self { items }
    }

    fn items(&self) -> &[FlattenItem] {
        &self.items
    }

    fn into_format_element(self) -> FormatElement {
        concat_elements(self.items.into_iter().map(FlattenItem::into))
    }

    fn expand_group(&mut self, group: Vec<FlattenItem>) {
        self.items.extend(group)
    }

    fn has_comments(&self) -> bool {
        self.items.iter().any(|item| item.has_trailing_comments())
    }
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
    Node(JsSyntaxNode, FormatElement),
}

impl FlattenItem {
    /// checks if the current node is a [rome_js_syntax::JsCallExpression],  [rome_js_syntax::JsImportExpression] or a [rome_js_syntax::JsNewExpression]
    pub fn is_loose_call_expression(&self) -> bool {
        match self {
            FlattenItem::CallExpression(_, _) => true,
            FlattenItem::Node(node, _) => {
                JsImportCallExpression::can_cast(node.kind())
                    | JsNewExpression::can_cast(node.kind())
            }
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

    fn as_syntax(&self) -> &JsSyntaxNode {
        match self {
            FlattenItem::StaticMember(node, _) => node.syntax(),
            FlattenItem::CallExpression(node, _) => node.syntax(),
            FlattenItem::ComputedExpression(node, _) => node.syntax(),
            FlattenItem::Node(node, _) => node,
        }
    }

    fn has_leading_comments(&self) -> bool {
        match self {
            FlattenItem::StaticMember(node, _) => node.syntax().has_leading_comments(),
            FlattenItem::CallExpression(node, _) => node.syntax().has_leading_comments(),
            FlattenItem::ComputedExpression(node, _) => node.syntax().has_leading_comments(),
            FlattenItem::Node(node, _) => node.has_leading_comments(),
        }
    }

    fn has_trailing_comments(&self) -> bool {
        match self {
            FlattenItem::StaticMember(node, _) => node.syntax().has_trailing_comments(),
            FlattenItem::CallExpression(node, _) => node.syntax().has_trailing_comments(),
            FlattenItem::ComputedExpression(node, _) => node.syntax().has_trailing_comments(),
            FlattenItem::Node(node, _) => node.has_trailing_comments(),
        }
    }

    fn is_computed_expression(&self) -> bool {
        matches!(self, FlattenItem::ComputedExpression(..))
    }

    fn is_this_expression(&self) -> bool {
        match self {
            FlattenItem::Node(node, _) => JsThisExpression::can_cast(node.kind()),
            _ => false,
        }
    }

    fn is_identifier_expression(&self) -> bool {
        match self {
            FlattenItem::Node(node, _) => JsIdentifierExpression::can_cast(node.kind()),
            _ => false,
        }
    }

    /// There are cases like Object.keys(), Observable.of(), _.values() where
    /// they are the subject of all the chained calls and therefore should
    /// be kept on the same line:
    ///
    /// ```js
    ///   Object.keys(items)
    ///     .filter(x => x)
    ///     .map(x => x)
    /// ```
    /// In order to detect those cases, we use an heuristic: if the first
    /// node is an identifier with the name starting with a capital
    /// letter or just a sequence of _$. The rationale is that they are
    /// likely to be factories.
    ///
    /// Comment from [Prettier]
    ///
    /// [Prettier]: https://github.com/prettier/prettier/blob/main/src/language-js/print/member-chain.js#L252-L266
    fn is_factory(&self, check_left_hand_side: bool) -> SyntaxResult<bool> {
        fn check_str(text: &str) -> bool {
            text.chars().next().map_or(false, |c| c.is_uppercase())
                || text.starts_with('_')
                || text.starts_with('$')
        }

        if let FlattenItem::StaticMember(static_member, ..) = self {
            if check_left_hand_side {
                if let JsAnyExpression::JsIdentifierExpression(identifier_expression) =
                    static_member.object()?
                {
                    let value_token = identifier_expression.name()?.value_token()?;
                    let text = value_token.text_trimmed();
                    Ok(check_str(text))
                } else {
                    Ok(false)
                }
            } else {
                Ok(check_str(static_member.member()?.text().as_str()))
            }
        } else if let FlattenItem::Node(node, ..) = self {
            if let Some(identifier_expression) = JsIdentifierExpression::cast(node.clone()) {
                let value_token = identifier_expression.name()?.value_token()?;
                let text = value_token.text_trimmed();
                Ok(check_str(text))
            } else {
                Ok(false)
            }
        } else {
            Ok(false)
        }
    }

    fn has_short_name(&self, tab_width: u8) -> SyntaxResult<bool> {
        if let FlattenItem::StaticMember(static_member, ..) = self {
            if let JsAnyExpression::JsIdentifierExpression(identifier_expression) =
                static_member.object()?
            {
                let value_token = identifier_expression.name()?.value_token()?;
                let text = value_token.text_trimmed();
                Ok(text.len() <= tab_width as usize)
            } else {
                Ok(false)
            }
        } else {
            Ok(false)
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
            FlattenItem::Node(node, formatted) => write!(f, "{:?} {:?}", node.kind(), formatted),
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

#[derive(Clone)]
/// Handles creation of groups while scanning the flatten items
struct Groups<'f> {
    /// If the current group is inside an expression statement.
    ///
    /// This information is important when evaluating the break of the groups.
    in_expression_statement: bool,
    /// keeps track of the groups created
    groups: Vec<Vec<FlattenItem>>,
    /// keeps track of the current group that is being created/updated
    current_group: Vec<FlattenItem>,

    /// instance of the formatter
    formatter: &'f Formatter,

    /// This is a threshold of when we should start breaking the groups
    ///
    /// By default, it's 2, meaning that we start breaking after the second group.
    cutoff: u8,
}

impl<'f> Groups<'f> {
    pub fn new(formatter: &'f Formatter, in_expression_statement: bool) -> Self {
        Self {
            formatter,
            in_expression_statement,
            groups: Vec::new(),
            current_group: Vec::new(),
            cutoff: 2,
        }
    }

    /// This function checks if the current grouping should be merged with the first group.
    pub fn should_merge(&self, head_group: &HeadGroup) -> SyntaxResult<bool> {
        Ok(!self.groups.len() >= 1
            && self.should_not_wrap(head_group)?
            && !self.groups[0]
                .first()
                .map_or(false, |item| item.has_trailing_comments()))
    }

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
    pub fn groups_should_break(
        &self,
        calls_count: usize,
        head_group: &HeadGroup,
    ) -> SyntaxResult<bool> {
        // Do not allow the group to break if it only contains a single call expression
        if calls_count <= 1 {
            return Ok(false);
        }

        let node_has_comments = self.has_comments() || head_group.has_comments();
        // we want to check the simplicity of the call expressions only if we have at least
        // two of them
        // Check prettier: https://github.com/prettier/prettier/blob/main/src/language-js/print/member-chain.js#L389
        let call_expressions_are_not_simple = if calls_count > 2 {
            self.call_expressions_are_not_simple()?
        } else {
            false
        };
        let last_group_will_break_and_other_calls_have_function_arguments =
            self.last_group_will_break_and_other_calls_have_function_arguments()?;

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

        let should_break = has_line_breaks
            || node_has_comments
            || call_expressions_are_not_simple
            || last_group_will_break_and_other_calls_have_function_arguments;

        Ok(should_break)
    }

    fn into_formatted_groups(self) -> Vec<FormatElement> {
        self.groups
            .into_iter()
            .map(|group| concat_elements(group.into_iter().map(|flatten_item| flatten_item.into())))
            .collect()
    }

    /// Format groups on multiple lines
    pub fn into_joined_hard_line_groups(self) -> FormatElement {
        let formatted_groups = self.into_formatted_groups();
        join_elements(hard_line_break(), formatted_groups)
    }

    /// Creates two different versions of the formatted groups, one that goes in one line
    /// and the other one that goes on multiple lines.
    ///
    /// It's up to the printer to decide which one to use.
    pub fn into_format_elements(self) -> (FormatElement, FormatElement) {
        let formatted_groups = self.into_formatted_groups();
        (
            concat_elements(formatted_groups.clone()),
            join_elements(soft_line_break(), formatted_groups),
        )
    }

    /// Checks if the groups contain comments.
    pub fn has_comments(&self) -> bool {
        let has_leading_comments = if self.groups.len() > 1 {
            // SAFETY: access guarded by the previous check
            self.groups
                .iter()
                .flat_map(|item| item.iter())
                .skip(1)
                .any(|item| item.has_leading_comments())
        } else {
            false
        };
        let has_trailing_comments = self
            .groups
            .iter()
            .flat_map(|item| item.iter())
            .any(|item| item.has_trailing_comments());

        // This check might not be needed... trying to understand why Prettier has it
        let cutoff_has_leading_comments = if self.groups.len() >= self.cutoff as usize {
            self.groups
                .get(self.cutoff as usize)
                .map_or(false, |group| {
                    group
                        .first()
                        .map_or(false, |group| group.has_leading_comments())
                })
        } else {
            false
        };

        has_leading_comments || has_trailing_comments || cutoff_has_leading_comments
    }

    /// Filters the stack of [FlattenItem] and return only the ones that
    /// contain [JsCallExpression]. The function returns the actual nodes.
    pub fn get_call_expressions(&self) -> impl Iterator<Item = &JsCallExpression> {
        self.groups
            .iter()
            .flat_map(|group| group.iter())
            .filter_map(|item| {
                if let FlattenItem::CallExpression(call_expression, ..) = item {
                    Some(call_expression)
                } else {
                    None
                }
            })
    }

    /// We retrieve all the call expressions inside the group and we check if
    /// their arguments are not simple.
    pub fn call_expressions_are_not_simple(&self) -> SyntaxResult<bool> {
        Ok(self.get_call_expressions().any(|call_expression| {
            call_expression.arguments().map_or(false, |arguments| {
                !arguments
                    .args()
                    .iter()
                    .filter_map(|argument| argument.ok())
                    .all(|argument| SimpleArgument::new(argument).is_simple(0))
            })
        }))
    }

    /// Checks if the last group will break - by emulating the behaviour of the printer,
    /// or if there's a call expression that contain a function/arrow function as argument
    pub fn last_group_will_break_and_other_calls_have_function_arguments(
        &self,
    ) -> SyntaxResult<bool> {
        let last_group = self.groups.iter().flat_map(|group| group.iter()).last();

        if let Some(last_group) = last_group {
            let element = last_group.as_format_elements().last();
            let group_will_break = element.map_or(false, |element| element.has_hard_line_breaks());

            let is_call_expression = last_group.is_loose_call_expression();

            Ok(group_will_break
                && is_call_expression
                && self.call_expressions_have_function_or_arrow_func_as_argument()?)
        } else {
            Ok(false)
        }
    }

    /// Checks if any of the call expressions contains arguments that are functions or arrow
    /// functions.
    pub fn call_expressions_have_function_or_arrow_func_as_argument(&self) -> SyntaxResult<bool> {
        for call_expression in self.get_call_expressions() {
            let arguments = call_expression.arguments()?;
            for argument in arguments.args() {
                if matches!(
                    argument?,
                    JsAnyCallArgument::JsAnyExpression(JsAnyExpression::JsFunctionExpression(_))
                        | JsAnyCallArgument::JsAnyExpression(
                            JsAnyExpression::JsArrowFunctionExpression(_)
                        )
                ) {
                    return Ok(true);
                }
            }
        }

        Ok(false)
    }

    /// This is an heuristic needed to check when the first element of the group
    /// Should be part of the "head" or the "tail".
    fn should_not_wrap(&self, first_group: &HeadGroup) -> SyntaxResult<bool> {
        let tab_with = self.formatter.options().tab_width();
        let has_computed_property = if self.groups.len() > 1 {
            // SAFETY: guarded by the previous check
            let group = &self.groups[0];
            group
                .first()
                .map_or(false, |item| item.is_computed_expression())
        } else {
            false
        };

        if first_group.items.len() == 1 {
            // SAFETY: access is guarded by the previous check
            let first_node = first_group.items().first().unwrap();

            return Ok(first_node.is_this_expression()
                || (first_node.is_identifier_expression()
                    && (first_node.is_factory(true)?
                // If an identifier has a name that is shorter than the tab with, then we join it with the "head"
                || (self.in_expression_statement
                && first_node.has_short_name(tab_with)?)
                || has_computed_property)));
        }

        let last_node_is_factory = self
            .groups
            .iter()
            .flat_map(|group| group.iter())
            .last()
            .map_or(false, |item| item.is_factory(false).unwrap_or(false));

        Ok(last_node_is_factory || has_computed_property)
    }

    /// Here we check if the first group can be merged to the head. If so, then
    /// we move out the first group out of the groups
    fn should_merge_with_first_group(
        &mut self,
        head_group: &HeadGroup,
    ) -> Option<Vec<Vec<FlattenItem>>> {
        if self.should_merge(head_group).unwrap_or(false) {
            // While we are at it, we also update the the cutoff.
            // If we should merge the groups, it means that also the cutoff has to be increased by one
            self.cutoff = 3;
            let mut new_groups = self.groups.split_off(1);
            // self.groups is now the head (one element), while `new_groups` is a new vector without the
            // first element.
            // As we need to achieve the opposite, we now swap them.
            mem::swap(&mut self.groups, &mut new_groups);
            Some(new_groups)
        } else {
            None
        }
    }
}
