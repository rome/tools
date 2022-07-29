use crate::prelude::*;

use rome_formatter::write;
use rome_js_syntax::JsMethodClassMember;
use rome_js_syntax::JsMethodClassMemberFields;

#[derive(Debug, Clone, Default)]
pub struct FormatJsMethodClassMember;

impl FormatNodeRule<JsMethodClassMember> for FormatJsMethodClassMember {
    fn fmt_fields(&self, node: &JsMethodClassMember, f: &mut JsFormatter) -> FormatResult<()> {
        let JsMethodClassMemberFields {
            modifiers,
            async_token,
            star_token,
            name,
            question_mark_token,
            type_parameters,
            parameters,
            return_type_annotation,
            body,
        } = node.as_fields();

        write![f, [modifiers.format(), space(),]]?;

        if let Some(async_token) = async_token {
            write!(f, [async_token.format(), space()])?;
        }

        write!(
            f,
            [
                star_token.format(),
                name.format(),
                question_mark_token.format(),
                type_parameters.format(),
                parameters.format(),
                return_type_annotation.format(),
                space(),
                body.format()
            ]
        )
    }
}
