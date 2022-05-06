use crate::format_traits::FormatOptional;
use rome_formatter::FormatResult;

use crate::{
    formatted, space_token, Format, FormatElement, FormatNode, Formatter,
};

use rome_js_syntax::JsExtendsClause;
use rome_js_syntax::JsExtendsClauseFields;

impl FormatNode for JsExtendsClause {
    fn format_fields(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        let JsExtendsClauseFields {
            extends_token,
            super_class,
            type_arguments,
        } = self.as_fields();

        Ok(formatted![
            formatter,
            extends_token.format(formatter)?,
            space_token(),
            super_class.format(formatter)?,
            type_arguments.format_or_empty(formatter)?,
        ]?)
    }
}
