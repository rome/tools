use crate::prelude::*;
use rome_js_syntax::TsDeclareModifier;
use rome_js_syntax::TsDeclareModifierFields;

impl FormatNode for TsDeclareModifier {
    fn format_fields(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        let TsDeclareModifierFields { modifier_token } = self.as_fields();
        modifier_token.format(formatter)
    }
}
