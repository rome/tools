use crate::generated::FormatTsPropertyParameterModifierList;
use crate::prelude::*;
use crate::utils::sort_modifiers_by_precedence;
use rome_js_syntax::TsPropertyParameterModifierList;

impl FormatRule<TsPropertyParameterModifierList> for FormatTsPropertyParameterModifierList {
    type Context = JsFormatContext;

    fn fmt(node: &TsPropertyParameterModifierList, f: &mut JsFormatter) -> FormatResult<()> {
        f.join_with(&space_token())
            .entries(sort_modifiers_by_precedence(node).into_iter().formatted())
            .finish()
    }
}
