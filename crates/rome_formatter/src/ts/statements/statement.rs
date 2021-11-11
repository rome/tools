use crate::{FormatElement, FormatResult, Formatter, ToFormatElement};
use rslint_parser::ast::JsAnyStatement;

impl ToFormatElement for JsAnyStatement {
	fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
		match self {
			JsAnyStatement::BlockStmt(block) => block.to_format_element(formatter),
			JsAnyStatement::EmptyStmt(empty_statement) => {
				empty_statement.to_format_element(formatter)
			}
			JsAnyStatement::ExprStmt(expr_stmt) => expr_stmt.to_format_element(formatter),
			JsAnyStatement::IfStmt(if_stmt) => if_stmt.to_format_element(formatter),
			JsAnyStatement::DoWhileStmt(do_while_statement) => {
				do_while_statement.to_format_element(formatter)
			}
			JsAnyStatement::WhileStmt(while_statement) => {
				while_statement.to_format_element(formatter)
			}
			JsAnyStatement::ForStmt(for_stmt) => for_stmt.to_format_element(formatter),
			JsAnyStatement::ForInStmt(for_in_statement) => {
				for_in_statement.to_format_element(formatter)
			}
			JsAnyStatement::ContinueStmt(continue_statement) => {
				continue_statement.to_format_element(formatter)
			}
			JsAnyStatement::BreakStmt(break_statement) => {
				break_statement.to_format_element(formatter)
			}
			JsAnyStatement::ReturnStmt(stmt) => stmt.to_format_element(formatter),
			JsAnyStatement::WithStmt(with_statement) => with_statement.to_format_element(formatter),
			JsAnyStatement::LabelledStmt(label_statement) => {
				label_statement.to_format_element(formatter)
			}
			JsAnyStatement::SwitchStmt(switch_statement) => {
				switch_statement.to_format_element(formatter)
			}
			JsAnyStatement::ThrowStmt(throw_statement) => {
				throw_statement.to_format_element(formatter)
			}
			JsAnyStatement::TryStmt(try_statement) => try_statement.to_format_element(formatter),
			JsAnyStatement::DebuggerStmt(debugger_statement) => {
				debugger_statement.to_format_element(formatter)
			}

			JsAnyStatement::ForOfStmt(_) => todo!(),
			JsAnyStatement::Decl(decl) => decl.to_format_element(formatter),
			JsAnyStatement::JsUnknownStatement(node) => Ok(formatter.format_raw(node.syntax())),
			JsAnyStatement::ImportDecl(_) => todo!(),
			JsAnyStatement::ExportNamed(_) => todo!(),
			JsAnyStatement::ExportDefaultDecl(_) => todo!(),
			JsAnyStatement::ExportDefaultExpr(_) => todo!(),
			JsAnyStatement::ExportWildcard(_) => todo!(),
			JsAnyStatement::ExportDecl(_) => todo!(),
			JsAnyStatement::TsImportEqualsDecl(_) => todo!(),
			JsAnyStatement::TsExportAssignment(_) => todo!(),
			JsAnyStatement::TsNamespaceExportDecl(_) => todo!(),
		}
	}
}
