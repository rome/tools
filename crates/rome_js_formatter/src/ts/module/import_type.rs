use crate::formatter_traits::{FormatOptionalTokenAndNode, FormatTokenAndNode};
use crate::utils::format_string_literal_token;
use crate::{
    format_elements, space_token, FormatElement, FormatResult, Formatter, ToFormatElement,
};
use rome_js_syntax::TsImportType;
use rome_js_syntax::TsImportTypeFields;

impl ToFormatElement for TsImportType {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        let TsImportTypeFields {
            typeof_token,
            import_token,
            l_paren_token,
            argument_token,
            r_paren_token,
            qualifier_clause,
            type_arguments,
        } = self.as_fields();

        Ok(format_elements![
            typeof_token
                .format_with_or_empty(formatter, |token| format_elements![token, space_token()])?,
            import_token.format(formatter)?,
            l_paren_token.format(formatter)?,
            format_string_literal_token(argument_token?, formatter),
            r_paren_token.format(formatter)?,
            qualifier_clause.format_or_empty(formatter)?,
            type_arguments.format_or_empty(formatter)?,
        ])
    }
}
