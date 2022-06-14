use crate::prelude::*;
use crate::FormatNodeFields;
use rome_formatter::write;
use rome_js_syntax::{TsTypeParameter, TsTypeParameterFields};

impl FormatNodeFields<TsTypeParameter> for FormatNodeRule<TsTypeParameter> {
    fn fmt_fields(node: &TsTypeParameter, f: &mut JsFormatter) -> FormatResult<()> {
        let TsTypeParameterFields {
            name,
            constraint,
            default,
            in_modifier_token,
            out_modifier_token,
        } = node.as_fields();

        if let Some(in_modifier_token) = in_modifier_token {
            write!(f, [in_modifier_token.format(), space_token()])?;
        }

        if let Some(out_modifier_token) = out_modifier_token {
            write!(f, [out_modifier_token.format(), space_token()])?;
        }
        write!(f, [name.format()])?;
        if let Some(constraint) = constraint {
            write!(f, [space_token(), constraint.format()])?;
        }

        if let Some(default) = default {
            write!(f, [space_token(), default.format()])?;
        }

        Ok(())
    }
}
