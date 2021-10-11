use rslint_parser::ast::Stmt;

use crate::{FormatElement, Formatter, ToFormatElement};

impl ToFormatElement for Stmt {
	fn to_format_element(&self, formatter: &Formatter) -> FormatElement {
		match self {
			Stmt::BlockStmt(block) => block.to_format_element(formatter),
			Stmt::EmptyStmt(empty_statement) => empty_statement.to_format_element(formatter),
			Stmt::ExprStmt(expr_stmt) => expr_stmt.to_format_element(formatter),
			Stmt::IfStmt(_) => todo!(),
			Stmt::DoWhileStmt(_) => todo!(),
			Stmt::WhileStmt(_) => todo!(),
			Stmt::ForStmt(_) => todo!(),
			Stmt::ForInStmt(_) => todo!(),
			Stmt::ContinueStmt(_) => todo!(),
			Stmt::BreakStmt(_) => todo!(),
			Stmt::ReturnStmt(stmt) => stmt.to_format_element(formatter),
			Stmt::WithStmt(_) => todo!(),
			Stmt::LabelledStmt(_) => todo!(),
			Stmt::SwitchStmt(_) => todo!(),
			Stmt::ThrowStmt(_) => todo!(),
			Stmt::TryStmt(_) => todo!(),
			Stmt::DebuggerStmt(_) => todo!(),
			Stmt::Decl(decl) => decl.to_format_element(formatter),
		}
	}
}
