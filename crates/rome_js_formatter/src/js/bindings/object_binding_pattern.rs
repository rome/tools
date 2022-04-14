use crate::{
    formatter_traits::FormatTokenAndNode, FormatElement, FormatResult, Formatter, ToFormatElement,
};
use rome_js_syntax::{JsAnyObjectBindingPatternMember, JsObjectBindingPatternFields};
use rome_js_syntax::{JsObjectBindingPattern, JsSyntaxKind};
use rome_rowan::{AstNode, SyntaxResult};

impl ToFormatElement for JsObjectBindingPattern {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        let JsObjectBindingPatternFields {
            l_curly_token,
            properties,
            r_curly_token,
        } = self.as_fields();

        if should_hard_break(self)? {
            formatter.format_delimited_block_indent(
                &l_curly_token?,
                properties.format(formatter)?,
                &r_curly_token?,
            )
        } else {
            formatter.format_delimited_soft_block_spaces(
                &l_curly_token?,
                properties.format(formatter)?,
                &r_curly_token?,
            )
        }
    }
}

/// This function inspects the properties of a [JsObjectBindingPattern] and decides that should break when:
/// - the node is inside a [JsVariableDeclaratorNode]
/// - at least one of the properties is a [JsObjectBindingPatternProperty] or a [JsObjectBindingPatternShorthandProperty] with an
/// initializer
///
/// [JsObjectBindingPattern]: rome_js_syntax::JsObjectBindingPattern
/// [JsVariableDeclaratorNode]: rome_js_syntax::JsVariableDeclaratorNode
/// [JsObjectBindingPatternProperty]: rome_js_syntax::JsObjectBindingPatternProperty
/// [JsObjectBindingPatternShorthandProperty]: rome_js_syntax::JsObjectBindingPatternShorthandProperty
fn should_hard_break(binding_pattern: &JsObjectBindingPattern) -> SyntaxResult<bool> {
    let properties = binding_pattern.properties();
    let parent_kind = binding_pattern.syntax().parent().map(|k| k.kind());

    if let Some(parent_kind) = parent_kind {
        if parent_kind != JsSyntaxKind::JS_VARIABLE_DECLARATOR {
            return Ok(false);
        }
    }

    for property in properties {
        let property = property?;

        if let JsAnyObjectBindingPatternMember::JsObjectBindingPatternProperty(_) = property {
            return Ok(true);
        } else if let JsAnyObjectBindingPatternMember::JsObjectBindingPatternShorthandProperty(
            short_hand_property,
        ) = property
        {
            if short_hand_property.init().is_some() {
                return Ok(true);
            }
        }
    }
    Ok(false)
}
