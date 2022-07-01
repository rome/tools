use crate::context::TabWidth;
use crate::prelude::*;
use crate::utils::member_chain::flatten_item::FlattenItem;
use crate::utils::member_chain::simple_argument::SimpleArgument;
use rome_js_syntax::JsCallExpression;
use rome_rowan::{AstSeparatedList, SyntaxResult};
use std::mem;

#[derive(Clone)]
/// Handles creation of groups while scanning the flatten items
pub(crate) struct Groups {
    /// If the current group is inside an expression statement.
    ///
    /// This information is important when evaluating the break of the groups.
    in_expression_statement: bool,
    /// keeps track of the groups created
    groups: Vec<Vec<FlattenItem>>,
    /// keeps track of the current group that is being created/updated
    current_group: Vec<FlattenItem>,

    /// This is a threshold of when we should start breaking the groups
    ///
    /// By default, it's 2, meaning that we start breaking after the second group.
    cutoff: u8,

    tab_width: TabWidth,
}

impl Groups {
    pub fn new(in_expression_statement: bool, tab_width: TabWidth) -> Self {
        Self {
            in_expression_statement,
            groups: Vec::new(),
            current_group: Vec::new(),
            cutoff: 2,
            tab_width,
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
    ) -> FormatResult<bool> {
        // Do not allow the group to break if it only contains a single call expression
        if calls_count <= 1 {
            return Ok(false);
        }

        // we want to check the simplicity of the call expressions only if we have at least
        // two of them
        // Check prettier: https://github.com/prettier/prettier/blob/main/src/language-js/print/member-chain.js#L389
        let call_expressions_are_not_simple =
            calls_count > 2 && self.call_expressions_are_not_simple()?;

        // TODO: add here will_break logic

        let node_has_comments = self.has_comments()? || head_group.has_comments();

        let should_break = node_has_comments || call_expressions_are_not_simple;

        Ok(should_break)
    }

    /// Checks if the groups contain comments.
    pub fn has_comments(&self) -> SyntaxResult<bool> {
        let mut has_leading_comments = false;

        let flat_groups = self.groups.iter().flat_map(|item| item.iter());
        for item in flat_groups {
            if item.has_leading_comments()? {
                has_leading_comments = true;
                break;
            }
        }

        let has_trailing_comments = self
            .groups
            .iter()
            .flat_map(|item| item.iter())
            .any(|item| item.has_trailing_comments());

        let cutoff_has_leading_comments = if self.groups.len() >= self.cutoff as usize {
            let group = self.groups.get(self.cutoff as usize);
            if let Some(group) = group {
                let first_item = group.first();
                if let Some(first_item) = first_item {
                    first_item.has_leading_comments()?
                } else {
                    false
                }
            } else {
                false
            }
        } else {
            false
        };

        Ok(has_leading_comments || has_trailing_comments || cutoff_has_leading_comments)
    }

    /// Format groups on multiple lines
    pub fn write_joined_with_hard_line_breaks(&self, f: &mut JsFormatter) -> FormatResult<()> {
        f.join_with(hard_line_break())
            .entries(
                self.groups
                    .iter()
                    .map(|group| format_with(|f| f.join().entries(group.iter()).finish())),
            )
            .finish()
    }

    /// Creates two different versions of the formatted groups, one that goes in one line
    /// and the other one that goes on multiple lines.
    ///
    /// It's up to the printer to decide which one to use.
    pub fn write(&self, f: &mut JsFormatter) -> FormatResult<()> {
        if self.groups.is_empty() {
            return Ok(());
        }

        f.join()
            .entries(self.groups.iter().flat_map(|group| group.iter()))
            .finish()
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

    /// This is an heuristic needed to check when the first element of the group
    /// Should be part of the "head" or the "tail".
    fn should_not_wrap(&self, first_group: &HeadGroup) -> SyntaxResult<bool> {
        let tab_with = self.tab_width;
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
    pub(crate) fn should_merge_with_first_group(
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

#[derive(Debug)]
pub(crate) struct HeadGroup {
    items: Vec<FlattenItem>,
}

impl HeadGroup {
    pub(crate) fn new(items: Vec<FlattenItem>) -> Self {
        Self { items }
    }

    fn items(&self) -> &[FlattenItem] {
        &self.items
    }

    pub fn expand_group(&mut self, group: Vec<FlattenItem>) {
        self.items.extend(group)
    }

    fn has_comments(&self) -> bool {
        self.items.iter().any(|item| item.has_trailing_comments())
    }
}

impl Format<JsFormatContext> for HeadGroup {
    fn fmt(&self, f: &mut JsFormatter) -> FormatResult<()> {
        f.join().entries(self.items.iter()).finish()
    }
}
