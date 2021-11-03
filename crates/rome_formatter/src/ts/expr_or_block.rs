use crate::{FormatElement, Formatter, ToFormatElement};
use rslint_parser::ast::ExprOrBlock;

impl ToFormatElement for ExprOrBlock {
	fn to_format_element(&self, formatter: &Formatter) -> Option<FormatElement> {
		match self {
			ExprOrBlock::BlockStmt(block) => block.to_format_element(formatter),
			ExprOrBlock::Literal(literal) => literal.to_format_element(formatter),
			ExprOrBlock::ArrowExpr(node) => node.to_format_element(formatter),
			ExprOrBlock::Template(_) => todo!(),
			ExprOrBlock::NameRef(node) => node.to_format_element(formatter),
			ExprOrBlock::ThisExpr(_) => todo!(),
			ExprOrBlock::ArrayExpr(node) => node.to_format_element(formatter),
			ExprOrBlock::ObjectExpr(node) => node.to_format_element(formatter),
			ExprOrBlock::GroupingExpr(_) => todo!(),
			ExprOrBlock::BracketExpr(_) => todo!(),
			ExprOrBlock::DotExpr(_) => todo!(),
			ExprOrBlock::NewExpr(_) => todo!(),
			ExprOrBlock::CallExpr(node) => node.to_format_element(formatter),
			ExprOrBlock::UnaryExpr(_) => todo!(),
			ExprOrBlock::BinExpr(_) => todo!(),
			ExprOrBlock::CondExpr(_) => todo!(),
			ExprOrBlock::AssignExpr(_) => todo!(),
			ExprOrBlock::SequenceExpr(node) => node.to_format_element(formatter),
			ExprOrBlock::FnExpr(_) => todo!(),
			ExprOrBlock::ClassExpr(_) => todo!(),
			ExprOrBlock::NewTarget(_) => todo!(),
			ExprOrBlock::ImportMeta(_) => todo!(),
			ExprOrBlock::SuperCall(node) => node.to_format_element(formatter),
			ExprOrBlock::ImportCall(_) => todo!(),
			ExprOrBlock::YieldExpr(_) => todo!(),
			ExprOrBlock::AwaitExpr(_) => todo!(),
			ExprOrBlock::PrivatePropAccess(_) => todo!(),
			ExprOrBlock::TsNonNull(_) => todo!(),
			ExprOrBlock::TsAssertion(_) => todo!(),
			ExprOrBlock::TsConstAssertion(_) => todo!(),
		}
	}
}
