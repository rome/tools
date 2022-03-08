use crate::formatter_traits::{FormatOptionalTokenAndNode, FormatTokenAndNode};

use crate::{
    format_elements, space_token, FormatElement, FormatResult, Formatter, ToFormatElement,
};

use rome_js_syntax::JsObjectBindingPatternShorthandProperty;
use rome_js_syntax::JsObjectBindingPatternShorthandPropertyFields;

impl ToFormatElement for JsObjectBindingPatternShorthandProperty {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        let JsObjectBindingPatternShorthandPropertyFields { identifier, init } = self.as_fields();

        let init_node =
            init.format_with_or_empty(formatter, |node| format_elements![space_token(), node])?;
        Ok(format_elements![identifier.format(formatter)?, init_node])
    }
}
