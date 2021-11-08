use crate::{concat_elements, hard_line_break, join_elements, FormatElement, Formatter};
use rslint_parser::ast::{AstNodeList, Stmt};
use rslint_parser::AstNode;

mod block;
mod break_statement;
mod continue_statement;
mod debugger_statement;
mod do_while_statement;
mod empty_statement;
mod expression_statement;
mod for_in_statement;
mod for_stmt;
mod if_stmt;
mod label_statement;
mod return_statement;
mod statement;
mod switch_statement;
mod throw_statement;
mod try_statement;
mod while_statement;
mod with_statement;

/// Formats a list of statements
pub fn format_statements(stmts: AstNodeList<Stmt>, formatter: &Formatter) -> FormatElement {
	join_elements(
		hard_line_break(),
		stmts.iter().map(|stmt| {
			formatter.format_node(stmt.clone()).unwrap_or_else(|| {
				let verbatim = formatter.format_raw(stmt.syntax());

				match verbatim {
					FormatElement::List(list) => {
						if let Some(FormatElement::Token(token)) = list.last() {
							if token.as_str() == "\n" {
								let mut elements = (*list).clone();
								elements.pop(); // Pop the last new line
								concat_elements(elements)
							} else {
								FormatElement::List(list)
							}
						} else {
							FormatElement::List(list)
						}
					}
					_ => verbatim,
				}
			})
		}),
	)
}
