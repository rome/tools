use crate::prelude::*;
use crate::FormatNodeFields;
use rome_formatter::write;
use rome_js_syntax::{TsIndexSignatureParameter, TsIndexSignatureParameterFields};

impl FormatNodeFields<TsIndexSignatureParameter> for FormatNodeRule<TsIndexSignatureParameter> {
    fn fmt_fields(node: &TsIndexSignatureParameter, f: &mut JsFormatter) -> FormatResult<()> {
        let TsIndexSignatureParameterFields {
            binding,
            type_annotation,
        } = node.as_fields();
        let binding = binding.format();
        let type_annotation = type_annotation.format();

        write![f, [binding, type_annotation]]
    }
}
