use crate::{Format, FormatElement, FormatNode, Formatter};
use rome_formatter::FormatResult;
use rome_js_syntax::TsAbstractModifier;
use rome_js_syntax::TsAbstractModifierFields;

impl FormatNode for TsAbstractModifier {
    fn format_fields(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        let TsAbstractModifierFields { modifier_token } = self.as_fields();

        modifier_token.format(formatter)
    }
}
