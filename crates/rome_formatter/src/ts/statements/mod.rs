use crate::{join_elements_hard_line, FormatElement, Formatter};
use rslint_parser::ast::{AstNodeList, JsStatementList};
use rslint_parser::AstNode;

mod block;
mod break_statement;
mod continue_statement;
mod debugger_statement;
mod do_while_statement;
mod empty_statement;
mod expression_statement;
mod for_in_statement;
mod for_of_statement;
mod for_stmt;
mod function_statement;
mod if_stmt;
mod label_statement;
mod return_statement;
mod statement;
mod switch_statement;
mod throw_statement;
mod try_statement;
mod variable_declaration_statement;
mod while_statement;
mod with_statement;

/// Formats a list of statements
pub fn format_statements(stmts: JsStatementList, formatter: &Formatter) -> FormatElement {
    join_elements_hard_line(stmts.iter().map(|stmt| {
        let snapshot = formatter.snapshot();
        let elem = match formatter.format_node(stmt.clone()) {
            Ok(result) => result,
            Err(_) => {
                formatter.restore(snapshot);
                formatter
                    .format_verbatim(stmt.syntax())
                    .trim_start()
                    .trim_end()
            }
        };

        (stmt, elem)
    }))
}
