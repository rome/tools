use crate::{FormatToken, FormatValue};
use rslint_parser::ast::Decl;

impl FormatValue for Decl {
	fn format(&self) -> FormatToken {
		match self {
			Decl::FnDecl(fn_decl) => fn_decl.format(),
			Decl::ClassDecl(_) => todo!(),
			Decl::VarDecl(var_decl) => var_decl.format(),
			Decl::TsEnum(_) => todo!(),
			Decl::TsTypeAliasDecl(_) => todo!(),
			Decl::TsNamespaceDecl(_) => todo!(),
			Decl::TsModuleDecl(_) => todo!(),
			Decl::TsInterfaceDecl(_) => todo!(),
		}
	}
}
