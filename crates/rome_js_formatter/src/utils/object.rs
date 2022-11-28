use crate::prelude::*;
use crate::utils::FormatLiteralStringToken;
use crate::utils::StringLiteralParentKind;
use rome_formatter::write;
use rome_js_syntax::JsSyntaxKind::JS_STRING_LITERAL;
use rome_js_syntax::{AnyJsClassMemberName, AnyJsObjectMemberName};
use rome_rowan::{declare_node_union, AstNode};
use unicode_width::UnicodeWidthStr;

declare_node_union! {
    pub(crate) AnyJsMemberName = AnyJsObjectMemberName | AnyJsClassMemberName
}

impl Format<JsFormatContext> for AnyJsMemberName {
    fn fmt(&self, f: &mut Formatter<JsFormatContext>) -> FormatResult<()> {
        match self {
            AnyJsMemberName::AnyJsObjectMemberName(node) => {
                write!(f, [node.format()])
            }
            AnyJsMemberName::AnyJsClassMemberName(node) => {
                write!(f, [node.format()])
            }
        }
    }
}

pub(crate) fn write_member_name(
    name: &AnyJsMemberName,
    f: &mut JsFormatter,
) -> FormatResult<usize> {
    match name {
        name @ AnyJsMemberName::AnyJsClassMemberName(AnyJsClassMemberName::JsLiteralMemberName(
            literal,
        ))
        | name @ AnyJsMemberName::AnyJsObjectMemberName(
            AnyJsObjectMemberName::JsLiteralMemberName(literal),
        ) => {
            let value = literal.value()?;

            if value.kind() == JS_STRING_LITERAL {
                let format = FormatLiteralStringToken::new(&value, StringLiteralParentKind::Member);
                let cleaned = format.clean_text(f.options());

                write!(
                    f,
                    [
                        format_leading_comments(name.syntax()),
                        cleaned,
                        format_trailing_comments(name.syntax())
                    ]
                )?;

                Ok(cleaned.width())
            } else {
                write!(f, [name])?;

                Ok(value.text_trimmed().width())
            }
        }
        name => {
            write!(f, [&name])?;
            Ok(name.text().width())
        }
    }
}
