use crate::prelude::*;
use crate::utils::{FormatMemberName, FormatWithSemicolon};
use crate::FormatNodeFields;
use rome_formatter::{format_args, write};
use rome_js_syntax::{TsMethodSignatureClassMember, TsMethodSignatureClassMemberFields};

impl FormatNodeFields<TsMethodSignatureClassMember>
    for FormatNodeRule<TsMethodSignatureClassMember>
{
    fn fmt_fields(node: &TsMethodSignatureClassMember, f: &mut JsFormatter) -> FormatResult<()> {
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

        write!(
            f,
            [FormatWithSemicolon::new(
                &format_args!(
                    modifiers.format(),
                    async_token
                        .format()
                        .with_or_empty(|token, f| write![f, [token, space_token()]]),
                    space_token(),
                    FormatMemberName::from(name?),
                    question_mark_token.format(),
                    type_parameters.format(),
                    parameters.format(),
                    return_type_annotation.format(),
                ),
                semicolon_token.as_ref()
            )]
        )
    }
}
