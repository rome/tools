use crate::comments::map::CommentsMap;
use crate::comments::CommentPosition;
use crate::{CommentPlacement, CommentStyle, DecoratedComment, SourceComment};
use rome_rowan::syntax::SyntaxElementKey;
use rome_rowan::{
    Direction, Language, SyntaxElement, SyntaxKind, SyntaxNode, SyntaxToken, WalkEvent,
};
use rustc_hash::FxHashSet;

pub(super) struct CommentsBuilderVisitor<'a, Style: CommentStyle> {
    builder: CommentsBuilder<Style::Language>,
    style: &'a Style,

    // State
    pending_comments: Vec<DecoratedComment<Style::Language>>,
    preceding_node: Option<SyntaxNode<Style::Language>>,
    following_node_index: Option<usize>,
    parents: Vec<SyntaxNode<Style::Language>>,
    last_token: Option<SyntaxToken<Style::Language>>,
}

impl<'a, Style> CommentsBuilderVisitor<'a, Style>
where
    Style: CommentStyle,
{
    pub(super) fn new(style: &'a Style) -> Self {
        Self {
            style,
            builder: Default::default(),

            pending_comments: Default::default(),
            preceding_node: Default::default(),
            following_node_index: Default::default(),
            parents: Default::default(),
            last_token: Default::default(),
        }
    }

    pub(super) fn visit(
        mut self,
        root: &SyntaxNode<Style::Language>,
    ) -> (
        CommentsMap<SyntaxElementKey, SourceComment<Style::Language>>,
        FxHashSet<SyntaxElementKey>,
    ) {
        for event in root.preorder_with_tokens(Direction::Next) {
            match event {
                WalkEvent::Enter(SyntaxElement::Node(node)) => {
                    self.visit_node(WalkEvent::Enter(node))
                }

                WalkEvent::Leave(SyntaxElement::Node(node)) => {
                    self.visit_node(WalkEvent::Leave(node))
                }

                WalkEvent::Enter(SyntaxElement::Token(token)) => self.visit_token(token),
                WalkEvent::Leave(SyntaxElement::Token(_)) => {
                    // Handled as part of enter
                }
            }
        }

        assert!(
            self.parents.is_empty(),
            "Expected all enclosing nodes to have been processed but contains {:#?}",
            self.parents
        );

        self.flush_comments(None);

        self.builder.finish()
    }

    fn visit_node(&mut self, event: WalkEvent<SyntaxNode<Style::Language>>) {
        match event {
            WalkEvent::Enter(node) => {
                // Lists cannot have comments attached. They either belong to the entire parent or to
                // the first child. So we ignore lists all together
                if node.kind().is_list() {
                    return;
                }

                let is_root = self.parents.is_empty();

                // Associate comments with the most outer node
                // Set following here because it is the "following node" of the next token's leading trivia.
                if !is_root && self.following_node_index.is_none() {
                    // Flush in case the node doesn't have any tokens.
                    self.flush_comments(Some(&node));
                    self.following_node_index = Some(self.parents.len());
                }

                self.parents.push(node);
            }

            WalkEvent::Leave(node) => {
                if node.kind().is_list() {
                    return;
                }

                self.parents.pop().unwrap();

                // We're passed this node, flush any pending comments for its children
                self.following_node_index = None;
                self.flush_comments(None);

                // We're passed this node, so it must precede the sibling that comes next.
                self.preceding_node = Some(node);
            }
        }
    }

    fn visit_token(&mut self, token: SyntaxToken<Style::Language>) {
        let comments_start = self.pending_comments.len();

        // The index of the last trailing comment in `pending_comments`.
        let mut trailing_end: Option<usize> = None;

        // Number of lines before the next comment, token, or skipped token trivia
        let mut lines_before = 0;

        // Trailing comments are all `SameLine` comments EXCEPT if any is followed by a line break,
        // a leading comment (that always have line breaks), or there's a line break before the token.
        let mut position = CommentPosition::SameLine;

        // Process the trailing trivia of the last token
        if let Some(last_token) = self.last_token.take() {
            for piece in last_token.trailing_trivia().pieces() {
                if piece.is_newline() {
                    lines_before += 1;
                    // All comments following from here are own line comments
                    position = CommentPosition::OwnLine;

                    if trailing_end.is_none() {
                        trailing_end = Some(self.pending_comments.len());
                    }
                } else if let Some(comment) = piece.as_comments() {
                    self.queue_comment(DecoratedComment {
                        enclosing: self.enclosing_node().clone(),
                        preceding: self.preceding_node.clone(),
                        following: None,
                        following_token: token.clone(),
                        lines_before,
                        lines_after: 0, // Will be initialized after
                        position,
                        kind: Style::get_comment_kind(&comment),
                        comment,
                    });

                    lines_before = 0;
                }
            }
        }

        let trailing_end = trailing_end.unwrap_or(self.pending_comments.len());

        // Process the leading trivia of the current token. the trailing trivia is handled as part of the next token
        for leading in token.leading_trivia().pieces() {
            if leading.is_newline() {
                lines_before += 1;
            } else if leading.is_skipped() {
                self.builder.mark_has_skipped(&token);

                lines_before = 0;
                break;
            } else if let Some(comment) = leading.as_comments() {
                let kind = Style::get_comment_kind(&comment);

                self.queue_comment(DecoratedComment {
                    enclosing: self.enclosing_node().clone(),
                    preceding: self.preceding_node.clone(),
                    following: None,
                    following_token: token.clone(),
                    lines_before,
                    lines_after: 0,
                    position: CommentPosition::OwnLine,
                    kind,
                    comment,
                });

                lines_before = 0;
            }
        }

        self.last_token = Some(token);

        let has_leading_comments = self.pending_comments.len() > trailing_end;
        let mut comments = self.pending_comments[comments_start..]
            .iter_mut()
            .enumerate()
            .peekable();

        // Update the lines after of all comments as well as the positioning of end of line comments.
        while let Some((index, comment)) = comments.next() {
            if index < trailing_end && (has_leading_comments || lines_before > 0) {
                comment.position = CommentPosition::EndOfLine;
            }

            comment.lines_after = comments
                .peek()
                .map_or(lines_before, |(_, next)| next.lines_before);
        }

        // Set following node to `None` because it now becomes the enclosing node.
        if let Some(following_node) = self.following_node() {
            self.flush_comments(Some(&following_node.clone()));
            self.following_node_index = None;

            // The following node is only set after entering a node
            // That means, following node is only set for the first token of a node.
            // Unset preceding node if this is the first token because the preceding node belongs to the parent.
            self.preceding_node = None;
        }
    }

    fn enclosing_node(&self) -> &SyntaxNode<Style::Language> {
        let element = match self.following_node_index {
            None => self.parents.last(),
            Some(index) => Some(&self.parents[index - 1]),
        };

        element.expect("Expected enclosing nodes to at least contain the root node.")
    }

    fn following_node(&self) -> Option<&SyntaxNode<Style::Language>> {
        self.following_node_index.map(|index| {
            self.parents
                .get(index)
                .expect("Expected following node index to point to a valid parent node")
        })
    }

    fn queue_comment(&mut self, comment: DecoratedComment<Style::Language>) {
        self.pending_comments.push(comment);
    }

    fn flush_comments(&mut self, following: Option<&SyntaxNode<Style::Language>>) {
        for mut comment in self.pending_comments.drain(..) {
            comment.following = following.cloned();

            let placement = self.style.place_comment(comment);
            self.builder.add_comment(placement);
        }
    }
}

