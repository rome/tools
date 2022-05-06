use crate::format_traits::FormatOptional;
use rome_formatter::FormatResult;

use crate::{
    formatted, space_token, Format, FormatElement, FormatNode, Formatter,
};

use rome_js_syntax::JsImportDefaultClause;
use rome_js_syntax::JsImportDefaultClauseFields;

impl FormatNode for JsImportDefaultClause {
    fn format_fields(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        let JsImportDefaultClauseFields {
            type_token,
            local_name,
            from_token,
            source,
            assertion,
        } = self.as_fields();

        let type_token = type_token.format_with_or_empty(formatter, |token| {
            formatted![formatter, token, space_token()]
        })?;
        let local_name = local_name.format(formatter)?;
        let from = from_token.format(formatter)?;
        let source = source.format(formatter)?;
        let assertion = assertion.format_with_or_empty(formatter, |assertion| {
            formatted![formatter, space_token(), assertion]
        })?;

        formatted![
            formatter,
            type_token,
            local_name,
            space_token(),
            from,
            space_token(),
            source,
            assertion
        ]
    }
}
