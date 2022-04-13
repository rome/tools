use crate::formatter_traits::{FormatOptionalTokenAndNode, FormatTokenAndNode};
use crate::{format_elements, FormatElement, FormatResult, Formatter, ToFormatElement};
use rome_js_syntax::{JsxAttribute, JsxAttributeFields};

impl ToFormatElement for JsxAttribute {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        let JsxAttributeFields { name, initializer } = self.as_fields();

        Ok(format_elements![
            name.format(formatter)?,
            initializer.format_or_empty(formatter)?
        ])
    }
}
