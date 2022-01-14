use crate::ts::statements::format_statements;
use crate::FormatResult;
use crate::{
    block_indent, format_element::indent, format_elements, group_elements, hard_line_break,
    join_elements, soft_indent, space_token, FormatElement, Formatter, ToFormatElement,
};
use rslint_parser::ast::{JsAnySwitchClause, JsCaseClause, JsDefaultClause, JsSwitchStatement};

impl ToFormatElement for JsSwitchStatement {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        Ok(format_elements![
            formatter.format_token(&self.switch_token()?)?,
            space_token(),
            group_elements(format_elements![
                formatter.format_token(&self.l_paren_token()?)?,
                soft_indent(formatter.format_node(self.discriminant()?)?),
                formatter.format_token(&self.r_paren_token()?)?
            ]),
            space_token(),
            group_elements(format_elements![
                formatter.format_token(&self.l_curly_token()?)?,
                block_indent(join_elements(
                    hard_line_break(),
                    formatter.format_nodes(self.cases())?
                )),
                formatter.format_token(&self.r_curly_token()?)?
            ])
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
        let default = formatter.format_token(&self.default_token()?)?;
        let colon = formatter.format_token(&self.colon_token()?)?;
        let statements = format_statements(self.consequent(), formatter);

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
        let case_word = formatter.format_token(&self.case_token()?)?;
        let colon = formatter.format_token(&self.colon_token()?)?;

        let test = formatter.format_node(self.test()?)?;

        let cons = format_statements(self.consequent(), formatter);

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
