use crate::parentheses::NeedsParentheses;
use crate::prelude::*;
use crate::ts::types::intersection_type::{
    union_or_intersection_type_needs_parentheses, FormatTypeSetLeadingSeparator,
    TsIntersectionOrUnionTypeList,
};
use rome_formatter::{format_args, write, Buffer};
use rome_js_syntax::{JsSyntaxKind, TsUnionType};
use rome_js_syntax::{JsSyntaxNode, TsUnionTypeFields};

#[derive(Debug, Clone, Default)]
pub struct FormatTsUnionType;

impl FormatNodeRule<TsUnionType> for FormatTsUnionType {
    fn fmt_fields(&self, node: &TsUnionType, f: &mut JsFormatter) -> FormatResult<()> {
        let TsUnionTypeFields {
            leading_separator_token,
            types,
        } = node.as_fields();

        let body = format_with(|f| {
            write!(
                f,
                [
                    soft_line_break(),
                    FormatTypeSetLeadingSeparator {
                        separator: JsSyntaxKind::PIPE,
                        leading_separator: leading_separator_token.as_ref()
                    },
                    types.format()
                ]
            )
        });

        if node.needs_parentheses() {
            return write!(f, [group(&format_args![indent(&body), soft_line_break()])]);
        }

        let should_indent = {
            let parent_kind = node.syntax().parent().map(|p| p.kind());

            !matches!(
                parent_kind,
                Some(
                    JsSyntaxKind::TS_REFERENCE_TYPE
                        | JsSyntaxKind::TS_TYPE_ASSERTION_EXPRESSION
                        | JsSyntaxKind::TS_TUPLE_TYPE
                        | JsSyntaxKind::TS_TYPE_ASSERTION_ASSIGNMENT
                        | JsSyntaxKind::TS_FUNCTION_TYPE
                        | JsSyntaxKind::TS_TYPE_ARGUMENTS
                )
            )
        };

        write![
            f,
            [group(&format_with(|f| {
                if should_indent {
                    write!(f, [&indent(&body)])
                } else {
                    write!(f, [&body])
                }
            }))]
        ]
    }

    fn needs_parentheses(&self, item: &TsUnionType) -> bool {
        item.needs_parentheses()
    }
}

impl NeedsParentheses for TsUnionType {
    fn needs_parentheses_with_parent(&self, parent: &JsSyntaxNode) -> bool {
        union_or_intersection_type_needs_parentheses(
            self.syntax(),
            parent,
            &TsIntersectionOrUnionTypeList::TsUnionTypeVariantList(self.types()),
        )
    }
}
