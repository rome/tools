use crate::prelude::*;
use crate::utils::FormatMemberName;
use crate::FormatNodeFields;
use rome_formatter::write;
use rome_js_syntax::JsMethodClassMember;
use rome_js_syntax::JsMethodClassMemberFields;

impl FormatNodeFields<JsMethodClassMember> for FormatNodeRule<JsMethodClassMember> {
    fn fmt_fields(node: &JsMethodClassMember, f: &mut JsFormatter) -> FormatResult<()> {
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

        write![f, [modifiers.format(), space_token(),]]?;

        if let Some(async_token) = async_token {
            write!(f, [async_token.format(), space_token()])?;
        }

        write!(
            f,
            [
                star_token.format(),
                FormatMemberName::from(name?),
                question_mark_token.format(),
                type_parameters.format(),
                parameters.format(),
                return_type_annotation.format(),
                space_token(),
                body.format()
            ]
        )
    }
}
