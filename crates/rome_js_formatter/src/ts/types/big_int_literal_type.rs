use std::borrow::Cow;

use crate::prelude::*;

use crate::parentheses::NeedsParentheses;
use crate::utils::string_utils::ToAsciiLowercaseCow;
use rome_formatter::write;
use rome_js_syntax::{JsSyntaxNode, TsBigIntLiteralType, TsBigIntLiteralTypeFields};

#[derive(Debug, Clone, Default)]
pub struct FormatTsBigIntLiteralType;

impl FormatNodeRule<TsBigIntLiteralType> for FormatTsBigIntLiteralType {
    fn fmt_fields(&self, node: &TsBigIntLiteralType, f: &mut JsFormatter) -> FormatResult<()> {
        let TsBigIntLiteralTypeFields {
            minus_token,
            literal_token,
        } = node.as_fields();
        write![f, [minus_token.format()]]?;
        let literal_token = literal_token?;

        let original = literal_token.text_trimmed();
        match original.to_ascii_lowercase_cow() {
            Cow::Borrowed(_) => write![f, [literal_token.format()]],
            Cow::Owned(lowercase) => {
                write!(
                    f,
                    [format_replaced(
                        &literal_token,
                        &dynamic_text(&lowercase, literal_token.text_trimmed_range().start())
                    )]
                )
            }
        }
    }

    fn needs_parentheses(&self, item: &TsBigIntLiteralType) -> bool {
        item.needs_parentheses()
    }
}

impl NeedsParentheses for TsBigIntLiteralType {
    fn needs_parentheses_with_parent(&self, _parent: &JsSyntaxNode) -> bool {
        false
    }
}
