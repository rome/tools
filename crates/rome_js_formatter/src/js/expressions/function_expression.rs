use crate::prelude::*;

use rome_js_syntax::{JsAnyFunction, JsFunctionExpression};

impl FormatNode for JsFunctionExpression {
    fn format_fields(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        JsAnyFunction::from(self.clone()).format(formatter)
    }
}
