use crate::prelude::*;
use crate::FormatNodeFields;
use rome_js_syntax::TsUnionType;
use rome_js_syntax::TsUnionTypeFields;

impl FormatNodeFields<TsUnionType> for FormatNodeRule<TsUnionType> {
    fn format_fields(node: &TsUnionType, formatter: &JsFormatter) -> FormatResult<FormatElement> {
        let TsUnionTypeFields {
            leading_separator_token,
            types,
        } = node.as_fields();

        let leading_separator_token = match leading_separator_token {
            Some(token) => {
                // The SyntaxToken is converted into a FormatElement using
                // Token::from to strip the token's trivia pieces which are
                // then reinserted informat_replaced outside of the
                // if_group_breaks block to avoid removing comments when the
                // group does not break
                let replaced =
                    if_group_breaks(format_elements![Token::from(&token), space_token()]);
                formatter.format_replaced(&token, replaced)
            }
            None => if_group_breaks(format_elements![token("|"), space_token()]),
        };

        let types = formatted![formatter, [types.format()]]?;

        // Push trailing comments for the union out of the group (and indent block),
        // so any potential line break doesn't influence the formatting of the type itself
        let (leading_comments, types, trailing_comments) = types.split_trivia();

        formatted![
            formatter,
            [
                group_elements(indent(formatted![
                    formatter,
                    [
                        soft_line_break(),
                        leading_separator_token,
                        leading_comments,
                        types,
                    ]
                ]?)),
                trailing_comments
            ]
        ]
    }
}
