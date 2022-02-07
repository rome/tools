use crate::formatter_traits::FormatTokenAndNode;
use crate::{
    format_elements, group_elements, join_elements, soft_block_indent, soft_line_break_or_space,
    token, FormatElement, FormatResult, Formatter, ToFormatElement,
};
use rslint_parser::ast::{
    JsAnyArrayAssignmentPatternElement, JsArrayAssignmentPattern,
    JsArrayAssignmentPatternRestElement,
};

impl ToFormatElement for JsArrayAssignmentPattern {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        let elements = formatter.format_separated_list(self.elements(), || token(","))?;
        Ok(group_elements(formatter.format_delimited(
            &self.l_brack_token()?,
            |open_token_trailing, close_token_leading| {
                Ok(soft_block_indent(format_elements![
                    open_token_trailing,
                    join_elements(soft_line_break_or_space(), elements),
                    close_token_leading,
                ]))
            },
            &self.r_brack_token()?,
        )?))
    }
}

impl ToFormatElement for JsAnyArrayAssignmentPatternElement {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        match self {
            JsAnyArrayAssignmentPatternElement::JsAssignmentWithDefault(
                assignment_with_default,
            ) => assignment_with_default.to_format_element(formatter),
            JsAnyArrayAssignmentPatternElement::JsAnyAssignmentPattern(any_assignment_pattern) => {
                any_assignment_pattern.to_format_element(formatter)
            }
            JsAnyArrayAssignmentPatternElement::JsArrayAssignmentPatternRestElement(
                array_assignment_pattern_rest_element,
            ) => array_assignment_pattern_rest_element.to_format_element(formatter),
            JsAnyArrayAssignmentPatternElement::JsArrayHole(array_hole) => {
                array_hole.to_format_element(formatter)
            }
            JsAnyArrayAssignmentPatternElement::JsUnknownAssignment(unknown_assignment) => {
                unknown_assignment.to_format_element(formatter)
            }
        }
    }
}

impl ToFormatElement for JsArrayAssignmentPatternRestElement {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        Ok(format_elements![
            self.dotdotdot_token().format(formatter)?,
            self.pattern().format(formatter)?
        ])
    }
}
