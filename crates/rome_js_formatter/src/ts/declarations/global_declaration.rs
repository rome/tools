use crate::{
    format_elements, space_token, Format, FormatElement, FormatNode, FormatResult, Formatter,
};
use rome_js_syntax::TsGlobalDeclaration;
use rome_js_syntax::TsGlobalDeclarationFields;

impl FormatNode for TsGlobalDeclaration {
    fn format_fields(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        let TsGlobalDeclarationFields { global_token, body } = self.as_fields();

        Ok(format_elements![
            global_token.format(formatter)?,
            space_token(),
            body.format(formatter)?
        ])
    }
}
