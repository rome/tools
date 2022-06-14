use crate::prelude::*;
use crate::FormatNodeFields;
use rome_formatter::write;
use rome_js_syntax::{JsxNamespaceName, JsxNamespaceNameFields};

impl FormatNodeFields<JsxNamespaceName> for FormatNodeRule<JsxNamespaceName> {
    fn fmt_fields(node: &JsxNamespaceName, f: &mut JsFormatter) -> FormatResult<()> {
        let JsxNamespaceNameFields {
            namespace,
            colon_token,
            name,
        } = node.as_fields();

        write![f, [namespace.format(), colon_token.format(), name.format()]]
    }
}
