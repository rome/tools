use crate::formatter_traits::FormatTokenAndNode;
use crate::{
    format_elements, group_elements, join_elements, soft_block_indent, soft_line_break_or_space,
    token, FormatElement, FormatResult, Formatter, ToFormatElement,
};
use rslint_parser::ast::JsCallArguments;
use rslint_parser::AstSeparatedList;

impl ToFormatElement for JsCallArguments {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        let args_len = self.args().len();
        let mut args_iterator = self.args().elements();
        if let Some(first_arg) = args_iterator.next() {
            let args_tokens =
                formatter.format_separated_list(args_iterator, args_len - 1, || token(","))?;

            let node = first_arg.node()?.format(formatter)?;

            let first_delimiter = if args_len == 1 {
                empty_element()
            } else {
                first_arg
                    .trailing_separator()?
                    .map(|t| t.format(formatter))
                    .unwrap_or_else(|| Ok(token(",")))?
            };

            Ok(group_elements(formatter.format_delimited(
                &self.l_paren_token()?,
                |open_token_trailing, close_token_leading| {
                    Ok(format_elements![
                        group_elements(format_elements![
                            open_token_trailing,
                            node,
                            first_delimiter
                        ]),
                        soft_block_indent(format_elements![
                            join_elements(soft_line_break_or_space(), args_tokens),
                            close_token_leading
                        ])
                    ])
                },
                &self.r_paren_token()?,
            )?))
        } else {
            // Arguments are empty
            Ok(format_elements![
                self.l_paren_token()?.format(formatter)?,
                self.r_paren_token()?.format(formatter)?
            ])
        }
    }
}
