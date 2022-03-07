use crate::formatter_traits::{FormatOptionalTokenAndNode, FormatTokenAndNode};

use crate::{
    format_elements, soft_line_break_or_space, space_token, FormatElement, FormatResult, Formatter,
    ToFormatElement,
};

use rome_js_syntax::JsNamedImportSpecifier;
use rome_js_syntax::JsNamedImportSpecifierFields;

impl ToFormatElement for JsNamedImportSpecifier {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        let JsNamedImportSpecifierFields {
            type_token,
            name,
            as_token,
            local_name,
        } = self.as_fields();

        let type_token = type_token
            .format_with_or_empty(formatter, |token| format_elements![token, space_token()])?;

        let name = name.format(formatter)?;
        let as_token = as_token.format(formatter)?;
        let local_name = local_name.format(formatter)?;

        Ok(format_elements![
            type_token,
            name,
            soft_line_break_or_space(),
            as_token,
            soft_line_break_or_space(),
            local_name
        ])
    }
}
