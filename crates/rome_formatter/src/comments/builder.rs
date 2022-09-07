use crate::comments::multimap::AppendOnlyMultiMap;
use crate::comments::CommentsData;
use crate::{CommentPlacement, CommentStyle, DecoratedComment, FormatLanguage, SourceComment};
use rome_rowan::{Language, SyntaxElement, SyntaxKind, SyntaxNode, SyntaxToken, WalkEvent};
use std::cell::RefCell;
use std::collections::HashSet;

#[derive(Debug)]
pub(super) struct CommentsBuilderVisitor<Language: FormatLanguage> {
    node_comments: NodeCommentsBuilder<Language::SyntaxLanguage>,
    with_skipped: HashSet<SyntaxToken<Language::SyntaxLanguage>>,
    preceding_node: Option<SyntaxNode<Language::SyntaxLanguage>>,
    following_node: Option<SyntaxNode<Language::SyntaxLanguage>>,
    last_token: Option<SyntaxToken<Language::SyntaxLanguage>>,
}

impl<L: FormatLanguage> Default for CommentsBuilderVisitor<L> {
    fn default() -> Self {
        Self {
            node_comments: Default::default(),
            with_skipped: Default::default(),
            preceding_node: Default::default(),
            following_node: Default::default(),
            last_token: Default::default(),
        }
    }
}

impl<Language> CommentsBuilderVisitor<Language>
where
    Language: FormatLanguage,
{
    pub(super) fn visit_node(&mut self, event: WalkEvent<SyntaxNode<Language::SyntaxLanguage>>) {
        match event {
            WalkEvent::Enter(node) => {
                // Lists cannot have comments attached. They either belong to the entire parent or to
                // the first child.
                if node.kind().is_list() {
                    return;
                }

                // Associate comments with the most outer node
                if self.following_node.is_none() {
                    self.following_node = Some(node);
                }
            }

            WalkEvent::Leave(node) => {
                if node.kind().is_list() {
                    return;
                }

                if self.following_node.as_ref() == Some(&node) {
                    self.following_node = None;
                }

                self.preceding_node = Some(node);
            }
        }
    }

    pub(super) fn visit_token(&mut self, token: SyntaxToken<Language::SyntaxLanguage>) {
        // Store the last processed comment so that we can set `line_break_after`
        let mut last_comment = None;

        if let Some(last_token) = self.last_token.take() {
            for piece in last_token
                .trailing_trivia()
                .pieces()
                .filter_map(|piece| piece.as_comments())
            {
                if let Some(last_comment) = last_comment.take() {
                    self.handle_comment(last_comment);
                }

                last_comment = Some(DecoratedComment {
                    preceding: self.preceding_node.clone(),
                    following: self.following_node.clone(),
                    following_token: token.clone(),
                    lines_before: 0,
                    lines_after: 0,
                    trailing_token_comment: true,
                    kind: Language::CommentStyle::get_comment_kind(&piece),
                    comment: piece,
                });
            }
        }

        let mut lines_before = 0;
        let mut has_skipped = false;

        for leading in token.leading_trivia().pieces() {
            if leading.is_newline() {
                lines_before += 1;
            } else if leading.is_skipped() {
                if let Some(mut last_comment) = last_comment.take() {
                    last_comment.lines_after = lines_before;
                    self.handle_comment(last_comment);
                }

                self.with_skipped.insert(token.clone());

                lines_before = 0;
                has_skipped = true;
            } else if let Some(comment) = leading.as_comments() {
                if let Some(mut last_comment) = last_comment.take() {
                    last_comment.lines_after = lines_before;
                    self.handle_comment(last_comment);
                }

                let kind = Language::CommentStyle::get_comment_kind(&comment);
                if !has_skipped {
                    last_comment = Some(DecoratedComment {
                        preceding: self.preceding_node.clone(),
                        following: self.following_node.clone(),
                        following_token: token.clone(),
                        lines_before,
                        lines_after: 0,
                        trailing_token_comment: false,
                        kind,
                        comment,
                    });
                }
                lines_before = 0;
            }
        }

        if let Some(mut last_comment) = last_comment.take() {
            last_comment.lines_after = lines_before;
            self.handle_comment(last_comment);
        }

        // Any comment following now is preceded by 'token' and not a node.

        // TODO: Difference to prettier:
        // - Prettier keeps the preceding node around, even if there has been a token in between
        // - Prettier keeps the following node around, even if there has been a token in between
        // - a = b; a is still the preceding even if positioned at b;
        // - same with following. It takes the first node that follows (and belongs to the same parent)
        // They then use a breakTie in situations where there are preceding and following nodes set.
        // Emphasis the importance of nodes even more. Reduces the places where dangling comments can appear
        // Has mainly become relevant for trailing comments. Is there also a noticable difference for leading?
        self.preceding_node = None;
        self.following_node = None;
        self.last_token = Some(token);
    }

    fn handle_comment(&mut self, comment: DecoratedComment<Language::SyntaxLanguage>) {
        match Language::CommentStyle::place_comment(comment) {
            CommentPlacement::Leading { node, comment } => {
                self.node_comments
                    .insert_leading_comment(node, comment.into());
            }
            CommentPlacement::Trailing { node, comment } => {
                self.node_comments
                    .insert_trailing_comment(node, comment.into());
            }
            CommentPlacement::Dangling { node, comment } => self
                .node_comments
                .insert_dangling_comment(node, comment.into()),
            CommentPlacement::Default(mut comment) => {
                if comment.is_trailing_token_trivia() {
                    let enclosing = comment.enclosing_node();

                    // The enclosing can only ever be a list if the comment is a leading or trailing comment of a
                    // separator token in a separated list.
                    // Example:
                    // ```js
                    // [
                    //   a, // test
                    //   b
                    // ]
                    // ```
                    // The default algorithm would make `// test` a leading comment of the node `b` but
                    // it should be a trailing comment of `a` because that's most likely what the user intended.
                    if enclosing.kind().is_list() && comment.lines_after() > 0 {
                        if let Some(SyntaxElement::Node(node)) =
                            comment.comment.as_piece().token().prev_sibling_or_token()
                        {
                            self.node_comments
                                .insert_trailing_comment(node, comment.into());
                            return;
                        }
                    }

                    match (comment.take_preceding_node(), comment.take_following_node()) {
                        (Some(preceding), Some(following)) => {
                            // Always attach suppression with the next node.
                            if Language::CommentStyle::is_suppression(comment.comment.text()) {
                                self.node_comments
                                    .insert_leading_comment(following, comment.into());
                            } else {
                                // Attach comments with both preceding and following node to the preceding
                                // because there's a line break separating it from the following node.
                                // ```javascript
                                // a; // comment
                                // b
                                // ```
                                self.node_comments
                                    .insert_trailing_comment(preceding, comment.into());
                            }
                        }
                        (Some(preceding), None) => {
                            self.node_comments
                                .insert_trailing_comment(preceding, comment.into());
                        }
                        (None, Some(following)) => {
                            self.node_comments
                                .insert_leading_comment(following, comment.into());
                        }
                        (None, None) => {
                            self.node_comments
                                .insert_dangling_comment(enclosing, comment.into());
                        }
                    }
                } else {
                    match (comment.take_following_node(), comment.take_preceding_node()) {
                        // Following always wins for a leading comment
                        // ```javascript
                        // a;
                        // // comment
                        // b
                        // ```
                        // attach the comment to the `b` expression statement
                        (Some(following), _) => {
                            self.node_comments
                                .insert_leading_comment(following, comment.into());
                        }
                        (None, Some(preceding)) => {
                            self.node_comments
                                .insert_trailing_comment(preceding, comment.into());
                        }
                        (None, None) => {
                            self.node_comments
                                .insert_dangling_comment(comment.enclosing_node(), comment.into());
                        }
                    }
                }
            }
        }
    }

    pub(super) fn finish(self) -> CommentsData<Language::SyntaxLanguage> {
        let (leading_comments, dangling_comments, trailing_comments) = self.node_comments.finish();

        CommentsData {
            is_suppression: Language::CommentStyle::is_suppression,
            leading_comments,
            dangling_comments,
            trailing_comments,
            with_skipped: self.with_skipped,

            #[cfg(debug_assertions)]
            checked_suppressions: RefCell::default(),
        }
    }
}

