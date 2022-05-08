use crate::{format_elements, format_traits::FormatOptional, FormatElement, FormatNode, Formatter};
use rome_formatter::FormatResult;
use rome_js_syntax::{TsTypeParameterModifier, TsTypeParameterModifierFields};

impl FormatNode for TsTypeParameterModifier {
    fn format_fields(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        let TsTypeParameterModifierFields {
            in_token,
            out_token,
        } = self.as_fields();
        Ok(format_elements![
            in_token.format_or_empty(formatter)?,
            out_token.format_or_empty(formatter)?
        ])
    }
}
