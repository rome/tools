use crate::{format_tokens, FormatToken, FormatValue, LineToken};
use rslint_parser::ast::{Script, Stmt};

impl FormatValue for Script {
	fn format(&self) -> FormatToken {
		let mut tokens = vec![];

		if let Some(shebang) = self.shebang_token() {
			tokens.push(format_tokens!(shebang.text().as_str()));
			tokens.push(format_tokens!(LineToken::hard()));
		}

		for item in self.items() {
			let token = match item {
				Stmt::BlockStmt(block) => block.format(),
				Stmt::EmptyStmt(_) => todo!(),
				Stmt::ExprStmt(expression_statement) => expression_statement.format(),
				Stmt::IfStmt(_) => todo!(),
				Stmt::DoWhileStmt(_) => todo!(),
				Stmt::WhileStmt(_) => todo!(),
				Stmt::ForStmt(_) => todo!(),
				Stmt::ForInStmt(_) => todo!(),
				Stmt::ContinueStmt(_) => todo!(),
				Stmt::BreakStmt(_) => todo!(),
				Stmt::ReturnStmt(statement) => statement.format(),
				Stmt::WithStmt(_) => todo!(),
				Stmt::LabelledStmt(_) => todo!(),
				Stmt::SwitchStmt(_) => todo!(),
				Stmt::ThrowStmt(_) => todo!(),
				Stmt::TryStmt(_) => todo!(),
				Stmt::DebuggerStmt(_) => todo!(),
				Stmt::Decl(decl) => decl.format(),
			};

			tokens.push(token);
		}

		FormatToken::concat(tokens)
	}
}
