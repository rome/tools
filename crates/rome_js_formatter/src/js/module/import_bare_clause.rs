use crate::format_traits::FormatOptional;
use rome_formatter::FormatResult;

use crate::{formatted, space_token, Format, FormatElement, FormatNode, Formatter};

use rome_js_syntax::JsImportBareClause;
use rome_js_syntax::JsImportBareClauseFields;

impl FormatNode for JsImportBareClause {
    fn format_fields(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        let JsImportBareClauseFields { source, assertion } = self.as_fields();

        let source = source.format(formatter)?;
        let assertion =
            assertion.with_or_empty(|assertion| formatted![formatter, space_token(), assertion]);

        formatted![formatter, source, assertion]
    }
}
