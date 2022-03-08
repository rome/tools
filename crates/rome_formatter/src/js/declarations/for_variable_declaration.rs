use rome_js_syntax::JsForVariableDeclaration;

use crate::formatter_traits::FormatTokenAndNode;

use crate::{
    format_elements, space_token, FormatElement, FormatResult, Formatter, ToFormatElement,
};
use rome_js_syntax::JsForVariableDeclarationFields;

impl ToFormatElement for JsForVariableDeclaration {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        let JsForVariableDeclarationFields {
            kind_token,
            declarator,
        } = self.as_fields();

        Ok(format_elements![
            kind_token.format(formatter)?,
            space_token(),
            declarator.format(formatter)?,
        ])
    }
}
