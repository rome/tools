use crate::prelude::*;

use crate::FormatNodeFields;
use rome_js_syntax::JsMethodClassMember;
use rome_js_syntax::JsMethodClassMemberFields;

impl FormatNodeFields<JsMethodClassMember> for FormatNodeRule<JsMethodClassMember> {
    fn format_fields(
        node: &JsMethodClassMember,
        formatter: &Formatter<JsFormatOptions>,
    ) -> FormatResult<FormatElement> {
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

        formatted![
            formatter,
            [
                modifiers.format(),
                space_token(),
                async_token
                    .format()
                    .with_or_empty(|token| formatted![formatter, [token, space_token()]]),
                star_token.format(),
                name.format(),
                question_mark_token.format(),
                type_parameters.format(),
                parameters.format(),
                return_type_annotation.format(),
                space_token(),
                body.format()
            ]
        ]
    }
}
