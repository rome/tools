use crate::{hard_line_break, join_elements, FormatElement, Formatter};
use rslint_parser::ast::{AstChildren, Stmt};

mod block;
mod break_statement;
mod do_while_statement;
mod empty_statement;
mod expression_statement;
mod for_stmt;
mod if_stmt;
mod return_statement;
mod statement;
mod switch_statement;
mod while_statement;

/// Formats a list of statements
pub fn format_statements(stmts: AstChildren<Stmt>, formatter: &Formatter) -> FormatElement {
	join_elements(
		hard_line_break(),
		stmts.map(|stmt| formatter.format_node(stmt)),
	)
}
