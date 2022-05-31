use crate::prelude::*;
use crate::FormatNodeFields;
use rome_js_syntax::{TsIndexSignatureParameter, TsIndexSignatureParameterFields};

impl FormatNodeFields<TsIndexSignatureParameter> for FormatNodeRule<TsIndexSignatureParameter> {
    fn format_fields(
        node: &TsIndexSignatureParameter,
        formatter: &JsFormatter,
    ) -> FormatResult<FormatElement> {
        let TsIndexSignatureParameterFields {
            binding,
            type_annotation,
        } = node.as_fields();
        let binding = binding.format();
        let type_annotation = type_annotation.format();

        formatted![formatter, [binding, type_annotation]]
    }
}
