use crate::prelude::*;
use rome_formatter::write;

use crate::FormatNodeFields;
use rome_js_syntax::JsMethodObjectMember;
use rome_js_syntax::JsMethodObjectMemberFields;

impl FormatNodeFields<JsMethodObjectMember> for FormatNodeRule<JsMethodObjectMember> {
    fn fmt_fields(node: &JsMethodObjectMember, f: &mut JsFormatter) -> FormatResult<()> {
        let JsMethodObjectMemberFields {
            async_token,
            star_token,
            name,
            type_parameters,
            parameters,
            return_type_annotation,
            body,
        } = node.as_fields();

        if let Some(async_token) = async_token {
            write!(f, [async_token.format(), space_token()])?;
        }

        write![
            f,
            [
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