struct CommentsBuilder<L: Language> {
    comments: CommentsMap<SyntaxElementKey, SourceComment<L>>,
    skipped: FxHashSet<SyntaxElementKey>,
}

impl<L: Language> CommentsBuilder<L> {
    fn add_comment(&mut self, placement: CommentPlacement<L>) {
        match placement {
            CommentPlacement::Leading { node, comment } => {
                self.push_leading_comment(&node, comment);
            }
            CommentPlacement::Trailing { node, comment } => {
                self.push_trailing_comment(&node, comment);
            }
            CommentPlacement::Dangling { node, comment } => {
                self.push_dangling_comment(&node, comment)
            }
            CommentPlacement::Default(mut comment) => {
                match comment.position {
                    CommentPosition::EndOfLine => {
                        match (comment.take_preceding_node(), comment.take_following_node()) {
                            (Some(preceding), Some(_)) => {
                                // Attach comments with both preceding and following node to the preceding
                                // because there's a line break separating it from the following node.
                                // ```javascript
                                // a; // comment
                                // b
                                // ```
                                self.push_trailing_comment(&preceding, comment);
                            }
                            (Some(preceding), None) => {
                                self.push_trailing_comment(&preceding, comment);
                            }
                            (None, Some(following)) => {
                                self.push_leading_comment(&following, comment);
                            }
                            (None, None) => {
                                self.push_dangling_comment(
                                    &comment.enclosing_node().clone(),
                                    comment,
                                );
                            }
                        }
                    }
                    CommentPosition::OwnLine => {
                        match (comment.take_preceding_node(), comment.take_following_node()) {
                            // Following always wins for a leading comment
                            // ```javascript
                            // a;
                            // // comment
                            // b
                            // ```
                            // attach the comment to the `b` expression statement
                            (_, Some(following)) => {
                                self.push_leading_comment(&following, comment);
                            }
                            (Some(preceding), None) => {
                                self.push_trailing_comment(&preceding, comment);
                            }
                            (None, None) => {
                                self.push_dangling_comment(
                                    &comment.enclosing_node().clone(),
                                    comment,
                                );
                            }
                        }
                    }
                    CommentPosition::SameLine => {
                        match (comment.take_preceding_node(), comment.take_following_node()) {
                            (Some(preceding), Some(following)) => {
                                // Only make it a trailing comment if it directly follows the preceding node but not if it is separated
                                // by one or more tokens
                                // ```javascript
                                // a /* comment */ b;   //  Comment is a trailing comment
                                // a, /* comment */ b;  // Comment should be a leading comment
                                // ```
                                if preceding.text_range().end()
                                    == comment.piece().as_piece().token().text_range().end()
                                {
                                    self.push_trailing_comment(&preceding, comment);
                                } else {
                                    self.push_leading_comment(&following, comment);
                                }
                            }
                            (Some(preceding), None) => {
                                self.push_trailing_comment(&preceding, comment);
                            }
                            (None, Some(following)) => {
                                self.push_leading_comment(&following, comment);
                            }
                            (None, None) => {
                                self.push_dangling_comment(
                                    &comment.enclosing_node().clone(),
                                    comment,
                                );
                            }
                        }
                    }
                }
            }
        }
    }

