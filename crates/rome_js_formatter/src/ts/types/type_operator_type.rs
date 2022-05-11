use crate::prelude::*;
use rome_js_syntax::TsTypeOperatorType;

impl FormatNode for TsTypeOperatorType {
    fn format_fields(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        formatted![
            formatter,
            self.operator_token()
                .with(|operator| { formatted![formatter, operator, space_token()] }),
            self.ty().format(formatter)?
        ]
    }
}
