use crate::{
    format_elements, space_token, Format, FormatElement, FormatNode, FormatResult, Formatter,
};
use rome_js_syntax::TsDeclareStatement;
use rome_js_syntax::TsDeclareStatementFields;

impl FormatNode for TsDeclareStatement {
    fn format_fields(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        let TsDeclareStatementFields {
            declaration,
            declare_token,
        } = self.as_fields();
        Ok(format_elements![
            declare_token.format(formatter)?,
            space_token(),
            declaration.format(formatter)?
        ])
    }
}
