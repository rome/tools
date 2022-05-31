use crate::prelude::*;
use crate::FormatNodeFields;
use rome_js_syntax::TsIntersectionType;
use rome_js_syntax::TsIntersectionTypeFields;

impl FormatNodeFields<TsIntersectionType> for FormatNodeRule<TsIntersectionType> {
    fn format_fields(
        node: &TsIntersectionType,
        formatter: &JsFormatter,
    ) -> FormatResult<FormatElement> {
        let TsIntersectionTypeFields {
            leading_separator_token,
            types,
        } = node.as_fields();

        let leading_separator_token = match leading_separator_token {
            Some(token) => {
                // The SyntaxToken is converted into a FormatElement using
                // Token::from to strip the token's trivia pieces which are
                // then reinserted in format_replaced outside of the
                // if_group_breaks block to avoid removing comments when the
                // group does not break
                let replaced =
                    if_group_breaks(format_elements![Token::from(&token), space_token()]);
                formatter.format_replaced(&token, replaced)
            }
            None => if_group_breaks(format_elements![token("&"), space_token()]),
        };

        Ok(group_elements(indent(formatted![
            formatter,
            [soft_line_break(), leading_separator_token, types.format(),]
        ]?)))
    }
}
