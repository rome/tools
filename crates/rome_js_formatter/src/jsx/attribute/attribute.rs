use crate::format_traits::FormatOptional;
use crate::{format_elements, Format, FormatElement, FormatNode, FormatResult, Formatter};
use rome_js_syntax::{JsxAttribute, JsxAttributeFields};

impl FormatNode for JsxAttribute {
    fn format_fields(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        let JsxAttributeFields { name, initializer } = self.as_fields();

        Ok(format_elements![
            name.format(formatter)?,
            initializer.format_or_empty(formatter)?
        ])
    }
}
