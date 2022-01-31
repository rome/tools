mod any_export_clause;
mod any_export_name_specifier;
mod as_clause;
mod class_clause;
mod default_class_clause;
mod default_expression_clause;
mod default_function_clause;
mod from_clause;
mod function_clause;
mod named_clause;
mod named_from_clause;
mod named_from_specifier;
mod named_shorthand_specifier;
mod named_specifier;
mod variable_clause;

use crate::{
    format_elements, space_token, FormatElement, FormatResult, Formatter, ToFormatElement,
};
use rslint_parser::ast::JsExport;

impl ToFormatElement for JsExport {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        let export_token = formatter.format_token(&self.export_token()?)?;
        let export_clause = formatter.format_node(&self.export_clause()?)?;
        Ok(format_elements![export_token, space_token(), export_clause])
    }
}