// TODO necessary?
#[derive(Debug)]
struct NodeCommentsBuilder<L: Language> {
    leading_comments: AppendOnlyMultiMap<SyntaxNode<L>, SourceComment<L>>,
    dangling_comments: AppendOnlyMultiMap<SyntaxNode<L>, SourceComment<L>>,
    trailing_comments: AppendOnlyMultiMap<SyntaxNode<L>, SourceComment<L>>,
}

impl<L: Language> NodeCommentsBuilder<L> {
    fn insert_leading_comment(&mut self, node: SyntaxNode<L>, comment: SourceComment<L>) {
        self.leading_comments.append(node, comment);
    }

    fn insert_dangling_comment(&mut self, node: SyntaxNode<L>, comment: SourceComment<L>) {
        self.dangling_comments.append(node, comment);
    }

    fn insert_trailing_comment(&mut self, node: SyntaxNode<L>, comment: SourceComment<L>) {
        self.trailing_comments.append(node, comment);
    }

    fn finish(
        self,
    ) -> (
        AppendOnlyMultiMap<SyntaxNode<L>, SourceComment<L>>,
        AppendOnlyMultiMap<SyntaxNode<L>, SourceComment<L>>,
        AppendOnlyMultiMap<SyntaxNode<L>, SourceComment<L>>,
    ) {
        (
            self.leading_comments,
            self.dangling_comments,
            self.trailing_comments,
        )
    }
}

impl<L: Language> Default for NodeCommentsBuilder<L> {
    fn default() -> Self {
        Self {
            leading_comments: AppendOnlyMultiMap::new(),
            dangling_comments: AppendOnlyMultiMap::new(),
            trailing_comments: AppendOnlyMultiMap::new(),
        }
    }
}
