use crate::{FormatElement, Formatter, ToFormatElement};
use rslint_parser::ast::Stmt;

impl ToFormatElement for Stmt {
	fn to_format_element(&self, formatter: &Formatter) -> Option<FormatElement> {
		match self {
			Stmt::BlockStmt(block) => block.to_format_element(formatter),
			Stmt::EmptyStmt(empty_statement) => empty_statement.to_format_element(formatter),
			Stmt::ExprStmt(expr_stmt) => expr_stmt.to_format_element(formatter),
			Stmt::IfStmt(if_stmt) => if_stmt.to_format_element(formatter),
			Stmt::DoWhileStmt(do_while_statement) => {
				do_while_statement.to_format_element(formatter)
			}
			Stmt::WhileStmt(while_statement) => while_statement.to_format_element(formatter),
			Stmt::ForStmt(for_stmt) => for_stmt.to_format_element(formatter),
			Stmt::ForInStmt(_) => todo!(),
			Stmt::ContinueStmt(_) => todo!(),
			Stmt::BreakStmt(break_statement) => break_statement.to_format_element(formatter),
			Stmt::ReturnStmt(stmt) => stmt.to_format_element(formatter),
			Stmt::WithStmt(_) => todo!(),
			Stmt::LabelledStmt(_) => todo!(),
			Stmt::SwitchStmt(switch_statement) => switch_statement.to_format_element(formatter),
			Stmt::ThrowStmt(_) => todo!(),
			Stmt::TryStmt(_) => todo!(),
			Stmt::DebuggerStmt(_) => todo!(),
			Stmt::Decl(decl) => decl.to_format_element(formatter),
		}
	}
}
