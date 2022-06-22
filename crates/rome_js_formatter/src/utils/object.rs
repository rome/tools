use crate::prelude::*;

use crate::utils::FormatLiteralStringToken;
use crate::utils::StringLiteralParentKind;
use rome_formatter::{write, VecBuffer};
use rome_js_syntax::JsAnyObjectMemberName;
use rome_js_syntax::JsSyntaxKind::JS_STRING_LITERAL;
use rome_rowan::AstNode;
use unicode_width::UnicodeWidthStr;

pub(crate) fn write_member_name(
    name: &JsAnyObjectMemberName,
    buffer: &mut VecBuffer<JsFormatContext>,
) -> FormatResult<usize> {
    match name {
        name @ JsAnyObjectMemberName::JsLiteralMemberName(literal) => {
            let value = literal.value()?;

            if value.kind() == JS_STRING_LITERAL {
                let format = FormatLiteralStringToken::new(&value, StringLiteralParentKind::Member);
                let cleaned = format.clean_text(buffer.context());

                write!(buffer, [cleaned])?;

                Ok(cleaned.width())
            } else {
                write!(buffer, [name.format()])?;

                Ok(value.text_trimmed().width())
            }
        }
        name => {
            write!(buffer, [&name.format()])?;
            Ok(name.text().width())
        }
    }
}
