use crate::prelude::*;
use rome_js_syntax::TsDefaultTypeClause;

impl FormatNode for TsDefaultTypeClause {
    fn format_fields(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        let equals = self.eq_token().format(formatter)?;
        let ty = self.ty().format(formatter)?;
        formatted![formatter, equals, space_token(), ty]
    }
}
