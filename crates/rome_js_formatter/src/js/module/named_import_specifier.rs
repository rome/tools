use crate::prelude::*;

use rome_js_syntax::JsNamedImportSpecifier;
use rome_js_syntax::JsNamedImportSpecifierFields;

impl FormatNode for JsNamedImportSpecifier {
    fn format_fields(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        let JsNamedImportSpecifierFields {
            type_token,
            name,
            as_token,
            local_name,
        } = self.as_fields();

        let type_token =
            type_token.with_or_empty(|token| formatted![formatter, token, space_token()]);

        let name = name.format(formatter)?;
        let as_token = as_token.format(formatter)?;
        let local_name = local_name.format(formatter)?;

        formatted![
            formatter,
            type_token,
            name,
            soft_line_break_or_space(),
            as_token,
            soft_line_break_or_space(),
            local_name
        ]
    }
}
