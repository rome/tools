use crate::{
    empty_element, format_elements, group_elements, soft_indent, space_token, FormatElement,
    FormatResult, Formatter, ToFormatElement,
};
use rslint_parser::ast::{JsElseClause, JsIfStatement};

impl ToFormatElement for JsIfStatement {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        let formatted_else_clause = if let Some(else_clause) = self.else_clause() {
            format_elements![space_token(), formatter.format_node(else_clause)?]
        } else {
            empty_element()
        };

        Ok(format_elements![
            group_elements(format_elements![
                formatter.format_token(&self.if_token()?)?,
                space_token(),
                group_elements(formatter.format_delimited_group(
                    &self.l_paren_token()?,
                    |leading, trailing| Ok(soft_indent(format_elements![
                        leading,
                        formatter.format_node(self.test()?)?,
                        trailing
                    ])),
                    &self.r_paren_token()?,
                )?),
                space_token(),
            ]),
            formatter.format_node(self.consequent()?)?,
            formatted_else_clause
        ])
    }
}

impl ToFormatElement for JsElseClause {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        Ok(format_elements![
            formatter.format_token(&self.else_token()?)?,
            space_token(),
            formatter.format_node(self.alternate()?)?,
        ])
    }
}
