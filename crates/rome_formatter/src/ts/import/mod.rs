mod any_import_assertion_entry;
mod any_import_clause;
mod any_named_import;
mod any_named_import_specifier;
mod assertion;
mod assertion_entry;
mod bare_clause;
mod default_import_specifier;
mod import_call_expression;
mod import_default_clause;
mod literal_export_name;
mod module_source;
mod named_clause;
mod named_import_specifier;
mod named_import_specifiers;
mod shorthand_named_import_specifier;

use crate::{
    format_elements, space_token, token, FormatElement, FormatResult, Formatter, ToFormatElement,
};
use rslint_parser::ast::JsImport;

impl ToFormatElement for JsImport {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        let import_token = formatter.format_token(&self.import_token()?)?;
        let import_clause = formatter.format_node(self.import_clause()?)?;
        let semicolon = formatter
            .format_token(&self.semicolon_token())?
            .unwrap_or_else(|| token(';'));

        Ok(format_elements![
            import_token,
            space_token(),
            import_clause,
            semicolon
        ])
    }
}
