use crate::prelude::*;

use rome_js_syntax::{JsAnyClass, JsClassDeclaration};

impl FormatNode for JsClassDeclaration {
    fn format_fields(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        JsAnyClass::from(self.clone()).format(formatter)
    }
}
