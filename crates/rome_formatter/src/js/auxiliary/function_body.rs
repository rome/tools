use crate::{
    format_elements, formatter_traits::FormatTokenAndNode, FormatElement, FormatResult, Formatter,
    ToFormatElement,
};

use rome_js_syntax::JsFunctionBody;
use rome_js_syntax::JsFunctionBodyFields;

impl ToFormatElement for JsFunctionBody {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        let JsFunctionBodyFields {
            l_curly_token,
            directives,
            statements,
            r_curly_token,
        } = self.as_fields();

        formatter.format_delimited_block_indent(
            &l_curly_token?,
            format_elements![
                directives.format(formatter)?,
                formatter.format_list(statements),
            ],
            &r_curly_token?,
        )
    }
}
