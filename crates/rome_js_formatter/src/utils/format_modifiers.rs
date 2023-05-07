use crate::prelude::*;
use crate::utils::sort_modifiers_by_precedence;
use crate::{AsFormat, IntoFormat};
use rome_formatter::{format_args, write};
use rome_js_syntax::JsSyntaxKind::JS_DECORATOR;
use rome_js_syntax::{JsLanguage, Modifiers};
use rome_rowan::{AstNode, AstNodeList, NodeOrToken};

pub(crate) struct FormatModifiers<List> {
    pub(crate) list: List,
}

impl<List> FormatModifiers<List> {
    pub(crate) fn from(list: List) -> Self {
        Self { list }
    }
}

impl<List, Node> Format<JsFormatContext> for FormatModifiers<List>
where
    Node: AstNode<Language = JsLanguage> + AsFormat<JsFormatContext> + IntoFormat<JsFormatContext>,
    List: AstNodeList<Language = JsLanguage, Node = Node>,
    Modifiers: for<'a> From<&'a Node>,
{
    fn fmt(&self, f: &mut Formatter<JsFormatContext>) -> FormatResult<()> {
        let modifiers = sort_modifiers_by_precedence(&self.list);
        let should_expand = should_expand_decorators(&self.list);

        // need to use peek the iterator to check if the current node is a decorator and don't advance the iterator
        let mut iter = modifiers.into_iter().peekable();
        let decorators = format_once(|f| {
            let mut join = f.join_nodes_with_soft_line();

            // join only decorators here
            while let Some(node) = iter.peek() {
                // check if the current node is a decorator
                match node.syntax().kind() {
                    JS_DECORATOR => {
                        join.entry(node.syntax(), &node.format());
                        // advance the iterator
                        iter.next();
                    }
                    _ => {
                        // if we encounter a non-decorator we break out of the loop
                        break;
                    }
                }
            }

            join.finish()
        });

        write!(
            f,
            [group(&format_args![decorators, soft_line_break_or_space()])
                .should_expand(should_expand)]
        )?;

        // join the rest of the modifiers
        f.join_with(&space()).entries(iter.formatted()).finish()
    }
}

/// This function expands decorators enclosing a group if there is a newline between decorators or after the last decorator.
fn should_expand_decorators<List, Node>(list: &List) -> bool
where
    Node: AstNode<Language = JsLanguage>,
    List: AstNodeList<Language = JsLanguage, Node = Node>,
{
    // we need to skip the first node because we look for newlines between decorators or after the last decorator
    for node in list.iter().skip(1) {
        match node.syntax().kind() {
            JS_DECORATOR => {
                if node.syntax().has_leading_newline() {
                    return true;
                }
            }
            _ => {
                // if we encounter a non-decorator with a leading newline after a decorator and the next modifier
                return node.syntax().has_leading_newline();
            }
        }
    }

    // if we encounter a non-decorator with a leading newline after a decorator and the next node or token
    list.syntax_list()
        .node()
        .next_sibling_or_token()
        .map_or(false, |node| match node {
            NodeOrToken::Node(node) => node.has_leading_newline(),
            NodeOrToken::Token(token) => token.has_leading_newline(),
        })
}
