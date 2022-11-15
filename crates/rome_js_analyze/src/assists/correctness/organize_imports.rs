use std::{
    cmp::Ordering,
    collections::{btree_map::Entry, BTreeMap},
    mem::take,
};

use rome_analyze::{
    context::RuleContext, declare_rule, ActionCategory, Ast, Rule, SourceActionKind,
};
use rome_console::markup;
use rome_diagnostics::Applicability;
use rome_js_factory::make;
use rome_js_syntax::{
    JsAnyImportClause, JsAnyModuleItem, JsImport, JsModule, JsSyntaxToken, TriviaPieceKind,
};
use rome_rowan::{AstNode, AstNodeExt, AstNodeList, BatchMutationExt, SyntaxTokenText};

use crate::JsRuleAction;

declare_rule! {
    /// Provides a whole-source code action to sort the imports in the file
    ///
    /// ## Examples
    ///
    /// ```js
    /// import React, {
    ///     FC,
    ///     useEffect,
    ///     useRef,
    ///     ChangeEvent,
    ///     KeyboardEvent,
    /// } from 'react';
    /// import { logger } from '@core/logger';
    /// import { reduce, debounce } from 'lodash';
    /// import { Message } from '../Message';
    /// import { createServer } from '@server/node';
    /// import { Alert } from '@ui/Alert';
    /// import { repeat, filter, add } from '../utils';
    /// import { initializeApp } from '@core/app';
    /// import { Popup } from '@ui/Popup';
    /// import { createConnection } from '@server/database';
    /// ```
    pub(crate) OrganizeImports {
        version: "11.0.0",
        name: "organizeImports",
        recommended: false,
    }
}

impl Rule for OrganizeImports {
    type Query = Ast<JsModule>;
    type State = ImportGroups;
    type Signals = Option<Self::State>;
    type Options = ();

    fn run(ctx: &RuleContext<Self>) -> Option<Self::State> {
        let root = ctx.query();

        let mut groups = Vec::new();

        let mut first_slot = 0;
        let mut group_leading_newlines = 0;
        let mut nodes = BTreeMap::new();

        for item in root.items() {
            let import = match item {
                JsAnyModuleItem::JsImport(import) => import,
                JsAnyModuleItem::JsAnyStatement(_) | JsAnyModuleItem::JsExport(_) => {
                    // If we have pending nodes and encounter a non-import node, append the nodes to a new group
                    if !nodes.is_empty() {
                        groups.push(ImportGroup {
                            first_slot,
                            leading_newlines: group_leading_newlines,
                            nodes: take(&mut nodes),
                        });
                    }
                    continue;
                }
            };

            let first_token = import.import_token().ok()?;
            let leading_newlines = count_leading_newlines(&first_token);

            if !nodes.is_empty() {
                // If this is not the first import in the group, check for a group break
                if leading_newlines >= 2 {
                    groups.push(ImportGroup {
                        first_slot,
                        leading_newlines: group_leading_newlines,
                        nodes: take(&mut nodes),
                    });

                    first_slot = import.syntax().index();
                    group_leading_newlines = 2;
                }
            } else {
                // If this is the first import in the group save the number of leading newlines
                first_slot = import.syntax().index();
                group_leading_newlines = leading_newlines.min(2);
            }

            let source = match import.import_clause().ok()? {
                JsAnyImportClause::JsImportBareClause(clause) => clause.source().ok()?,
                JsAnyImportClause::JsImportDefaultClause(clause) => clause.source().ok()?,
                JsAnyImportClause::JsImportNamedClause(clause) => clause.source().ok()?,
                JsAnyImportClause::JsImportNamespaceClause(clause) => clause.source().ok()?,
            };

            let key = source.inner_string_text().ok()?;
            match nodes.entry(ImportKey(key)) {
                Entry::Vacant(entry) => {
                    entry.insert(vec![import]);
                }
                Entry::Occupied(mut entry) => {
                    entry.get_mut().push(import);
                }
            }
        }

        // Flush the remaining nodes
        if !nodes.is_empty() {
            groups.push(ImportGroup {
                first_slot,
                leading_newlines: group_leading_newlines,
                nodes,
            });
        }

        groups
            .iter()
            .any(|group| !group.is_sorted())
            .then_some(ImportGroups { groups })
    }

