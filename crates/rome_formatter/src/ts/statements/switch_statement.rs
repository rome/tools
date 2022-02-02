use crate::formatter_traits::FormatTokenAndNode;
use crate::{block_indent, FormatResult};
use crate::{
    format_element::indent, format_elements, group_elements, hard_line_break,
    join_elements_hard_line, soft_block_indent, space_token, FormatElement, Formatter,
    ToFormatElement,
};
use rslint_parser::ast::{JsAnySwitchClause, JsCaseClause, JsDefaultClause, JsSwitchStatement};

impl ToFormatElement for JsSwitchStatement {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        Ok(format_elements![
            self.switch_token().format(formatter)?,
            space_token(),
            group_elements(formatter.format_delimited(
                &self.l_paren_token()?,
                |open_token_trailing, close_token_leading| Ok(soft_block_indent(format_elements![
                    open_token_trailing,
                    self.discriminant().format(formatter)?,
                    close_token_leading,
                ])),
                &self.r_paren_token()?,
            )?),
            space_token(),
            group_elements(formatter.format_delimited(
                &self.l_curly_token()?,
                |open_token_trailing, close_token_leading| {
                    Ok(block_indent(format_elements![
                        open_token_trailing,
                        join_elements_hard_line(
                            self.cases()
                                .into_iter()
                                .zip(formatter.format_nodes(self.cases())?)
                        ),
                        close_token_leading,
                    ]))
                },
                &self.r_curly_token()?
            )?)
        ])
    }
}

impl ToFormatElement for JsAnySwitchClause {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        match self {
            JsAnySwitchClause::JsCaseClause(case_clause) => {
                case_clause.to_format_element(formatter)
            }
            JsAnySwitchClause::JsDefaultClause(default_clause) => {
                default_clause.to_format_element(formatter)
            }
        }
    }
}

impl ToFormatElement for JsDefaultClause {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        let default = self.default_token().format(formatter)?;
        let colon = self.colon_token().format(formatter)?;
        let statements = formatter.format_list(self.consequent());

        Ok(format_elements![
            default,
            colon,
            space_token(),
            // no line break needed after because it is added by the indent in the switch statement
            indent(format_elements![hard_line_break(), statements])
        ])
    }
}

impl ToFormatElement for JsCaseClause {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        let case_word = self.case_token().format(formatter)?;
        let colon = self.colon_token().format(formatter)?;
        let test = self.test().format(formatter)?;
        let cons = formatter.format_list(self.consequent());

        Ok(format_elements![
            case_word,
            space_token(),
            test,
            colon,
            // no line break needed after because it is added by the indent in the switch statement
            indent(format_elements![hard_line_break(), cons])
        ])
    }
}
