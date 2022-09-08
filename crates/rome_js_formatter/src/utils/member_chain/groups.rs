use crate::context::TabWidth;
use crate::parentheses::NeedsParentheses;
use crate::prelude::*;
use crate::utils::member_chain::chain_member::ChainMember;
use rome_formatter::write;
use rome_js_syntax::JsCallExpression;
use rome_rowan::SyntaxResult;
use std::mem;

pub(super) struct MemberChainGroupsBuilder {
    /// keeps track of the groups created
    groups: Vec<MemberChainGroup>,
    /// keeps track of the current group that is being created/updated
    current_group: MemberChainGroup,

    /// If the current group is inside an expression statement.
    ///
    /// This information is important when evaluating the break of the groups.
    in_expression_statement: bool,

    tab_width: TabWidth,
}

impl MemberChainGroupsBuilder {
    pub fn new(in_expression_statement: bool, tab_width: TabWidth) -> Self {
        Self {
            in_expression_statement,
            groups: Vec::new(),
            current_group: MemberChainGroup::default(),
            tab_width,
        }
    }

    /// starts a new group
    pub fn start_group(&mut self, flatten_item: ChainMember) {
        debug_assert!(self.current_group.members.is_empty());
        self.current_group.members.push(flatten_item);
    }

    /// continues of starts a new group
    pub fn start_or_continue_group(&mut self, flatten_item: ChainMember) {
        if self.current_group.members.is_empty() {
            self.start_group(flatten_item);
        } else {
            self.continue_group(flatten_item);
        }
    }

    /// adds the passed element to the current group
    pub fn continue_group(&mut self, flatten_item: ChainMember) {
        debug_assert!(!self.current_group.members.is_empty());
        self.current_group.members.push(flatten_item);
    }

    /// clears the current group, and adds a new group to the groups
    pub fn close_group(&mut self) {
        if !self.current_group.members.is_empty() {
            let mut elements = MemberChainGroup::default();
            std::mem::swap(&mut elements, &mut self.current_group);
            self.groups.push(elements);
        }
    }

    pub(super) fn finish(self) -> MemberChainGroups {
        debug_assert!(self.current_group.members().is_empty());

        MemberChainGroups {
            groups: self.groups,
            tab_width: self.tab_width,
            in_expression_statement: self.in_expression_statement,
            cutoff: 1,
        }
    }
}

#[derive(Clone, Debug)]
/// Handles creation of groups while scanning the flatten items
pub(super) struct MemberChainGroups {
    /// keeps track of the groups created
    groups: Vec<MemberChainGroup>,

    /// If the current group is inside an expression statement.
    ///
    /// This information is important when evaluating the break of the groups.
    in_expression_statement: bool,

    tab_width: TabWidth,

    /// This is a threshold of when we should start breaking the groups
    ///
    /// By default, it's 1, meaning that we start breaking after the first group.
    cutoff: u8,
}

impl MemberChainGroups {
    /// This function checks if the current grouping should be merged with the first group.
    pub fn should_merge(
        &self,
        head_group: &MemberChainGroup,
        comments: &JsComments,
    ) -> SyntaxResult<bool> {
        Ok(!self.groups.len() >= 1
            && self.should_not_wrap(head_group)?
            && !self.groups[0]
                .members
                .first()
                .map_or(false, |item| comments.has_trailing_comments(item.syntax())))
    }

    /// Checks if the groups contain comments.
    pub fn has_comments(&self, comments: &JsComments) -> bool {
        let mut members = self.groups.iter().flat_map(|item| item.members.iter());

        let has_comments = members.any(|item| {
            comments.has_trailing_comments(item.syntax())
                || comments.has_leading_comments(item.syntax())
        });

        let cutoff_has_leading_comments = if self.groups.len() >= self.cutoff as usize {
            let group = self.groups.get(self.cutoff as usize);
            if let Some(group) = group {
                let first_item = group.members.first();
                first_item.map_or(false, |first_item| {
                    comments.has_leading_comments(first_item.syntax())
                })
            } else {
                false
            }
        } else {
            false
        };

        has_comments || cutoff_has_leading_comments
    }

