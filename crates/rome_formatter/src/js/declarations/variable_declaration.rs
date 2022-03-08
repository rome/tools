use crate::formatter_traits::FormatTokenAndNode;

use crate::{
    format_elements, space_token, FormatElement, FormatResult, Formatter, ToFormatElement,
};

use rome_js_syntax::JsVariableDeclaration;
use rome_js_syntax::JsVariableDeclarationFields;

impl ToFormatElement for JsVariableDeclaration {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        let JsVariableDeclarationFields { kind, declarators } = self.as_fields();

        Ok(format_elements![
            kind.format(formatter)?,
            space_token(),
            declarators.format(formatter)?,
        ])
    }
}
