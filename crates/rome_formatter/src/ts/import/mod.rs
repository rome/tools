mod any_import_assertion_entry;
mod any_import_clause;
mod assertion;
mod assertion_entry;
mod bare_clause;
mod default_import_specifier;
mod import;
mod module_source;
mod named_clause;

use crate::{format_elements, FormatElement, FormatResult, Formatter, ToFormatElement};
use rslint_parser::ast::JsImportCallExpression;

impl ToFormatElement for JsImportCallExpression {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        Ok(format_elements![
            formatter.format_token(&self.import_token()?)?,
            formatter.format_node(&self.arguments()?)?,
        ])
    }
}