    fn mark_has_skipped(&mut self, token: &SyntaxToken<L>) {
        self.skipped.insert(token.key());
    }

    fn push_leading_comment(&mut self, node: &SyntaxNode<L>, comment: DecoratedComment<L>) {
        self.comments.push_leading(node.key(), comment.into());
    }

    fn push_dangling_comment(&mut self, node: &SyntaxNode<L>, comment: DecoratedComment<L>) {
        self.comments.push_dangling(node.key(), comment.into());
    }

    fn push_trailing_comment(&mut self, node: &SyntaxNode<L>, comment: DecoratedComment<L>) {
        self.comments.push_trailing(node.key(), comment.into());
    }

    fn finish(
        self,
    ) -> (
        CommentsMap<SyntaxElementKey, SourceComment<L>>,
        FxHashSet<SyntaxElementKey>,
    ) {
        (self.comments, self.skipped)
    }
}

impl<L: Language> Default for CommentsBuilder<L> {
    fn default() -> Self {
        Self {
            comments: CommentsMap::new(),
            skipped: FxHashSet::default(),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::comments::builder::CommentsBuilderVisitor;
    use crate::comments::map::CommentsMap;
    use crate::comments::CommentPosition;
    use crate::{CommentKind, CommentPlacement, CommentStyle, DecoratedComment, SourceComment};
    use rome_js_parser::parse_module;
    use rome_js_syntax::{
        JsLanguage, JsParameters, JsPropertyObjectMember, JsShorthandPropertyObjectMember,
        JsSyntaxKind, JsSyntaxNode,
    };
    use rome_rowan::syntax::SyntaxElementKey;
    use rome_rowan::{AstNode, SyntaxTriviaPieceComments};
    use std::cell::RefCell;

    #[test]
    fn leading_comment() {
        let (root, decorated, comments) = extract_comments(
            r#"const foo = {
  a: 'a',
  /* comment for this line */
  b
};"#,
        );

        assert_eq!(decorated.len(), 1);

        let comment = decorated.last().unwrap();

        assert_eq!(comment.position(), CommentPosition::OwnLine);
        assert_eq!(comment.lines_before(), 1);
        assert_eq!(comment.lines_after(), 1);
        assert_eq!(
            comment
                .preceding_node()
                .map(|node| node.text_trimmed().to_string())
                .as_deref(),
            Some("a: 'a'")
        );
        assert_eq!(
            comment
                .following_node()
                .map(|node| node.text_trimmed().to_string())
                .as_deref(),
            Some("b")
        );
        assert_eq!(
            comment.enclosing_node().kind(),
            JsSyntaxKind::JS_OBJECT_EXPRESSION
        );

        let b = root
            .descendants()
            .find_map(JsShorthandPropertyObjectMember::cast)
            .unwrap();

        assert!(!comments.leading(&b.syntax().key()).is_empty());
    }

    #[test]
    fn trailing_comment() {
        let (root, decorated, comments) = extract_comments(
            r#"const foo = {
  a: 'a' /* comment for this line */,
  b
};"#,
        );

        assert_eq!(decorated.len(), 1);

        let comment = decorated.last().unwrap();

        assert_eq!(comment.position(), CommentPosition::SameLine);
        assert_eq!(comment.lines_after(), 0);
        assert_eq!(comment.lines_before(), 0);
        assert_eq!(
            comment
                .preceding_node()
                .map(|node| node.text_trimmed().to_string())
                .as_deref(),
            Some("a: 'a'")
        );
        assert_eq!(
            comment
                .following_node()
                .map(|node| node.text_trimmed().to_string())
                .as_deref(),
            Some("b")
        );
        assert_eq!(
            comment.enclosing_node().kind(),
            JsSyntaxKind::JS_OBJECT_EXPRESSION
        );

        let a = root
            .descendants()
            .find_map(JsPropertyObjectMember::cast)
            .unwrap();

        assert!(!comments.trailing(&a.syntax().key()).is_empty());
    }

