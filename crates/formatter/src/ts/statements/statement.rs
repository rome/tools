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
			Stmt::ForInStmt(for_in_statement) => for_in_statement.to_format_element(formatter),
			Stmt::ContinueStmt(continue_statement) => {
				continue_statement.to_format_element(formatter)
			}
			Stmt::BreakStmt(break_statement) => break_statement.to_format_element(formatter),
			Stmt::ReturnStmt(stmt) => stmt.to_format_element(formatter),
			Stmt::WithStmt(with_statement) => with_statement.to_format_element(formatter),
			Stmt::LabelledStmt(label_statement) => label_statement.to_format_element(formatter),
			Stmt::SwitchStmt(switch_statement) => switch_statement.to_format_element(formatter),
			Stmt::ThrowStmt(throw_statement) => throw_statement.to_format_element(formatter),
			Stmt::TryStmt(try_statement) => try_statement.to_format_element(formatter),
			Stmt::DebuggerStmt(debugger_statement) => {
				debugger_statement.to_format_element(formatter)
			}

			Stmt::ForOfStmt(_) => todo!(),
			Stmt::FnDecl(node) => node.to_format_element(formatter),
			Stmt::ClassDecl(node) => node.to_format_element(formatter),
			Stmt::VarDecl(node) => node.to_format_element(formatter),
			Stmt::TsEnum(_) => todo!(),
			Stmt::TsTypeAliasDecl(_) => todo!(),
			Stmt::TsNamespaceDecl(_) => todo!(),
			Stmt::TsModuleDecl(_) => todo!(),
			Stmt::TsInterfaceDecl(_) => todo!(),
		}
	}
}
