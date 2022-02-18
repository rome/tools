use crate::{
    formatter_traits::FormatTokenAndNode, FormatElement, FormatResult, Formatter, ToFormatElement,
};

use rslint_parser::ast::JsCallArguments;
use rslint_parser::ast::JsCallArgumentsFields;

impl ToFormatElement for JsCallArguments {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        let JsCallArgumentsFields {
            l_paren_token,
            args,
            r_paren_token,
        } = self.as_fields();

        formatter.format_delimited_soft_block_indent(
            &l_paren_token?,
            args.format(formatter)?,
            &r_paren_token?,
        )
    }
}
