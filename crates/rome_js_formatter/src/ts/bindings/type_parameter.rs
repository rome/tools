use crate::prelude::*;

use rome_formatter::write;
use rome_js_syntax::{TsTypeParameter, TsTypeParameterFields};

#[derive(Debug, Clone, Default)]
pub struct FormatTsTypeParameter;

impl FormatNodeRule<TsTypeParameter> for FormatTsTypeParameter {
    fn fmt_fields(&self, node: &TsTypeParameter, f: &mut JsFormatter) -> FormatResult<()> {
        let TsTypeParameterFields {
            name,
            constraint,
            default,
            in_modifier_token,
            out_modifier_token,
        } = node.as_fields();

        if let Some(in_modifier_token) = in_modifier_token {
            write!(f, [in_modifier_token.format(), space()])?;
        }

        if let Some(out_modifier_token) = out_modifier_token {
            write!(f, [out_modifier_token.format(), space()])?;
        }
        write!(f, [name.format()])?;
        if let Some(constraint) = constraint {
            write!(f, [space(), constraint.format()])?;
        }

        if let Some(default) = default {
            write!(f, [space(), default.format()])?;
        }

        Ok(())
    }
}
