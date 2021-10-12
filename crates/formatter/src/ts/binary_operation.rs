use crate::{token, ToFormatElement};
use rslint_parser::ast::BinOp;

impl ToFormatElement for BinOp {
	fn to_format_element(&self, _formatter: &crate::Formatter) -> crate::FormatElement {
		match self {
			BinOp::LessThan => todo!(),
			BinOp::GreaterThan => todo!(),
			BinOp::LessThanOrEqual => todo!(),
			BinOp::GreaterThanOrEqual => todo!(),
			BinOp::Equality => todo!(),
			BinOp::StrictEquality => todo!(),
			BinOp::Inequality => todo!(),
			BinOp::StrictInequality => todo!(),
			BinOp::Plus => todo!(),
			BinOp::Minus => todo!(),
			BinOp::Times => todo!(),
			BinOp::Divide => todo!(),
			BinOp::Remainder => todo!(),
			BinOp::Exponent => todo!(),
			BinOp::LeftShift => todo!(),
			BinOp::RightShift => todo!(),
			BinOp::UnsignedRightShift => todo!(),
			BinOp::BitwiseAnd => todo!(),
			BinOp::BitwiseOr => todo!(),
			BinOp::BitwiseXor => todo!(),
			BinOp::NullishCoalescing => todo!(),
			BinOp::LogicalOr => todo!(),
			BinOp::LogicalAnd => token("&&"),
			BinOp::In => todo!(),
			BinOp::Instanceof => todo!(),
		}
	}
}