    fn action(ctx: &RuleContext<Self>, groups: &Self::State) -> Option<JsRuleAction> {
        let mut groups_iter = groups.groups.iter();
        let mut next_group = groups_iter.next().expect("state is empty");

        let old_list = ctx.query().items();
        let mut new_list = Vec::new();

        let mut items_iter = old_list.iter();
        let mut iter = (&mut items_iter).enumerate();

        // Iterate other the nodes of the old list
        while let Some((item_slot, item)) = iter.next() {
            // If the current position in the old list is lower than the start
            // of the new group, append the old node to the new list
            if item_slot < next_group.first_slot {
                new_list.push(item);
                continue;
            }

            let nodes_iter = next_group
                .nodes
                .values()
                // TODO: Try to merge nodes from the same source
                .flat_map(|nodes| nodes.iter())
                .enumerate();

            for (node_index, node) in nodes_iter {
                // For each node in the group, pop an item from the old list
                // iterator (ignoring `item` itself) and discard it
                if node_index > 0 {
                    iter.next()
                        .unwrap_or_else(|| panic!("mising node {item_slot} {node_index}"));
                }

                // Preserve the leading newlines count for the first import in
                // the group, and normalize it to 1 for the rest of the nodes
                let expected_leading_newlines = if node_index == 0 {
                    next_group.leading_newlines
                } else {
                    1
                };

                let node = normalize_leading_newlines(node, expected_leading_newlines)?;
                new_list.push(JsAnyModuleItem::JsImport(node));
            }

            // Load the next group before moving on to the next item in the old
            // list, breaking the loop if there a no remaining groups to insert
            next_group = match groups_iter.next() {
                Some(entry) => entry,
                None => break,
            };
        }

        // Append all remaining nodes to the new list if the loop performed an
        // early exit after reaching the last group
        new_list.extend(items_iter);

        let new_list = make::js_module_item_list(new_list);

        let mut mutation = ctx.root().begin();
        mutation.replace_node_discard_trivia(old_list, new_list);

        Some(JsRuleAction {
            category: ActionCategory::Source(SourceActionKind::OrganizeImports),
            applicability: Applicability::MaybeIncorrect,
            message: markup! { "Organize Imports" }.to_owned(),
            mutation,
        })
    }
}

#[derive(Debug)]
pub(crate) struct ImportGroups {
    /// The list of all the import groups in the file
    groups: Vec<ImportGroup>,
}

#[derive(Debug)]
struct ImportGroup {
    /// The starting index of this group in the module's item list
    first_slot: usize,
    /// The number of leading newlines the first import in the group should have
    leading_newlines: usize,
    /// Multimap storing all the imports for each import source in the group,
    /// sorted in alphabetical order
    nodes: BTreeMap<ImportKey, Vec<JsImport>>,
}

impl ImportGroup {
    /// Returns true if the nodes in the group are already sorted in the file
    fn is_sorted(&self) -> bool {
        // The imports are sorted if the start of each node in the `BTreeMap`
        // (sorted alphabetically) is higher or equal to the previous item in
        // the sequence
        let mut iter = self
            .nodes
            .values()
            .flat_map(|nodes| nodes.iter())
            .map(|node| node.syntax().text_range().start());

        let mut last = match iter.next() {
            Some(last) => last,
            None => return true,
        };

        iter.all(|value| {
            let result = value >= last;
            last = value;
            result
        })
    }
}

#[derive(Debug)]
struct ImportKey(SyntaxTokenText);

impl Ord for ImportKey {
    fn cmp(&self, other: &Self) -> Ordering {
        // Sort imports alphabetically by defering to the string ordering logic
        // of the standard library
        (*self.0).cmp(&*other.0)
    }
}

impl PartialOrd for ImportKey {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Eq for ImportKey {}

impl PartialEq for ImportKey {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

/// Return a copy of `node` with `expected_leading_newlines` newline trivia
/// pieces at the start of its leading trivia
fn normalize_leading_newlines(
    node: &JsImport,
    expected_leading_newlines: usize,
) -> Option<JsImport> {
    let first_token = node.import_token().ok()?;
    let actual_leading_newlines = count_leading_newlines(&first_token);

    if actual_leading_newlines != expected_leading_newlines {
        let mut first_token = first_token.detach();

        first_token = if actual_leading_newlines > expected_leading_newlines {
            // The node has more leading newlines than necessary: skip the
            // excess trivia pieces when reconstructing the leading trivia
            let diff = actual_leading_newlines - expected_leading_newlines;
            let leading_trivia: Vec<_> = first_token.leading_trivia().pieces().skip(diff).collect();

            first_token.with_leading_trivia(
                leading_trivia
                    .iter()
                    .map(|piece| (piece.kind(), piece.text())),
            )
        } else {
            // The node has less leading newlines than necessary: prepend the
            // reconstructed leading trivia with additional newline pieces
            let leading_trivia: Vec<_> = first_token.leading_trivia().pieces().collect();

            let diff = expected_leading_newlines - actual_leading_newlines;
            let total_len = leading_trivia.len() + diff;

            first_token.with_leading_trivia((0..total_len).map(|index| {
                if let Some(index) = index.checked_sub(diff) {
                    let piece = &leading_trivia[index];
                    (piece.kind(), piece.text())
                } else {
                    // TODO: Use "\r\n" if the file is using CRLF line terminators
                    (TriviaPieceKind::Newline, "\n")
                }
            }))
        };

        Some(node.clone().detach().with_import_token(first_token))
    } else {
        Some(node.clone())
    }
}

/// Return the number of newline trivia pieces contained in the leading trivia
/// of `token` before the first comment or skipped trivia
fn count_leading_newlines(token: &JsSyntaxToken) -> usize {
    token
        .leading_trivia()
        .pieces()
        .take_while(|piece| !piece.is_comments() && !piece.is_skipped())
        .filter(|piece| piece.is_newline())
        .count()
}
