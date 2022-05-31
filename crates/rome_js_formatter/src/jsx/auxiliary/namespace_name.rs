use crate::prelude::*;
use crate::FormatNodeFields;
use rome_js_syntax::{JsxNamespaceName, JsxNamespaceNameFields};

impl FormatNodeFields<JsxNamespaceName> for FormatNodeRule<JsxNamespaceName> {
    fn format_fields(
        node: &JsxNamespaceName,
        formatter: &JsFormatter,
    ) -> FormatResult<FormatElement> {
        let JsxNamespaceNameFields {
            namespace,
            colon_token,
            name,
        } = node.as_fields();

        formatted![
            formatter,
            [namespace.format(), colon_token.format(), name.format()]
        ]
    }
}
