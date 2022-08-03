use crate::js::bindings::parameters::ParameterLayout;
use crate::js::lists::parameter_list::{AnyParameterList, FormatParameterList};
use crate::prelude::*;
use rome_js_syntax::{JsConstructorParameterList};

#[derive(Debug, Clone, Default)]
pub struct FormatJsConstructorParameterList;

impl FormatRule<JsConstructorParameterList> for FormatJsConstructorParameterList {
    type Context = JsFormatContext;

    fn fmt(&self, node: &JsConstructorParameterList, f: &mut JsFormatter) -> FormatResult<()> {
        FormatParameterList::with_layout(
            &AnyParameterList::from(node.clone()),
            ParameterLayout::Default,
        )
        .fmt(f)
    }
}
