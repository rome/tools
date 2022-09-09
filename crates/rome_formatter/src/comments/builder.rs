use crate::comments::multimap::AppendOnlyMultiMap;
use crate::comments::{CommentPosition, CommentsData};
use crate::{CommentPlacement, CommentStyle, DecoratedComment, SourceComment};
use rome_rowan::syntax::SyntaxElementKey;
use rome_rowan::{
    Direction, Language, SyntaxElement, SyntaxKind, SyntaxNode, SyntaxToken, WalkEvent,
};
use std::cell::RefCell;
use std::collections::HashSet;

pub(super) struct CommentsBuilderVisitor<'a, Style: CommentStyle> {
    builder: CommentsBuilder<Style::Language>,
    style: &'a Style,

    // State
    pending_comments: Vec<DecoratedComment<Style::Language>>,
    preceding_node: Option<SyntaxNode<Style::Language>>,
    following_node: Option<SyntaxNode<Style::Language>>,
    enclosing_node: Option<SyntaxNode<Style::Language>>,
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
            following_node: Default::default(),
            enclosing_node: Default::default(),
            last_token: Default::default(),
        }
    }

    pub(super) fn visit(
        mut self,
        root: &SyntaxNode<Style::Language>,
    ) -> CommentsData<Style::Language> {
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

        self.finish()
    }

    fn visit_node(&mut self, event: WalkEvent<SyntaxNode<Style::Language>>) {
        match event {
            WalkEvent::Enter(node) => {
                // Lists cannot have comments attached. They either belong to the entire parent or to
                // the first child. So we ignore lists all together
                if node.kind().is_list() {
                    return;
                }

                let is_root = self.enclosing_node.is_none();

                if is_root {
                    self.enclosing_node = Some(node);
                }
                // Associate comments with the most outer node
                // Set following here because it is the "following node" of the next token's leading trivia.
                else if self.following_node.is_none() {
                    // Flush in case the node doesn't have any tokens.
                    self.flush_comments(Some(&node));
                    self.following_node = Some(node);
                }
            }

            WalkEvent::Leave(node) => {
                if node.kind().is_list() {
                    return;
                }

                // We're passed this node, flush any pending comments for its children
                self.following_node = None;
                self.flush_comments(None);

                self.enclosing_node = node.parent();

                while let Some(enclosing) = &self.enclosing_node {
                    if enclosing.kind().is_list() {
                        self.enclosing_node = enclosing.parent();
                    } else {
                        break;
                    }
                }

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
                        kind: self.style.get_comment_kind(&comment),
                        comment,
                    });

                    lines_before = 0;
                }
            }
        }

        let trailing_end = trailing_end.unwrap_or(self.pending_comments.len());

        let mut has_skipped = false;

        // Process the leading trivia of the current token. the trailing trivia is handled as part of the next token
        for leading in token.leading_trivia().pieces() {
            if leading.is_newline() {
                lines_before += 1;
            } else if leading.is_skipped() {
                self.builder.mark_has_skipped(&token);

                lines_before = 0;
                has_skipped = true;
            } else if let Some(comment) = leading.as_comments() {
                let kind = self.style.get_comment_kind(&comment);
                if !has_skipped {
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
                }
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
        if let Some(following_node) = self.following_node.take() {
            self.flush_comments(Some(&following_node.clone()));
            self.enclosing_node = Some(following_node);

            // The following node is only set after entering a node
            // That means, following node is only set for the first token of a node.
            // Unset preceding node if this is the first token because the preceding node belongs to the parent.
            self.preceding_node = None;
        }
    }

    fn enclosing_node(&self) -> &SyntaxNode<Style::Language> {
        self.enclosing_node
            .as_ref()
            .expect("Expected enclosing nodes to at least contain the root node.")
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

    fn finish(mut self) -> CommentsData<Style::Language> {
        self.flush_comments(None);

        self.builder.finish(Style::is_suppression)
    }
}

struct CommentsBuilder<L: Language> {
    leading_comments: AppendOnlyMultiMap<SyntaxNode<L>, SourceComment<L>>,
    dangling_comments: AppendOnlyMultiMap<SyntaxNode<L>, SourceComment<L>>,
    trailing_comments: AppendOnlyMultiMap<SyntaxNode<L>, SourceComment<L>>,
    skipped: HashSet<SyntaxElementKey>,
}

impl<L: Language> CommentsBuilder<L> {
    fn add_comment(&mut self, placement: CommentPlacement<L>) {
        match placement {
            CommentPlacement::Leading { node, comment } => {
                self.insert_leading_comment(node, comment.into());
            }
            CommentPlacement::Trailing { node, comment } => {
                self.insert_trailing_comment(node, comment.into());
            }
            CommentPlacement::Dangling { node, comment } => {
                self.insert_dangling_comment(node, comment.into())
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
                                self.insert_trailing_comment(preceding, comment.into());
                            }
                            (Some(preceding), None) => {
                                self.insert_trailing_comment(preceding, comment.into());
                            }
                            (None, Some(following)) => {
                                self.insert_leading_comment(following, comment.into());
                            }
                            (None, None) => {
                                self.insert_dangling_comment(
                                    comment.enclosing_node().clone(),
                                    comment.into(),
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
                                self.insert_leading_comment(following, comment.into());
                            }
                            (Some(preceding), None) => {
                                self.insert_trailing_comment(preceding, comment.into());
                            }
                            (None, None) => {
                                self.insert_dangling_comment(
                                    comment.enclosing_node().clone(),
                                    comment.into(),
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
                                    self.insert_trailing_comment(preceding, comment.into());
                                } else {
                                    self.insert_leading_comment(following, comment.into());
                                }
                            }
                            (Some(preceding), None) => {
                                self.insert_trailing_comment(preceding, comment.into());
                            }
                            (None, Some(following)) => {
                                self.insert_leading_comment(following, comment.into());
                            }
                            (None, None) => {
                                self.insert_dangling_comment(
                                    comment.enclosing_node().clone(),
                                    comment.into(),
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

    fn insert_leading_comment(&mut self, node: SyntaxNode<L>, comment: SourceComment<L>) {
        self.leading_comments.append(node, comment);
    }

    fn insert_dangling_comment(&mut self, node: SyntaxNode<L>, comment: SourceComment<L>) {
        self.dangling_comments.append(node, comment);
    }

    fn insert_trailing_comment(&mut self, node: SyntaxNode<L>, comment: SourceComment<L>) {
        self.trailing_comments.append(node, comment);
    }

    fn finish(self, is_suppression: fn(&str) -> bool) -> CommentsData<L> {
        CommentsData {
            is_suppression,
            leading_comments: self.leading_comments,
            dangling_comments: self.dangling_comments,
            trailing_comments: self.trailing_comments,
            with_skipped: self.skipped,
            #[cfg(debug_assertions)]
            checked_suppressions: RefCell::new(Default::default()),
        }
    }
}

impl<L: Language> Default for CommentsBuilder<L> {
    fn default() -> Self {
        Self {
            leading_comments: AppendOnlyMultiMap::new(),
            dangling_comments: AppendOnlyMultiMap::new(),
            trailing_comments: AppendOnlyMultiMap::new(),
            skipped: HashSet::default(),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::comments::builder::CommentsBuilderVisitor;
    use crate::comments::{CommentPosition, CommentsData};
    use crate::{CommentKind, CommentPlacement, CommentStyle, Comments, DecoratedComment};
    use rome_js_parser::parse_module;
    use rome_js_syntax::{JsLanguage, JsSyntaxKind, JsSyntaxToken};
    use rome_rowan::SyntaxTriviaPieceComments;
    use std::cell::RefCell;
    use std::rc::Rc;

    #[test]
    fn leading_comment() {
        let (decorated, comments) = extract_comments(
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

        assert_eq!(
            comments
                .leading_comments
                .keys()
                .map(|node| node.text_trimmed().to_string())
                .collect::<Vec<_>>(),
            vec![String::from("b")]
        )
    }

    #[test]
    fn trailing_comment() {
        let (decorated, comments) = extract_comments(
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

        assert_eq!(
            comments
                .trailing_comments
                .keys()
                .map(|node| node.text_trimmed().to_string())
                .collect::<Vec<_>>(),
            vec![String::from("a: 'a'")]
        )
    }

    #[test]
    fn end_of_line_comment() {
        let (decorated, comments) = extract_comments(
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

        assert_eq!(
            comments
                .trailing_comments
                .keys()
                .map(|node| node.text_trimmed().to_string())
                .collect::<Vec<_>>(),
            vec![String::from("a: 'a'")]
        )
    }

    #[test]
    fn dangling_comments() {
        let (decorated_comments, comments) = extract_comments(
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

        assert_eq!(
            comments
                .dangling_comments
                .keys()
                .map(|node| node.kind())
                .collect::<Vec<_>>(),
            vec![JsSyntaxKind::JS_PARAMETERS]
        );
    }

    #[test]
    fn comment_only_program() {
        let (decorated, comments) = extract_comments(
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

        assert_eq!(
            comments
                .dangling_comments
                .keys()
                .map(|node| node.kind())
                .collect::<Vec<_>>(),
            vec![JsSyntaxKind::JS_MODULE]
        );
    }

    fn extract_comments(
        source: &str,
    ) -> (Vec<DecoratedComment<JsLanguage>>, CommentsData<JsLanguage>) {
        let tree = parse_module(source, 0);

        let style = TestCommentStyle::default();
        let builder = CommentsBuilderVisitor::new(&style);
        let comments = builder.visit(&tree.syntax());

        (style.finish(), comments)
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

        fn get_comment_kind(&self, _: &SyntaxTriviaPieceComments<Self::Language>) -> CommentKind {
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
