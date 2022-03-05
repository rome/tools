use crate::formatter_traits::FormatTokenAndNode;
use crate::{FormatElement, FormatResult, Formatter, ToFormatElement};
use rslint_syntax::TsEmptyExternalModuleDeclarationBody;
use rslint_syntax::TsEmptyExternalModuleDeclarationBodyFields;

impl ToFormatElement for TsEmptyExternalModuleDeclarationBody {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        let TsEmptyExternalModuleDeclarationBodyFields { semicolon_token } = self.as_fields();
        semicolon_token.format(formatter)
    }
}
