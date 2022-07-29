use crate::prelude::*;
use crate::ts::types::intersection_type::FormatTypeSetLeadingSeparator;
use rome_formatter::{write, Buffer};
use rome_js_syntax::TsUnionTypeFields;
use rome_js_syntax::{JsSyntaxKind, TsUnionType};

#[derive(Debug, Clone, Default)]
pub struct FormatTsUnionType;

impl FormatNodeRule<TsUnionType> for FormatTsUnionType {
    fn fmt_fields(&self, node: &TsUnionType, f: &mut JsFormatter) -> FormatResult<()> {
        let TsUnionTypeFields {
            leading_separator_token,
            types,
        } = node.as_fields();

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
}
