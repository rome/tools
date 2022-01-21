use crate::{
    empty_element, format_elements, group_elements, join_elements, soft_indent,
    soft_line_break_or_space, space_token, token, FormatElement, FormatResult, Formatter,
    ToFormatElement,
};
use rslint_parser::ast::{JsAnyParameter, JsParameter, JsParameters, JsRestParameter};
use rslint_parser::AstNode;

impl ToFormatElement for JsParameters {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        let param_tokens = formatter.format_separated(self.items(), || token(","))?;

        Ok(group_elements(formatter.format_delimited(
            &self.l_paren_token()?,
            |leading, trailing| {
                Ok(soft_indent(format_elements![
                    leading,
                    join_elements(soft_line_break_or_space(), param_tokens),
                    trailing,
                ]))
            },
            &self.r_paren_token()?,
        )?))
    }
}

impl ToFormatElement for JsAnyParameter {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        match self {
            JsAnyParameter::JsParameter(parameter) => parameter.to_format_element(formatter),
            JsAnyParameter::JsUnknownParameter(unknown_parameter) => {
                Ok(formatter.format_verbatim(unknown_parameter.syntax()))
            }
            JsAnyParameter::JsRestParameter(binding) => binding.to_format_element(formatter),
        }
    }
}

impl ToFormatElement for JsRestParameter {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        Ok(format_elements![
            formatter.format_token(&self.dotdotdot_token()?)?,
            formatter.format_node(self.binding()?)?
        ])
    }
}

impl ToFormatElement for JsParameter {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        let initializer = if let Some(initializer) = self.initializer() {
            format_elements![space_token(), formatter.format_node(initializer)?]
        } else {
            empty_element()
        };

        Ok(format_elements![
            formatter.format_node(self.binding()?)?,
            initializer
        ])
    }
}
