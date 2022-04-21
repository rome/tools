use crate::{Format, FormatElement, FormatNode, FormatResult, Formatter};
use rome_formatter::format_elements;
use rome_js_syntax::{JsxNamespaceName, JsxNamespaceNameFields};

impl FormatNode for JsxNamespaceName {
    fn format_fields(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
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
