use crate::{FormatElement, FormatResult, Formatter, ToFormatElement};
use rslint_parser::{ast::JsAnyStatement, AstNode};

impl ToFormatElement for JsAnyStatement {
	fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
		match self {
			JsAnyStatement::JsBlockStatement(block) => block.to_format_element(formatter),
			JsAnyStatement::JsEmptyStatement(empty_statement) => {
				empty_statement.to_format_element(formatter)
			}
			JsAnyStatement::JsExpressionStatement(expr_stmt) => {
				expr_stmt.to_format_element(formatter)
			}
			JsAnyStatement::JsIfStatement(if_stmt) => if_stmt.to_format_element(formatter),
			JsAnyStatement::JsDoWhileStatement(do_while_statement) => {
				do_while_statement.to_format_element(formatter)
			}
			JsAnyStatement::JsWhileStatement(while_statement) => {
				while_statement.to_format_element(formatter)
			}
			JsAnyStatement::ForStmt(for_stmt) => for_stmt.to_format_element(formatter),
			JsAnyStatement::ForInStmt(for_in_statement) => {
				for_in_statement.to_format_element(formatter)
			}
			JsAnyStatement::JsContinueStatement(continue_statement) => {
				continue_statement.to_format_element(formatter)
			}
			JsAnyStatement::JsBreakStatement(break_statement) => {
				break_statement.to_format_element(formatter)
			}
			JsAnyStatement::JsReturnStatement(stmt) => stmt.to_format_element(formatter),
			JsAnyStatement::JsWithStatement(with_statement) => {
				with_statement.to_format_element(formatter)
			}
			JsAnyStatement::JsLabeledStatement(label_statement) => {
				label_statement.to_format_element(formatter)
			}
			JsAnyStatement::JsSwitchStatement(switch_statement) => {
				switch_statement.to_format_element(formatter)
			}
			JsAnyStatement::JsThrowStatement(throw_statement) => {
				throw_statement.to_format_element(formatter)
			}
			JsAnyStatement::JsTryStatement(try_statement) => {
				try_statement.to_format_element(formatter)
			}
			JsAnyStatement::JsDebuggerStatement(debugger_statement) => {
				debugger_statement.to_format_element(formatter)
			}

			JsAnyStatement::ForOfStmt(_) => todo!(),
			JsAnyStatement::JsFunctionDeclaration(decl) => decl.to_format_element(formatter),
			JsAnyStatement::JsClassDeclaration(decl) => decl.to_format_element(formatter),
			JsAnyStatement::JsVariableDeclarationStatement(decl) => {
				decl.to_format_element(formatter)
			}
			JsAnyStatement::JsUnknownStatement(unknown_statement) => {
				Ok(formatter.format_raw(unknown_statement.syntax()))
			}
			JsAnyStatement::ImportDecl(_) => todo!(),
			JsAnyStatement::ExportNamed(_) => todo!(),
			JsAnyStatement::ExportDefaultDecl(_) => todo!(),
			JsAnyStatement::ExportDefaultExpr(_) => todo!(),
			JsAnyStatement::ExportWildcard(_) => todo!(),
			JsAnyStatement::ExportDecl(_) => todo!(),
			JsAnyStatement::TsImportEqualsDecl(_) => todo!(),
			JsAnyStatement::TsExportAssignment(_) => todo!(),
			JsAnyStatement::TsNamespaceExportDecl(_) => todo!(),
			JsAnyStatement::JsTryFinallyStatement(try_finally) => {
				try_finally.to_format_element(formatter)
			}
			JsAnyStatement::TsEnum(_) => todo!(),
			JsAnyStatement::TsTypeAliasDecl(_) => todo!(),
			JsAnyStatement::TsNamespaceDecl(_) => todo!(),
			JsAnyStatement::TsModuleDecl(_) => todo!(),
			JsAnyStatement::TsInterfaceDecl(_) => todo!(),
		}
	}
}