    #[test]
    fn end_of_line_comment() {
        let (root, decorated, comments) = extract_comments(
            r#"const foo = {
  a: 'a', /* comment for this line */
  b
};"#,
        );

        assert_eq!(decorated.len(), 1);

        let comment = decorated.last().unwrap();

        assert_eq!(comment.position(), CommentPosition::EndOfLine);
        assert_eq!(comment.lines_before(), 0);
        assert_eq!(comment.lines_after(), 1);
        assert_eq!(
            comment
                .preceding_node()
                .map(|node| node.text_trimmed().to_string())
                .as_deref(),
            Some("a: 'a'")
        );
        assert_eq!(
            comment
                .following_node()
                .map(|node| node.text_trimmed().to_string())
                .as_deref(),
            Some("b")
        );
        assert_eq!(
            comment.enclosing_node().kind(),
            JsSyntaxKind::JS_OBJECT_EXPRESSION
        );

        let a = root
            .descendants()
            .find_map(JsPropertyObjectMember::cast)
            .unwrap();

        assert!(!comments.trailing(&a.syntax().key()).is_empty());
    }

    #[test]
    fn dangling_arrow() {
        let (root, decorated_comments, comments) = extract_comments("(/* comment */)  => true");

        assert_eq!(decorated_comments.len(), 1);

        let decorated = &decorated_comments[0];
        assert_eq!(decorated.position(), CommentPosition::SameLine);
        assert_eq!(decorated.lines_before(), 0);
        assert_eq!(decorated.lines_after(), 0);
        assert_eq!(decorated.preceding_node(), None);
        assert_eq!(decorated.following_node(), None);
        assert_eq!(
            decorated.enclosing_node().kind(),
            JsSyntaxKind::JS_PARAMETERS
        );

        let parameters = root.descendants().find_map(JsParameters::cast).unwrap();
        assert!(!comments.dangling(&parameters.syntax().key()).is_empty());
    }

    #[test]
    fn dangling_comments() {
        let (root, decorated_comments, comments) = extract_comments(
            r#"
            function (/* test */) {}
            "#,
        );

        assert_eq!(decorated_comments.len(), 1);

        let decorated = &decorated_comments[0];
        assert_eq!(decorated.position(), CommentPosition::SameLine);
        assert_eq!(decorated.lines_before(), 0);
        assert_eq!(decorated.lines_after(), 0);
        assert_eq!(decorated.preceding_node(), None);
        assert_eq!(decorated.following_node(), None);
        assert_eq!(
            decorated.enclosing_node().kind(),
            JsSyntaxKind::JS_PARAMETERS
        );

        let parameters = root.descendants().find_map(JsParameters::cast).unwrap();
        assert!(!comments.dangling(&parameters.syntax().key()).is_empty());
    }

    #[test]
    fn comment_only_program() {
        let (root, decorated, comments) = extract_comments(
            r#"/* test */

/* test */"#,
        );

        assert_eq!(decorated.len(), 2);

        let first = &decorated[0];
        assert_eq!(first.position(), CommentPosition::OwnLine);
        assert_eq!(first.lines_before(), 0);
        assert_eq!(first.lines_after(), 2);
        assert_eq!(first.preceding_node(), None);
        assert_eq!(first.following_node(), None);
        assert_eq!(first.enclosing_node().kind(), JsSyntaxKind::JS_MODULE);

        let second = &decorated[1];
        assert_eq!(second.position(), CommentPosition::OwnLine);
        assert_eq!(second.lines_before(), 2);
        assert_eq!(second.lines_after(), 0);
        assert_eq!(second.preceding_node(), None);
        assert_eq!(second.following_node(), None);
        assert_eq!(second.enclosing_node().kind(), JsSyntaxKind::JS_MODULE);

        assert!(!comments.dangling(&root.key()).is_empty());
    }

    fn extract_comments(
        source: &str,
    ) -> (
        JsSyntaxNode,
        Vec<DecoratedComment<JsLanguage>>,
        CommentsMap<SyntaxElementKey, SourceComment<JsLanguage>>,
    ) {
        let tree = parse_module(source, 0);

        let style = TestCommentStyle::default();
        let builder = CommentsBuilderVisitor::new(&style);
        let (comments, _) = builder.visit(&tree.syntax());

        (tree.syntax(), style.finish(), comments)
    }

    #[derive(Default)]
    struct TestCommentStyle {
        recorded_comments: RefCell<Vec<DecoratedComment<JsLanguage>>>,
    }

    impl CommentStyle for TestCommentStyle {
        type Language = JsLanguage;

        fn is_suppression(_: &str) -> bool {
            false
        }

        fn get_comment_kind(_: &SyntaxTriviaPieceComments<Self::Language>) -> CommentKind {
            CommentKind::Block
        }

        fn place_comment(
            &self,
            comment: DecoratedComment<Self::Language>,
        ) -> CommentPlacement<Self::Language> {
            self.recorded_comments.borrow_mut().push(comment.clone());

            CommentPlacement::Default(comment)
        }
    }

    impl TestCommentStyle {
        fn finish(self) -> Vec<DecoratedComment<JsLanguage>> {
            self.recorded_comments.take()
        }
    }
}
