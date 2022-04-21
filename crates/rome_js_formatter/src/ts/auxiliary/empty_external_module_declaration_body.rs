use crate::{Format, FormatElement, FormatNode, Formatter};
use rome_formatter::FormatResult;
use rome_js_syntax::TsEmptyExternalModuleDeclarationBody;
use rome_js_syntax::TsEmptyExternalModuleDeclarationBodyFields;

impl FormatNode for TsEmptyExternalModuleDeclarationBody {
    fn format_fields(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        let TsEmptyExternalModuleDeclarationBodyFields { semicolon_token } = self.as_fields();
        semicolon_token.format(formatter)
    }
}
