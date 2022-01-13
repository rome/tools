use rslint_parser::ast::{JsAnyForInOrOfInitializer, JsForInStatement, JsForVariableDeclaration};

use crate::{
    format_elements, group_elements, soft_indent, soft_line_break_or_space, space_token,
    FormatElement, FormatResult, Formatter, ToFormatElement,
};

impl ToFormatElement for JsForInStatement {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        let for_token = formatter.format_token(&self.for_token()?)?;
        let initializer = formatter.format_node(self.initializer()?)?;
        let in_token = formatter.format_token(&self.in_token()?)?;
        let expression = formatter.format_node(self.expression()?)?;
        let body = formatter.format_node(self.body()?)?;

        Ok(format_elements![
            for_token,
            space_token(),
            formatter.format_delimited_group(
                &self.l_paren_token()?,
                |leading, trailing| Ok(group_elements(soft_indent(format_elements![
                    leading,
                    initializer,
                    soft_line_break_or_space(),
                    in_token,
                    soft_line_break_or_space(),
                    expression,
                    trailing,
                ]))),
                &self.r_paren_token()?
            )?,
            space_token(),
            body
        ])
    }
}

impl ToFormatElement for JsAnyForInOrOfInitializer {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        match self {
            JsAnyForInOrOfInitializer::JsAnyAssignmentPattern(assignment) => {
                assignment.to_format_element(formatter)
            }
            JsAnyForInOrOfInitializer::JsForVariableDeclaration(decl) => {
                decl.to_format_element(formatter)
            }
        }
    }
}

impl ToFormatElement for JsForVariableDeclaration {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        Ok(format_elements![
            formatter.format_token(&self.kind_token()?)?,
            space_token(),
            formatter.format_node(self.declaration()?)?
        ])
    }
}
