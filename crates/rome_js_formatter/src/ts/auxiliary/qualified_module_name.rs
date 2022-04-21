use crate::{format_elements, Format, FormatElement, FormatNode, FormatResult, Formatter};
use rome_js_syntax::TsQualifiedModuleName;
use rome_js_syntax::TsQualifiedModuleNameFields;

impl FormatNode for TsQualifiedModuleName {
    fn format_fields(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        let TsQualifiedModuleNameFields {
            left,
            dot_token,
            right,
        } = self.as_fields();

        Ok(format_elements![
            left.format(formatter)?,
            dot_token.format(formatter)?,
            right.format(formatter)?,
        ])
    }
}
