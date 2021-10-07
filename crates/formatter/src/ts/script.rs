use crate::{
	concat_elements, hard_line_break, ts::format_syntax_token, FormatElement, ToFormatElement,
};
use rslint_parser::ast::{Script, Stmt};

impl ToFormatElement for Script {
	fn to_format_element(&self) -> FormatElement {
		let mut tokens = vec![];

		if let Some(shebang) = self.shebang_token() {
			tokens.push(format_syntax_token(shebang));
			tokens.push(hard_line_break());
		}

		for item in self.items() {
			let token = match item {
				Stmt::BlockStmt(block) => block.to_format_element(),
				Stmt::EmptyStmt(_) => todo!(),
				Stmt::ExprStmt(expression_statement) => expression_statement.to_format_element(),
				Stmt::IfStmt(_) => todo!(),
				Stmt::DoWhileStmt(_) => todo!(),
				Stmt::WhileStmt(_) => todo!(),
				Stmt::ForStmt(_) => todo!(),
				Stmt::ForInStmt(_) => todo!(),
				Stmt::ContinueStmt(_) => todo!(),
				Stmt::BreakStmt(_) => todo!(),
				Stmt::ReturnStmt(statement) => statement.to_format_element(),
				Stmt::WithStmt(_) => todo!(),
				Stmt::LabelledStmt(_) => todo!(),
				Stmt::SwitchStmt(_) => todo!(),
				Stmt::ThrowStmt(_) => todo!(),
				Stmt::TryStmt(_) => todo!(),
				Stmt::DebuggerStmt(_) => todo!(),
				Stmt::Decl(decl) => decl.to_format_element(),
			};

			tokens.push(token);
		}

		concat_elements(tokens)
	}
}
