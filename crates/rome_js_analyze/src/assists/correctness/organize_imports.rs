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
use rome_js_syntax::{JsAnyImportClause, JsAnyModuleItem, JsImport, JsLanguage, JsModule};
use rome_rowan::{
    syntax::SyntaxTrivia, AstNode, AstNodeExt, AstNodeList, BatchMutationExt, SyntaxTokenText,
    SyntaxTriviaPiece,
};

use crate::JsRuleAction;

declare_rule! {
    /// Provides a whole-source code action to sort the imports in the file alphabetically
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

        let mut first_node = None;
        let mut nodes = BTreeMap::new();

        for item in root.items() {
            let import = match item {
                JsAnyModuleItem::JsImport(import) => import,
                JsAnyModuleItem::JsAnyStatement(_) | JsAnyModuleItem::JsExport(_) => {
                    // If we have pending nodes and encounter a non-import node, append the nodes to a new group
                    if let Some(first_node) = first_node.take() {
                        groups.push(ImportGroup {
                            first_node,
                            nodes: take(&mut nodes),
                        });
                    }
                    continue;
                }
            };

            let first_token = import.import_token().ok()?;

            // If this is not the first import in the group, check for a group break
            if has_empty_line(first_token.leading_trivia()) {
                if let Some(first_node) = first_node.take() {
                    groups.push(ImportGroup {
                        first_node,
                        nodes: take(&mut nodes),
                    });
                }
            }

            // If this is the first import in the group save the leading trivia
            // and slot index
            if first_node.is_none() {
                first_node = Some(import.clone());
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
        if let Some(first_node) = first_node.take() {
            groups.push(ImportGroup { first_node, nodes });
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
            if item_slot < next_group.first_node.syntax().index() {
                new_list.push(item);
                continue;
            }

            // Extract the leading trivia for the whole group from the leading
            // trivia for the import token of the first node in the group. If
            // the trivia contains empty lines the leading trivia for the group
            // comprise all trivia pieces coming before the empty line that's
            // closest to the token. Otherwise the group leading trivia is
            // created from all the newline and whitespace pieces on the first
            // token before the first comment or skipped piece.
            let group_first_token = next_group.first_node.import_token().ok()?;
            let group_leading_trivia = group_first_token.leading_trivia();

            let mut prev_newline = None;
            let mut group_leading_trivia: Vec<_> = group_leading_trivia
                .pieces()
                .enumerate()
                .rev()
                .find_map(|(index, piece)| {
                    if piece.is_whitespace() {
                        return None;
                    }

                    let is_newline = piece.is_newline();
                    if let Some(first_newline) = prev_newline.filter(|_| is_newline) {
                        return Some(first_newline + 1);
                    }

                    prev_newline = is_newline.then_some(index);
                    None
                })
                .map_or_else(
                    || {
                        group_leading_trivia
                            .pieces()
                            .take_while(is_ascii_whitespace)
                            .collect()
                    },
                    |length| group_leading_trivia.pieces().take(length).collect(),
                );

            let mut saved_leading_trivia = Vec::new();
            let group_leading_pieces = group_leading_trivia.len();

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

                let first_token = node.import_token().ok()?;
                let mut node = node.clone().detach();

                if node_index == 0 && group_first_token != first_token {
                    // If this node was not previously in the leading position
                    // but is being moved there, replace its leading whitespace
                    // with the group's leading trivia
                    let group_leading_trivia = group_leading_trivia.drain(..);
                    let mut token_leading_trivia = first_token.leading_trivia().pieces().peekable();

                    // Save off the leading whitespace of the token to be
                    // reused by the import take the place of this node in the list
                    while let Some(piece) = token_leading_trivia.next_if(is_ascii_whitespace) {
                        saved_leading_trivia.push(piece);
                    }

                    node = node.with_import_token(first_token.with_leading_trivia_pieces(
                        exact_chain(group_leading_trivia, token_leading_trivia),
                    ));
                } else if node_index > 0 && group_first_token == first_token {
                    // If this node used to be in the leading position but
                    // got moved, remove the group leading trivia from its
                    // first token
                    let saved_leading_trivia = saved_leading_trivia.drain(..);
                    let token_leading_trivia = first_token
                        .leading_trivia()
                        .pieces()
                        .skip(group_leading_pieces);

                    node = node.with_import_token(first_token.with_leading_trivia_pieces(
                        exact_chain(saved_leading_trivia, token_leading_trivia),
                    ));
                }

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
    /// The import that was at the start of the group before sorting
    first_node: JsImport,
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

/// Returns true is this trivia piece is "ASCII whitespace" (newline or whitespace)
fn is_ascii_whitespace(piece: &SyntaxTriviaPiece<JsLanguage>) -> bool {
    piece.is_newline() || piece.is_whitespace()
}

/// Returns true if the provided trivia contains an empty line (two consecutive newline pieces, ignoring whitespace)
fn has_empty_line(trivia: SyntaxTrivia<JsLanguage>) -> bool {
    let mut was_newline = false;
    trivia
        .pieces()
        .filter(|piece| !piece.is_whitespace())
        .any(|piece| {
            let prev_newline = was_newline;
            was_newline = piece.is_newline();
            prev_newline && was_newline
        })
}

/// Returns an iterator yielding the full content of `lhs`, then the full
/// content of `rhs`. This is similar to the `.chain()` method on the
/// [Iterator] trait except the returned iterator implements [ExactSizeIterator]
fn exact_chain<'a, T>(
    mut lhs: impl Iterator<Item = T> + ExactSizeIterator + 'a,
    mut rhs: impl Iterator<Item = T> + ExactSizeIterator + 'a,
) -> impl Iterator<Item = T> + ExactSizeIterator + 'a {
    let total_len = lhs.len() + rhs.len();
    (0..total_len).map(move |_| {
        // SAFETY: The above range iterator should have the exact length of lhs + rhs
        lhs.next().or_else(|| rhs.next()).unwrap()
    })
}
