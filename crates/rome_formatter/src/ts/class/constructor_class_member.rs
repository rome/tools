use crate::{
    format_elements, group_elements, join_elements, soft_indent, soft_line_break_or_space,
    space_token, token, FormatElement, FormatResult, Formatter, ToFormatElement,
};
use rslint_parser::ast::{
    JsAnyConstructorParameter, JsConstructorClassMember, JsConstructorParameters,
};
use rslint_parser::AstNode;

impl ToFormatElement for JsConstructorClassMember {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        Ok(format_elements![
            formatter.format_node(self.name()?)?,
            formatter.format_node(self.parameters()?)?,
            space_token(),
            formatter.format_node(self.body()?)?
        ])
    }
}

impl ToFormatElement for JsConstructorParameters {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        let params = formatter.format_separated(self.parameters(), || token(","))?;

        Ok(group_elements(formatter.format_delimited(
            &self.l_paren_token()?,
            |leading, trailing| {
                Ok(soft_indent(format_elements![
                    leading,
                    join_elements(soft_line_break_or_space(), params),
                    trailing,
                ]))
            },
            &self.r_paren_token()?,
        )?))
    }
}

impl ToFormatElement for JsAnyConstructorParameter {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        match self {
            JsAnyConstructorParameter::TsConstructorParam(_) => todo!(),
            JsAnyConstructorParameter::JsParameter(parameter) => {
                parameter.to_format_element(formatter)
            }
            JsAnyConstructorParameter::JsUnknownParameter(unknown_parameter) => {
                Ok(formatter.format_verbatim(unknown_parameter.syntax()))
            }
        }
    }
}
