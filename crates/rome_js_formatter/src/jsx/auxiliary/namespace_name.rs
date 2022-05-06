use crate::{formatted, Format, FormatElement, FormatNode, Formatter};
use rome_formatter::{FormatResult};
use rome_js_syntax::{JsxNamespaceName, JsxNamespaceNameFields};

impl FormatNode for JsxNamespaceName {
    fn format_fields(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        let JsxNamespaceNameFields {
            namespace,
            colon_token,
            name,
        } = self.as_fields();

        formatted![
            formatter,
            namespace.format(formatter)?,
            colon_token.format(formatter)?,
            name.format(formatter)?
        ]
    }
}
