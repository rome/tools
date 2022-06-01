use crate::prelude::*;
use crate::utils::{format_with_semicolon, FormatMemberName};
use crate::FormatNodeFields;
use rome_js_syntax::{TsMethodSignatureClassMember, TsMethodSignatureClassMemberFields};

impl FormatNodeFields<TsMethodSignatureClassMember>
    for FormatNodeRule<TsMethodSignatureClassMember>
{
    fn format_fields(
        node: &TsMethodSignatureClassMember,
        formatter: &JsFormatter,
    ) -> FormatResult<FormatElement> {
        let TsMethodSignatureClassMemberFields {
            modifiers,
            async_token,
            name,
            question_mark_token,
            type_parameters,
            parameters,
            return_type_annotation,
            semicolon_token,
        } = node.as_fields();

        format_with_semicolon(
            formatter,
            formatted![
                formatter,
                [
                    modifiers.format(),
                    async_token
                        .format()
                        .with_or_empty(|token| formatted![formatter, [token, space_token()]]),
                    space_token(),
                    FormatMemberName::from(name?).format(formatter),
                    question_mark_token.format(),
                    type_parameters.format(),
                    parameters.format(),
                    return_type_annotation.format(),
                ]
            ]?,
            semicolon_token,
        )
    }
}
