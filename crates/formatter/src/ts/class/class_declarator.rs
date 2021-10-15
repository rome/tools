use crate::{
	block_indent, empty_element, format_elements, group_elements, hard_line_break, join_elements,
	soft_indent, soft_line_break_or_space, space_token, token, FormatElement, Formatter,
	ToFormatElement,
};
use rslint_parser::ast::{
	ClassBody, ClassDecl, ClassElement, Constructor, ConstructorParamOrPat, ConstructorParameters,
	SuperCall,
};

impl ToFormatElement for ClassDecl {
	fn to_format_element(&self, formatter: &Formatter) -> Option<FormatElement> {
		let class_token = formatter.format_token(&self.class_token()?)?;
		let name = formatter.format_node(self.name()?)?;
		let extends = if let Some(parent) = self.parent() {
			let extends_token = formatter.format_token(&self.extends_token()?)?;
			format_elements![
				extends_token,
				space_token(),
				formatter.format_node(parent)?,
				space_token()
			]
		} else {
			empty_element()
		};

		let body = formatter.format_node(self.body()?)?;

		Some(format_elements![
			class_token,
			space_token(),
			name,
			space_token(),
			extends,
			body
		])
	}
}

impl ToFormatElement for ClassBody {
	fn to_format_element(&self, formatter: &Formatter) -> Option<FormatElement> {
		let l_paren = formatter.format_token(&self.l_curly_token()?)?;
		let elements = formatter.format_children(self.elements())?;
		let r_paren = formatter.format_token(&self.r_curly_token()?)?;

		Some(group_elements(format_elements![
			l_paren,
			block_indent(join_elements(hard_line_break(), elements)),
			r_paren
		]))
	}
}

impl ToFormatElement for ClassElement {
	fn to_format_element(&self, formatter: &Formatter) -> Option<FormatElement> {
		match self {
			ClassElement::EmptyStmt(_) => todo!(),
			ClassElement::Method(method) => method.to_format_element(formatter),
			ClassElement::PrivateProp(_) => todo!(),
			ClassElement::ClassProp(class_prop) => class_prop.to_format_element(formatter),
			ClassElement::Constructor(constructor) => constructor.to_format_element(formatter),
			ClassElement::TsIndexSignature(_) => todo!(),
			ClassElement::Getter(getter) => getter.to_format_element(formatter),
			ClassElement::Setter(setter) => setter.to_format_element(formatter),
		}
	}
}

impl ToFormatElement for Constructor {
	fn to_format_element(&self, formatter: &Formatter) -> Option<FormatElement> {
		let constructor_token = formatter.format_node(self.name()?)?;
		let params = formatter.format_node(self.parameters()?)?;
		let body = formatter.format_node(self.body()?)?;
		Some(format_elements![
			constructor_token,
			params,
			space_token(),
			body
		])
	}
}

impl ToFormatElement for ConstructorParameters {
	fn to_format_element(&self, formatter: &Formatter) -> Option<FormatElement> {
		let l_bracket = formatter.format_token(&self.l_paren_token()?)?;
		let params = formatter.format_children(self.parameters())?;
		let r_bracket = formatter.format_token(&self.r_paren_token()?)?;

		Some(format_elements![group_elements(format_elements![
			l_bracket,
			join_elements(
				format_elements![token(","), soft_line_break_or_space()],
				params
			),
			r_bracket
		])])
	}
}

impl ToFormatElement for ConstructorParamOrPat {
	fn to_format_element(&self, formatter: &Formatter) -> Option<FormatElement> {
		match self {
			ConstructorParamOrPat::TsConstructorParam(_) => todo!(),
			ConstructorParamOrPat::Pattern(pattern) => pattern.to_format_element(formatter),
		}
	}
}

impl ToFormatElement for SuperCall {
	fn to_format_element(&self, formatter: &Formatter) -> Option<FormatElement> {
		let super_token = formatter.format_token(&self.super_token()?)?;
		let arguments = formatter.format_node(self.arguments()?)?;
		Some(format_elements![super_token, arguments])
	}
}
