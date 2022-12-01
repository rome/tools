use std::{
    cell::Cell,
    cmp::Ordering,
    collections::{btree_map::Entry, BTreeMap},
    iter::once,
    mem::take,
};

use rome_analyze::{
    context::RuleContext, declare_rule, ActionCategory, Ast, Rule, SourceActionKind,
};
use rome_console::markup;
use rome_diagnostics::Applicability;
use rome_js_factory::make;
use rome_js_syntax::{
    AnyJsImportClause, AnyJsModuleItem, AnyJsNamedImport, AnyJsNamedImportSpecifier, JsImport,
    JsLanguage, JsModule, JsSyntaxToken, TextRange, TriviaPieceKind, T,
};
use rome_rowan::{
    syntax::SyntaxTrivia, AstNode, AstNodeExt, AstNodeList, AstSeparatedList, BatchMutationExt,
    SyntaxTokenText, SyntaxTriviaPiece, TriviaPiece,
};

use crate::JsRuleAction;

declare_rule! {
    /// Provides a whole-source code action to sort the imports in the file
    /// using natural ordering
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
                AnyJsModuleItem::JsImport(import) => import,
                AnyJsModuleItem::AnyJsStatement(_) | AnyJsModuleItem::JsExport(_) => {
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
                AnyJsImportClause::JsImportBareClause(clause) => clause.source().ok()?,
                AnyJsImportClause::JsImportDefaultClause(clause) => clause.source().ok()?,
                AnyJsImportClause::JsImportNamedClause(clause) => clause.source().ok()?,
                AnyJsImportClause::JsImportNamespaceClause(clause) => clause.source().ok()?,
            };

            let key = source.inner_string_text().ok()?;
            match nodes.entry(ImportKey(key)) {
                Entry::Vacant(entry) => {
                    entry.insert(vec![ImportNode::from(import)]);
                }
                Entry::Occupied(mut entry) => {
                    entry.get_mut().push(ImportNode::from(import));
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

            for (node_index, import_node) in nodes_iter {
                // For each node in the group, pop an item from the old list
                // iterator (ignoring `item` itself) and discard it
                if node_index > 0 {
                    iter.next()
                        .unwrap_or_else(|| panic!("mising node {item_slot} {node_index}"));
                }

                let first_token = import_node.node.import_token().ok()?;
                let mut node = import_node.build_sorted_node();

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

                new_list.push(AnyJsModuleItem::JsImport(node));
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
    /// sorted in natural order
    nodes: BTreeMap<ImportKey, Vec<ImportNode>>,
}

impl ImportGroup {
    /// Returns true if the nodes in the group are already sorted in the file
    fn is_sorted(&self) -> bool {
        // The imports are sorted if the start of each node in the `BTreeMap`
        // (sorted in natural order) is higher or equal to the previous item in
        // the sequence
        let mut iter = self.nodes.values().flat_map(|nodes| nodes.iter());

        let import_node = match iter.next() {
            Some(node) => node,
            None => return true,
        };

        if !import_node.is_sorted() {
            return false;
        }

        let mut last = import_node.node.syntax().text_range().start();
        iter.all(|import_node| {
            let start = import_node.node.syntax().text_range().start();
            if start < last {
                return false;
            }

            // Only return false if the node has been fully inspected and was
            // found to be unsorted, but continue visiting the remaining
            // imports if the node was sorted or contained syntax errors
            if !import_node.is_sorted() {
                return false;
            }

            last = start;
            true
        })
    }
}

#[derive(Debug)]
struct ImportNode {
    /// The original `JsImport` node this import node was created from
    node: JsImport,
    /// The number of separators present in the named specifiers list of this node if it has one
    separator_count: usize,
    /// Map storing all the named import specifiers and their associated trailing separator,
    /// sorted in natural order
    specifiers: BTreeMap<ImportKey, (AnyJsNamedImportSpecifier, Option<JsSyntaxToken>)>,
}

impl From<JsImport> for ImportNode {
    fn from(node: JsImport) -> Self {
        let import_clause = node.import_clause().ok();

        let mut separator_count = 0;
        let specifiers = import_clause.and_then(|import_clause| {
            let import_named_clause = match import_clause {
                AnyJsImportClause::JsImportBareClause(_)
                | AnyJsImportClause::JsImportDefaultClause(_)
                | AnyJsImportClause::JsImportNamespaceClause(_) => return None,
                AnyJsImportClause::JsImportNamedClause(import_clause) => import_clause,
            };

            let named_import = import_named_clause.named_import().ok()?;
            let named_import_specifiers = match named_import {
                AnyJsNamedImport::JsNamespaceImportSpecifier(_) => return None,
                AnyJsNamedImport::JsNamedImportSpecifiers(named_import_specifiers) => {
                    named_import_specifiers
                }
            };

            let mut result = BTreeMap::new();

            for element in named_import_specifiers.specifiers().elements() {
                let node = element.node.ok()?;
                let key = import_specifier_name(&node)?;

                let trailing_separator = element.trailing_separator.ok()?;
                separator_count += usize::from(trailing_separator.is_some());

                result.insert(ImportKey(key), (node, trailing_separator));
            }

            Some(result)
        });

        Self {
            node,
            separator_count,
            specifiers: specifiers.unwrap_or_default(),
        }
    }
}

impl ImportNode {
    /// Returns `true` if the named import specifiers of this import node are sorted
    fn is_sorted(&self) -> bool {
        let mut iter = self
            .specifiers
            .values()
            .map(|(node, _)| node.syntax().text_range().start());

        let mut last = match iter.next() {
            Some(last) => last,
            None => return true,
        };

        iter.all(|value| {
            if value < last {
                return false;
            }

            last = value;
            true
        })
    }

    /// Build a clone of the original node this import node was created from with its import specifiers sorted
    fn build_sorted_node(&self) -> JsImport {
        let import = self.node.clone().detach();

        let import_clause = import.import_clause();
        let import_named_clause =
            if let Ok(AnyJsImportClause::JsImportNamedClause(node)) = import_clause {
                node
            } else {
                return import;
            };

        let named_import = import_named_clause.named_import();
        let old_specifiers =
            if let Ok(AnyJsNamedImport::JsNamedImportSpecifiers(node)) = named_import {
                node
            } else {
                return import;
            };

        let element_count = self.specifiers.len();
        let last_element = element_count.saturating_sub(1);
        let separator_count = self.separator_count.max(last_element);
        let needs_newline: Cell<Option<Option<JsSyntaxToken>>> = Cell::new(None);

        let items = self
            .specifiers
            .values()
            .enumerate()
            .map(|(index, (node, sep))| {
                let is_last = index == last_element;

                let mut node = node.clone().detach();
                let prev_token = match node.syntax().last_token() {
                    Some(last_token) => last_token,
                    None => return node,
                };

                if let Some(sep) = sep {
                    if is_last && separator_count == last_element {
                        // If this is the last item and we are removing its trailing separator,
                        // move the trailing trivia from the separator to the node
                        let next_token = prev_token.with_trailing_trivia(exact_chain(
                            trivia_iter(&prev_token, TriviaPosition::Trailing),
                            trivia_iter(sep, TriviaPosition::Trailing),
                        ));

                        node = node
                            .replace_token_discard_trivia(prev_token, next_token)
                            .expect("prev_token should be a child of node");
                    }
                } else if !is_last {
                    // If the node has no separator and this is not the last item,
                    // remove the trailing trivia since it will get cloned on the inserted separator
                    let next_token = prev_token.with_trailing_trivia([]);
                    node = node
                        .replace_token_discard_trivia(prev_token, next_token)
                        .expect("prev_token should be a child of node");
                }

                // Check if the last separator we emitted ended with a single-line comment
                if let Some(newline_source) = needs_newline.take() {
                    if let Some(first_token) = node.syntax().first_token() {
                        if let Some(new_token) =
                            prepend_leading_newline(&first_token, newline_source)
                        {
                            node = node
                                .replace_token_discard_trivia(first_token, new_token)
                                .expect("first_token should be a child of node");
                        }
                    }
                }

                node
            });

        let separators = self
            .specifiers
            .values()
            .take(separator_count)
            .map(|(node, sep)| {
                // If this entry has an associated separator, reuse it
                let (token, will_need_newline) = if let Some(sep) = sep {
                    // If the last trivia piece for the separator token is a single-line comment,
                    // signal to the items iterator it will need to prepend a newline to the leading
                    // trivia of the next node
                    let will_need_newline = sep
                        .trailing_trivia()
                        .last()
                        .map(|piece| matches!(piece.kind(), TriviaPieceKind::SingleLineComment))
                        .unwrap_or(false);

                    (sep.clone(), will_need_newline)
                } else {
                    // If the node we're attaching this separator to has no trailing trivia, just create a simple comma token
                    let last_trailing_trivia = match node.syntax().last_trailing_trivia() {
                        Some(trivia) if !trivia.is_empty() => trivia,
                        _ => return make::token(T![,]),
                    };

                    // Otherwise we need to clone the trailing trivia from the node to the separator
                    // (the items iterator should have already filtered this trivia when it previously
                    // emitted the node)
                    let mut text = String::from(",");
                    let mut trailing = Vec::with_capacity(last_trailing_trivia.pieces().len());

                    let mut will_need_newline = false;
                    for piece in last_trailing_trivia.pieces() {
                        text.push_str(piece.text());
                        trailing.push(TriviaPiece::new(piece.kind(), piece.text_len()));
                        will_need_newline =
                            matches!(piece.kind(), TriviaPieceKind::SingleLineComment);
                    }

                    let token = JsSyntaxToken::new_detached(T![,], &text, [], trailing);
                    (token, will_need_newline)
                };

                // If the last trivia piece was a single-line comment, signal to the items iterator
                // it will need to prepend a newline to the leading trivia of the next node, and provide
                // it the token that followed this separator in the original source so the newline trivia
                // can be cloned from there
                let newline_source =
                    will_need_newline.then(|| sep.as_ref().and_then(|token| token.next_token()));

                needs_newline.set(newline_source);

                token
            });

        let mut new_specifiers = old_specifiers
            .clone()
            .detach()
            .with_specifiers(make::js_named_import_specifier_list(items, separators));

        // If the separators iterator has a pending newline, prepend it to closing curly token
        if let Some(newline_source) = needs_newline.into_inner() {
            let new_token = new_specifiers
                .r_curly_token()
                .ok()
                .and_then(|token| prepend_leading_newline(&token, newline_source));

            if let Some(new_token) = new_token {
                new_specifiers = new_specifiers.with_r_curly_token(new_token);
            }
        }

        import
            .replace_node_discard_trivia(old_specifiers, new_specifiers)
            .expect("old_specifiers should be a child of import")
    }
}

/// Return the name associated with a given named import specifier
///
/// Currently named import specifiers are sorted using their import name,
/// not the local name they get imported as
fn import_specifier_name(import_specifier: &AnyJsNamedImportSpecifier) -> Option<SyntaxTokenText> {
    let token = match import_specifier {
        AnyJsNamedImportSpecifier::JsNamedImportSpecifier(import_specifier) => {
            import_specifier.name().ok()?.value().ok()?
        }
        AnyJsNamedImportSpecifier::JsShorthandNamedImportSpecifier(import_specifier) => {
            import_specifier
                .local_name()
                .ok()?
                .as_js_identifier_binding()?
                .name_token()
                .ok()?
        }
        AnyJsNamedImportSpecifier::JsBogusNamedImportSpecifier(_) => return None,
    };

    Some(token.token_text_trimmed())
}

/// Return a clone of `prev_token` with a newline trivia piece prepended to its
/// leading trivia if it didn't have one already. This function will try to copy
/// the newline trivia piece from the leading trivia of `newline_source` if its set
fn prepend_leading_newline(
    prev_token: &JsSyntaxToken,
    newline_source: Option<JsSyntaxToken>,
) -> Option<JsSyntaxToken> {
    // Check if this node already starts with a newline,
    // if it does we don't need to prepend anything
    let leading_trivia = prev_token.leading_trivia();
    let has_leading_newline = leading_trivia
        .first()
        .map(|piece| piece.is_newline())
        .unwrap_or(false);

    if has_leading_newline {
        return None;
    }

    // Extract the leading newline from the `newline_source` token
    let leading_newline = newline_source.and_then(|newline_source| {
        let leading_trivia = newline_source.leading_trivia();
        let leading_piece = leading_trivia.first()?;

        if !leading_piece.is_newline() {
            return None;
        }

        Some(leading_piece)
    });

    // Prepend a newline trivia piece to the node, either by copying the leading newline
    // and whitespace from `newline_source`, or falling back to the "\n" character
    let leading_newline = if let Some(leading_newline) = &leading_newline {
        (leading_newline.kind(), leading_newline.text())
    } else {
        (TriviaPieceKind::Newline, "\n")
    };

    let piece_count = 1 + leading_trivia.pieces().len();
    let mut iter = once(leading_newline).chain(trivia_iter(prev_token, TriviaPosition::Leading));

    Some(prev_token.with_leading_trivia((0..piece_count).map(|_| iter.next().unwrap())))
}

enum TriviaPosition {
    Leading,
    Trailing,
}

/// Builds an iterator over the leading or trailing trivia pieces of a token
///
/// The items of the iterator inherit their lifetime from the token,
/// rather than the trivia pieces themselves
fn trivia_iter(
    token: &JsSyntaxToken,
    position: TriviaPosition,
) -> impl Iterator<Item = (TriviaPieceKind, &str)> + ExactSizeIterator {
    let token_text = token.text();
    let token_range = token.text_range();

    let trivia = match position {
        TriviaPosition::Leading => token.leading_trivia(),
        TriviaPosition::Trailing => token.trailing_trivia(),
    };

    trivia.pieces().map(move |piece| {
        let piece_range = piece.text_range();
        let range = TextRange::at(piece_range.start() - token_range.start(), piece_range.len());

        let text = &token_text[range];
        assert_eq!(text, piece.text());

        (piece.kind(), text)
    })
}

#[derive(Debug)]
struct ImportKey(SyntaxTokenText);

impl Ord for ImportKey {
    fn cmp(&self, other: &Self) -> Ordering {
        // Sort imports using natural ordering
        natord::compare(&self.0, &other.0)
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
