use crate::prelude::*;

use crate::parentheses::NeedsParentheses;
use rome_formatter::token::FormatLeadingComments;
use rome_formatter::{format_args, write};
use rome_js_syntax::{JsSyntaxNode, TsMappedType, TsMappedTypeFields};

#[derive(Debug, Clone, Default)]
pub struct FormatTsMappedType;

impl FormatNodeRule<TsMappedType> for FormatTsMappedType {
    fn fmt_fields(&self, node: &TsMappedType, f: &mut JsFormatter) -> FormatResult<()> {
        let TsMappedTypeFields {
            l_curly_token,
            readonly_modifier,
            l_brack_token,
            property_name,
            in_token,
            keys_type,
            as_clause,
            r_brack_token,
            optional_modifier,
            mapped_type,
            semicolon_token,
            r_curly_token,
        } = node.as_fields();

        let property_name = property_name?;

        let should_expand = node
            .syntax()
            .tokens()
            .flat_map(|token| {
                token
                    .leading_trivia()
                    .pieces()
                    .chain(token.trailing_trivia().pieces())
            })
            .any(|piece| piece.is_newline());

        let format_semi = format_with(|f| {
            if let Some(semi) = &semicolon_token {
                write!(f, [semi.format()])
            } else {
                write!(f, [text(";")])
            }
        });

        let comments = f.comments().clone();
        let dangling_comments = comments.dangling_comments(node.syntax());
        let type_annotation_has_leading_comment =
            mapped_type.as_ref().map_or(false, |annotation| {
                comments.has_leading_comments(annotation.syntax())
            });

        write!(
            f,
            [
                &l_curly_token.format(),
                group(&indent(&format_args!(
                    soft_line_break_or_space(),
                    readonly_modifier
                        .format()
                        .with_or_empty(|readonly, f| write![f, [readonly, space()]]),
                    FormatLeadingComments::Comments(dangling_comments),
                    group(&format_args![
                        l_brack_token.format(),
                        property_name.format(),
                        space(),
                        in_token.format(),
                        soft_line_indent_or_space(&format_args![
                            keys_type.format(),
                            as_clause.as_ref().map(|_| space()),
                            as_clause.format(),
                        ]),
                        r_brack_token.format(),
                    ]),
                    optional_modifier.format(),
                    type_annotation_has_leading_comment.then_some(space()),
                    mapped_type.format(),
                    if_group_breaks(&format_semi)
                )))
                .should_expand(should_expand),
                soft_line_break_or_space(),
                r_curly_token.format(),
            ]
        )
    }

    fn needs_parentheses(&self, item: &TsMappedType) -> bool {
        item.needs_parentheses()
    }

    fn fmt_dangling_comments(&self, _: &TsMappedType, _: &mut JsFormatter) -> FormatResult<()> {
        // Handled inside of `fmt_fields`
        Ok(())
    }
}

impl NeedsParentheses for TsMappedType {
    fn needs_parentheses_with_parent(&self, _parent: &JsSyntaxNode) -> bool {
        false
    }
}
