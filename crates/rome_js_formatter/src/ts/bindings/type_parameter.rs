use crate::prelude::*;
use rome_js_syntax::{TsTypeParameter, TsTypeParameterFields};

impl FormatNode for TsTypeParameter {
    fn format_fields(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        let TsTypeParameterFields {
            name,
            constraint,
            default,
        } = self.as_fields();

        let name = name.format(formatter)?;
        let constraint =
            constraint.with_or_empty(|constraint| formatted![formatter, space_token(), constraint]);
        let default =
            default.with_or_empty(|default| formatted![formatter, space_token(), default]);
        formatted![formatter, name, constraint, default]
    }
}