    /// Filters the stack of [FlattenItem] and return only the ones that
    /// contain [JsCallExpression]. The function returns the actual nodes.
    pub fn get_call_expressions(&self) -> impl Iterator<Item = &JsCallExpression> {
        self.groups
            .iter()
            .flat_map(|group| group.members.iter())
            .filter_map(|item| {
                if let ChainMember::CallExpression { expression, .. } = item {
                    Some(expression)
                } else {
                    None
                }
            })
    }

    /// This is an heuristic needed to check when the first element of the group
    /// Should be part of the "head" or the "tail".
    fn should_not_wrap(&self, first_group: &MemberChainGroup) -> SyntaxResult<bool> {
        let tab_with = self.tab_width;
        let has_computed_property = if self.groups.len() > 1 {
            // SAFETY: guarded by the previous check
            let group = &self.groups[0];
            group
                .members
                .first()
                .map_or(false, |item| item.is_computed_expression())
        } else {
            false
        };

        if first_group.members().len() == 1 {
            // SAFETY: access is guarded by the previous check
            let first_node = first_group.members().first().unwrap();

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
            .flat_map(|group| group.members.iter())
            .last()
            .map_or(false, |item| item.is_factory(false).unwrap_or(false));

        Ok(last_node_is_factory || has_computed_property)
    }

    /// Here we check if the first group can be merged to the head. If so, then
    /// we move out the first group out of the groups
    pub(crate) fn should_merge_with_first_group(
        &mut self,
        head_group: &MemberChainGroup,
        comments: &JsComments,
    ) -> Option<Vec<MemberChainGroup>> {
        if self.should_merge(head_group, comments).unwrap_or(false) {
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

    /// Here we check if the length of the groups exceeds the cutoff or there are comments
    /// This function is the inverse of the prettier function
    /// [Prettier applies]: https://github.com/prettier/prettier/blob/a043ac0d733c4d53f980aa73807a63fc914f23bd/src/language-js/print/member-chain.js#L342
    pub(crate) fn is_member_call_chain(&self, comments: &JsComments) -> bool {
        self.groups.len() > self.cutoff as usize || self.has_comments(comments)
    }

    pub(super) fn iter(&self) -> impl Iterator<Item = &MemberChainGroup> {
        self.groups.iter()
    }
}

impl Format<JsFormatContext> for MemberChainGroups {
    fn fmt(&self, f: &mut Formatter<JsFormatContext>) -> FormatResult<()> {
        f.join().entries(self.groups.iter()).finish()
    }
}

#[derive(Debug, Clone, Default)]
pub(super) struct MemberChainGroup {
    members: Vec<ChainMember>,
}

impl MemberChainGroup {
    pub(super) fn into_members(self) -> Vec<ChainMember> {
        self.members
    }

    fn members(&self) -> &[ChainMember] {
        &self.members
    }

    pub(super) fn expand_group(&mut self, group: impl IntoIterator<Item = ChainMember>) {
        self.members.extend(group)
    }

    pub(super) fn has_comments(&self, comments: &JsComments) -> bool {
        self.members.iter().enumerate().any(|(index, member)| {
            if index > 0 && comments.has_leading_comments(member.syntax()) {
                true
            } else if index + 1 < self.members.len() {
                comments.has_leading_comments(member.syntax())
                    || comments.has_trailing_comments(member.syntax())
            } else {
                false
            }
        })
    }
}

impl From<Vec<ChainMember>> for MemberChainGroup {
    fn from(entries: Vec<ChainMember>) -> Self {
        Self { members: entries }
    }
}

impl Format<JsFormatContext> for MemberChainGroup {
    fn fmt(&self, f: &mut Formatter<JsFormatContext>) -> FormatResult<()> {
        let last = self.members.last();

        let needs_parens = last.map_or(false, |last| match last {
            ChainMember::StaticMember { expression, .. } => expression.needs_parentheses(),
            ChainMember::ComputedMember { expression, .. } => expression.needs_parentheses(),
            _ => false,
        });

        let format_entries = format_with(|f| f.join().entries(self.members.iter()).finish());

        if needs_parens {
            write!(f, [text("("), format_entries, text(")")])
        } else {
            write!(f, [format_entries])
        }
    }
}
