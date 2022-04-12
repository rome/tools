use crate::formatter_traits::FormatTokenAndNode;
use crate::{FormatElement, FormatResult, Formatter, ToFormatElement};
use rome_formatter::format_elements;
use rome_js_syntax::{JsxNamespaceName, JsxNamespaceNameFields};

impl ToFormatElement for JsxNamespaceName {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        let JsxNamespaceNameFields {
            namespace,
            colon_token,
            name,
        } = self.as_fields();

        Ok(format_elements![
            namespace.format(formatter)?,
            colon_token.format(formatter)?,
            name.format(formatter)?
        ])
    }
}
