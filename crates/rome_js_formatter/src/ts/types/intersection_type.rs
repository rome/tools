use crate::prelude::*;

use rome_formatter::{format_args, write};
use rome_js_syntax::{JsSyntaxKind, TsIntersectionTypeFields};
use rome_js_syntax::{JsSyntaxToken, TsIntersectionType};

#[derive(Debug, Clone, Default)]
pub struct FormatTsIntersectionType;

impl FormatNodeRule<TsIntersectionType> for FormatTsIntersectionType {
    fn fmt_fields(&self, node: &TsIntersectionType, f: &mut JsFormatter) -> FormatResult<()> {
        let TsIntersectionTypeFields {
            leading_separator_token,
            types,
        } = node.as_fields();

        write!(
            f,
            [group(&indent(&format_args!(
                soft_line_break(),
                FormatTypeSetLeadingSeparator {
                    separator: JsSyntaxKind::AMP,
                    leading_separator: leading_separator_token.as_ref()
                },
                types.format()
            )))]
        )
    }
}

pub struct FormatTypeSetLeadingSeparator<'a> {
    pub(crate) separator: JsSyntaxKind,
    pub(crate) leading_separator: Option<&'a JsSyntaxToken>,
}

impl Format<JsFormatContext> for FormatTypeSetLeadingSeparator<'_> {
    fn fmt(&self, f: &mut JsFormatter) -> FormatResult<()> {
        match &self.leading_separator {
            Some(token) => {
                format_only_if_breaks(token, &format_args!(token.format(), space())).fmt(f)
            }
            None => write!(
                f,
                [if_group_breaks(&format_args![
                    format_inserted(self.separator),
                    space()
                ])]
            ),
        }
    }
}
