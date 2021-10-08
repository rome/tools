use crate::{FormatContext, FormatElement, ToFormatElement};
use rslint_parser::ast::Decl;

impl ToFormatElement for Decl {
	fn to_format_element(&self, context: &FormatContext) -> FormatElement {
		match self {
			Decl::FnDecl(fn_decl) => fn_decl.to_format_element(context),
			Decl::ClassDecl(_) => todo!(),
			Decl::VarDecl(var_decl) => var_decl.to_format_element(context),
			Decl::TsEnum(_) => todo!(),
			Decl::TsTypeAliasDecl(_) => todo!(),
			Decl::TsNamespaceDecl(_) => todo!(),
			Decl::TsModuleDecl(_) => todo!(),
			Decl::TsInterfaceDecl(_) => todo!(),
		}
	}
}
