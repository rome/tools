use crate::{Format, FormatElement, FormatNode, Formatter};
use rome_formatter::FormatResult;

use rome_js_syntax::JsSuperExpression;
use rome_js_syntax::JsSuperExpressionFields;

impl FormatNode for JsSuperExpression {
    fn format_fields(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        let JsSuperExpressionFields { super_token } = self.as_fields();

        super_token.format(formatter)
    }
}
