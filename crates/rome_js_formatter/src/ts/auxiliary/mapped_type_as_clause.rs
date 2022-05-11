use crate::prelude::*;
use rome_js_syntax::TsMappedTypeAsClause;

impl FormatNode for TsMappedTypeAsClause {
    fn format_fields(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        formatted![
            formatter,
            self.as_token()
                .with(|as_token| { formatted![formatter, as_token, space_token()] }),
            self.ty().format(formatter)?
        ]
    }
}
