use crate::{space_token, Format, FormatElement, FormatNode, Formatter};
use rome_formatter::FormatResult;
use rome_js_syntax::TsTypeofType;

impl FormatNode for TsTypeofType {
    fn format_fields(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        let r#typeof = self.typeof_token().format(formatter)?;
        let expression_name = self.expression_name().format(formatter)?;
        formatted![formatter, r#typeof, space_token(), expression_name]
    }
}
