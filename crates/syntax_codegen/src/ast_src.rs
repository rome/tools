#[derive(Default, Debug)]
pub(crate) struct AstSrc {
	pub(crate) tokens: Vec<String>,
	pub(crate) nodes: Vec<AstNodeSrc>,
	pub(crate) enums: Vec<AstEnumSrc>,
}

#[derive(Debug, Default)]
pub(crate) struct AstNodeSrc {
	pub(crate) doc: Vec<String>,
	pub(crate) name: String,
	pub(crate) traits: Vec<String>,
	pub(crate) fields: Vec<Field>,
}

#[allow(unused)]
#[derive(Debug, Eq, PartialEq)]
pub(crate) enum Field {
	Token(String),
	Node {
		name: String,
		ty: String,
		cardinality: Cardinality,
	},
	NamedToken {
		name: String,
		ty: String,
		cardinality: Cardinality,
	},
}

#[derive(Debug, Eq, PartialEq)]
pub(crate) enum Cardinality {
	Optional,
	Many,
}

#[derive(Debug, Default)]
pub(crate) struct AstEnumSrc {
	pub(crate) doc: Vec<String>,
	pub(crate) name: String,
	pub(crate) traits: Vec<String>,
	pub(crate) variants: Vec<String>,
}
