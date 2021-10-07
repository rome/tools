use rslint_parser::ast::Stmt;

use crate::FormatValue;

impl FormatValue for Stmt {
	fn format(&self) -> crate::FormatElement {
		match self {
			Stmt::BlockStmt(_) => todo!(),
			Stmt::EmptyStmt(_) => todo!(),
			Stmt::ExprStmt(_) => todo!(),
			Stmt::IfStmt(_) => todo!(),
			Stmt::DoWhileStmt(_) => todo!(),
			Stmt::WhileStmt(_) => todo!(),
			Stmt::ForStmt(_) => todo!(),
			Stmt::ForInStmt(_) => todo!(),
			Stmt::ContinueStmt(_) => todo!(),
			Stmt::BreakStmt(_) => todo!(),
			Stmt::ReturnStmt(stmt) => stmt.format(),
			Stmt::WithStmt(_) => todo!(),
			Stmt::LabelledStmt(_) => todo!(),
			Stmt::SwitchStmt(_) => todo!(),
			Stmt::ThrowStmt(_) => todo!(),
			Stmt::TryStmt(_) => todo!(),
			Stmt::DebuggerStmt(_) => todo!(),
			Stmt::Decl(_) => todo!(),
		}
	}
}
