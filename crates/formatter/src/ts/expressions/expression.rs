use crate::{FormatElement, FormatValue};
use rslint_parser::ast::Expr;

impl FormatValue for Expr {
	fn format(&self) -> FormatElement {
		match self {
			Expr::ArrowExpr(arrow) => arrow.format(),
			Expr::Literal(literal) => literal.format(),
			Expr::Template(_) => todo!(),
			Expr::NameRef(name_ref) => name_ref.format(),
			Expr::ThisExpr(_) => todo!(),
			Expr::ArrayExpr(array_expression) => array_expression.format(),
			Expr::ObjectExpr(_) => todo!(),
			Expr::GroupingExpr(_) => todo!(),
			Expr::BracketExpr(_) => todo!(),
			Expr::DotExpr(_) => todo!(),
			Expr::NewExpr(_) => todo!(),
			Expr::CallExpr(_) => todo!(),
			Expr::UnaryExpr(_) => todo!(),
			Expr::BinExpr(_) => todo!(),
			Expr::CondExpr(_) => todo!(),
			Expr::AssignExpr(_) => todo!(),
			Expr::SequenceExpr(_) => todo!(),
			Expr::FnExpr(_) => todo!(),
			Expr::ClassExpr(_) => todo!(),
			Expr::NewTarget(_) => todo!(),
			Expr::ImportMeta(_) => todo!(),
			Expr::SuperCall(_) => todo!(),
			Expr::ImportCall(_) => todo!(),
			Expr::YieldExpr(_) => todo!(),
			Expr::AwaitExpr(_) => todo!(),
			Expr::PrivatePropAccess(_) => todo!(),
			Expr::TsNonNull(_) => todo!(),
			Expr::TsAssertion(_) => todo!(),
			Expr::TsConstAssertion(_) => todo!(),
		}
	}
}
