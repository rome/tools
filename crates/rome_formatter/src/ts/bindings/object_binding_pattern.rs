use crate::formatter_traits::{FormatOptionalTokenAndNode, FormatTokenAndNode};
use crate::{
    format_elements, group_elements, join_elements, soft_block_indent, soft_line_break_or_space,
    space_token, token, FormatElement, FormatResult, Formatter, ToFormatElement,
};
use rslint_parser::ast::{
    JsAnyObjectBindingPatternMember, JsObjectBindingPattern, JsObjectBindingPatternProperty,
    JsObjectBindingPatternRest, JsObjectBindingPatternShorthandProperty,
};

impl ToFormatElement for JsObjectBindingPattern {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        let properties = formatter.format_separated_list(self.properties(), || token(","))?;

        Ok(group_elements(formatter.format_delimited(
            &self.l_curly_token()?,
            |open_token_trailing, close_token_leading| {
                Ok(format_elements![
                    space_token(),
                    soft_block_indent(format_elements![
                        open_token_trailing,
                        join_elements(soft_line_break_or_space(), properties),
                        close_token_leading,
                    ]),
                    space_token(),
                ])
            },
            &self.r_curly_token()?,
        )?))
    }
}

impl ToFormatElement for JsAnyObjectBindingPatternMember {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        match self {
            JsAnyObjectBindingPatternMember::JsObjectBindingPatternProperty(
                object_binding_pattern_property,
            ) => object_binding_pattern_property.to_format_element(formatter),
            JsAnyObjectBindingPatternMember::JsObjectBindingPatternRest(
                object_binding_pattern_rest,
            ) => object_binding_pattern_rest.to_format_element(formatter),
            JsAnyObjectBindingPatternMember::JsObjectBindingPatternShorthandProperty(
                object_binding_pattern_shorthand_property,
            ) => object_binding_pattern_shorthand_property.to_format_element(formatter),
            JsAnyObjectBindingPatternMember::JsIdentifierBinding(identifier_binding) => {
                identifier_binding.to_format_element(formatter)
            }
            JsAnyObjectBindingPatternMember::JsUnknownBinding(unknown_binding) => {
                unknown_binding.to_format_element(formatter)
            }
        }
    }
}

impl ToFormatElement for JsObjectBindingPatternProperty {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        let init_node = self
            .init()
            .format_with_or_empty(formatter, |node| format_elements![space_token(), node])?;
        Ok(format_elements![
            self.member().format(formatter)?,
            self.colon_token().format(formatter)?,
            space_token(),
            self.pattern().format(formatter)?,
            init_node,
        ])
    }
}

impl ToFormatElement for JsObjectBindingPatternRest {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        Ok(format_elements![
            self.dotdotdot_token().format(formatter)?,
            self.binding().format(formatter)?,
        ])
    }
}

impl ToFormatElement for JsObjectBindingPatternShorthandProperty {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        let init_node = self
            .init()
            .format_with_or_empty(formatter, |node| format_elements![space_token(), node])?;
        Ok(format_elements![
            self.identifier().format(formatter)?,
            init_node
        ])
    }
}
