use crate::prelude::*;

use crate::FormatNodeFields;
use rome_js_syntax::JsMethodObjectMember;
use rome_js_syntax::JsMethodObjectMemberFields;

impl FormatNodeFields<JsMethodObjectMember> for FormatNodeRule<JsMethodObjectMember> {
    fn format_fields(node: &JsMethodObjectMember, f: &mut JsFormatter) -> FormatResult<()> {
        let JsMethodObjectMemberFields {
            async_token,
            star_token,
            name,
            type_parameters,
            parameters,
            return_type_annotation,
            body,
        } = node.as_fields();

        write![
            f,
            [
                async_token
                    .format()
                    .with_or_empty(|async_token, f| write![f, [async_token, space_token()]]),
                star_token.format(),
                name.format(),
                type_parameters.format(),
                parameters.format(),
                return_type_annotation.format(),
                space_token(),
                body.format(),
            ]
        ]
    }
}
