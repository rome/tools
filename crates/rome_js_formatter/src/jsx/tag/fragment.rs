use crate::prelude::*;
use rome_js_syntax::{JsxFragment, JsxFragmentFields};

impl FormatNode for JsxFragment {
    fn format_fields(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        let JsxFragmentFields {
            opening_fragment,
            children,
            closing_fragment,
        } = self.as_fields();

        let children = children.format(formatter)?;
        Ok(format_elements![
            opening_fragment.format(formatter)?,
            children,
            closing_fragment.format(formatter)?
        ])
    }
}
