use crate::prelude::*;
use crate::ts::types::intersection_type::FormatTypeSetLeadingSeparator;

use rome_formatter::{format_args, write, Buffer};
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

        write![
            f,
            [group_elements(&indent(&format_args![
                soft_line_break(),
                FormatTypeSetLeadingSeparator {
                    separator: JsSyntaxKind::PIPE,
                    leading_separator: leading_separator_token.as_ref()
                },
                types.format()
            ]))]
        ]
    }
}
