use crate::utils::format_string_literal_token;
use crate::{
    if_group_breaks, if_group_fits_on_single_line, soft_line_break, token, Formatter, Token,
};
use rome_formatter::{format_elements, space_token, FormatElement};

pub fn jsx_whitespace(formatter: &Formatter) -> FormatElement {
    format_elements![
        if_group_breaks(format_elements![
            //            format_string_literal_token(token("{{\" \"}}").value(), formatter),
            soft_line_break()
        ]),
        if_group_fits_on_single_line(space_token())
    ]
}
