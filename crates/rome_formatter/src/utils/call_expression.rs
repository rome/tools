use crate::formatter_traits::{FormatOptionalTokenAndNode, FormatTokenAndNode};
use crate::{
    concat_elements, format_elements, hard_line_break, indent, join_elements, soft_line_break,
    FormatElement, FormatResult, Formatter, ToFormatElement,
};
use rslint_parser::ast::{
    JsCallExpression, JsComputedMemberExpression, JsImportCallExpression, JsStaticMemberExpression,
};
use rslint_parser::{AstNode, JsSyntaxKind, SyntaxNode};
use std::fmt::Debug;

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
/// When we track the first [rslint_parser::ast::JsCallExpression], we hold basically all the children,
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
/// 1. as many as [rslint_parser::ast::JsCallExpression] we can find, this cover cases like
/// `something()()().then()`;
/// 2. as many as [rslint_parser::ast::JsComputedMemberExpression] we can find, this cover cases like
/// `something()()[1][3].then()`;
/// 3. as many as consecutive [rslint_parser::ast::JsStaticMemberExpression] or [rslint_parser::ast::JsComputedExpression], this cover cases like
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
    let mut flattened_expression = vec![];

    flatten_call_expression(&mut flattened_expression, syntax_node.to_owned(), formatter)?;

    let (first_group, rest_of_flatten_items) = compute_first_group(&flattened_expression)?;
    let rest_of_groups = compute_groups(rest_of_flatten_items.to_owned())?;

    Ok(format_groups(first_group, rest_of_groups))
}

/// Computes the first group by keeping track of the index where to split the given flatten_items
fn compute_first_group(
    flatten_items: &[FlattenItem],
) -> FormatResult<(&[FlattenItem], &[FlattenItem])> {
    let mut current_index = 0;
    dbg!(&flatten_items);

    // the first element will always be part of the first group, so we skip it
    for (index, item) in flatten_items.iter().enumerate().skip(1) {
        // This where we apply the first two points explained in the description of the main public function.
        // We want to keep iterating over the items until we have call expressions or computed expressions:
        // - `something()()()()`
        // - `something[1][2][4]`
        // - `something[1]()[3]()`
        // - `something()[2].something.else[0]`
        match item {
            FlattenItem::CallExpression(_, _) | FlattenItem::ComputedExpression(_, _) => {
                current_index = index;
            }

            FlattenItem::StaticMember(_, _) => {
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
                let next_flatten_item = &flatten_items[index + 1];
                if matches!(
                    next_flatten_item,
                    FlattenItem::StaticMember(_, _) | FlattenItem::ComputedExpression(_, _)
                ) {
                    current_index = index;
                } else {
                    break;
                }
            }
            _ => break,
        }
    }

    Ok(flatten_items.split_at(current_index + 1))
}

/// computes groups coming after the first group
fn compute_groups(flatten_items: Vec<FlattenItem>) -> FormatResult<Groups> {
    let mut has_seen_call_expression = false;
    let mut groups = Groups::default();
    for item in flatten_items {
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
    }
    // closing possible loose groups
    groups.close_group();

    Ok(groups)
}

#[derive(Clone)]
/// Data structure that holds the node with its formatted version
pub(crate) enum FlattenItem {
    /// Holds onto a [rslint_parser::ast::JsStaticMemberExpression]
    StaticMember(JsStaticMemberExpression, Vec<FormatElement>),
    /// Holds onto a [rslint_parser::ast::JsCallExpression]
    CallExpression(JsCallExpression, Vec<FormatElement>),
    /// Holds onto a [rslint_parser::ast::JsComputedMemberExpression]
    ComputedExpression(JsComputedMemberExpression, Vec<FormatElement>),
    /// Any other node that are not  [rslint_parser::ast::JsCallExpression] or [rslint_parser::ast::JsStaticMemberExpression]
    /// Are tracked using this variant
    Node(SyntaxNode, FormatElement),
}

impl FlattenItem {
    /// checks if the current node is a [rslint_parser::ast::JsCallExpression] or a [rslint_parser::ast::JsImportExpression]
    pub fn is_loose_call_expression(&self) -> bool {
        match self {
            FlattenItem::CallExpression(_, _) => true,
            FlattenItem::Node(node, _) => JsImportCallExpression::can_cast(node.kind()),
            _ => false,
        }
    }

    /// It strictly checks if the current item is a [rslint_parser::ast::JsCallExpression]
    pub fn is_call_expression(&self) -> bool {
        matches!(&self, FlattenItem::CallExpression(_, _))
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
    pub fn groups_should_break(&self) -> bool {
        // TODO: this should have more checks
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

/// Formats together the first group and the rest of groups
fn format_groups(first_group: &[FlattenItem], groups: Groups) -> FormatElement {
    let first_formatted_group: Vec<FormatElement> = first_group
        .iter()
        .map(|flatten_item| flatten_item.clone().into())
        .collect();

    let formatted_groups = if groups.groups_should_break() {
        indent(format_elements![
            hard_line_break(),
            groups.into_joined_groups()
        ])
    } else {
        groups.into_concatenated_groups()
    };

    format_elements![concat_elements(first_formatted_group), formatted_groups]
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
            flatten_call_expression(queue, callee.syntax().to_owned(), formatter)?;
            let formatted = vec![
                call_expression
                    .optional_chain_token_token()
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
            flatten_call_expression(queue, object.syntax().to_owned(), formatter)?;
            let formatted = vec![
                static_member.operator().format(formatter)?,
                static_member.member().format(formatter)?,
            ];
            queue.push(FlattenItem::StaticMember(static_member, formatted));
        }

        JsSyntaxKind::JS_COMPUTED_MEMBER_EXPRESSION => {
            let computed_expression = JsComputedMemberExpression::cast(node).unwrap();
            let object = computed_expression.object()?;
            flatten_call_expression(queue, object.syntax().to_owned(), formatter)?;
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
