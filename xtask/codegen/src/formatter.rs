#![allow(deprecated)]

use std::{
    cell::RefCell,
    cmp::Ordering,
    collections::{BTreeMap, BTreeSet},
    fs::{create_dir_all, read_dir, read_to_string, remove_file, File},
    io::{self, Write},
    path::{Path, PathBuf},
    rc::Rc,
};

use proc_macro2::{Ident, Span};
use quote::quote;
use syn::{parse_file, spanned::Spanned};
use xtask::project_root;

use crate::ast::load_ast;

#[deprecated]
struct FileStore {
    use_items: Vec<String>,
    impl_items: BTreeMap<String, String>,
}

#[deprecated]
type FileSet = Rc<RefCell<FileStore>>;

#[deprecated]
#[derive(Default)]
struct NodeIndex {
    entries: BTreeMap<String, NodeIndexEntry>,
    files: BTreeMap<PathBuf, FileSet>,
}

#[deprecated]
struct NodeIndexEntry {
    use_items: Vec<String>,
    impl_item: String,
    file: FileSet,
}

#[deprecated]
fn load_source(index: &mut NodeIndex, path: &Path) {
    let code_str = read_to_string(path).unwrap();
    let code = parse_file(&code_str)
        .unwrap_or_else(|err| panic!("failed to parse {}: {:?}", path.display(), err));

    let mut use_items: Vec<String> = Vec::new();
    let mut impl_items: BTreeMap<String, String> = BTreeMap::new();

    for item in code.items {
        match item {
            syn::Item::Use(use_item) => {
                use_items.push(slice_source(&code_str, &use_item));
            }

            syn::Item::Impl(impl_item) => {
                if let Some((_, path, _)) = &impl_item.trait_ {
                    if let Some(segment) = path.segments.last() {
                        if segment.ident == "ToFormatElement" {
                            if let syn::Type::Path(path) = &*impl_item.self_ty {
                                if let Some(segment) = path.path.segments.last() {
                                    impl_items.insert(
                                        segment.ident.to_string(),
                                        slice_source(&code_str, &impl_item),
                                    );
                                }
                            }
                        }
                    }
                }
            }

            _ => {}
        }
    }

    let file = Rc::new(RefCell::new(FileStore {
        use_items: use_items.clone(),
        impl_items: impl_items.clone(),
    }));

    index.files.insert(path.into(), Rc::clone(&file));

    for (key, impl_item) in impl_items {
        index.entries.insert(
            key,
            NodeIndexEntry {
                use_items: use_items.clone(),
                impl_item,
                file: Rc::clone(&file),
            },
        );
    }
}

#[deprecated]
fn slice_source(source: &str, node: &impl Spanned) -> String {
    let span = node.span();
    let start = span.start();
    let end = span.end();

    let mut buffer = String::new();
    for (line_index, line_content) in source.lines().enumerate() {
        let line_number = line_index + 1;
        let (offset, line_content) = match line_number.cmp(&start.line) {
            Ordering::Less => continue,
            Ordering::Equal => (start.column, &line_content[start.column..]),
            Ordering::Greater => (0, line_content),
        };

        let line_content = match line_number.cmp(&end.line) {
            Ordering::Less => line_content,
            Ordering::Equal => &line_content[..end.column - offset],
            Ordering::Greater => break,
        };

        buffer.push_str(line_content);
        buffer.push('\n');
    }

    buffer
}

#[deprecated]
fn traverse_directory(index: &mut NodeIndex, path: &Path) -> io::Result<()> {
    for entry in read_dir(path)? {
        let entry = match entry {
            Ok(entry) => entry,
            Err(_) => continue,
        };

        let kind = match entry.file_type() {
            Ok(kind) => kind,
            Err(_) => continue,
        };

        if kind.is_file() {
            load_source(index, &entry.path());
            continue;
        }

        if kind.is_dir() {
            traverse_directory(index, &entry.path()).ok();
            continue;
        }
    }

    Ok(())
}

struct ModuleIndex {
    root: PathBuf,
    modules: BTreeMap<PathBuf, BTreeSet<String>>,
}

impl ModuleIndex {
    fn new(root: PathBuf) -> Self {
        Self {
            root,
            modules: BTreeMap::default(),
        }
    }

    /// Add a new module to the index
    fn insert(&mut self, path: &Path) {
        // Walk up from the module file towards the root
        let mut parent = path.parent();
        let mut file_stem = path.file_stem();

        while let (Some(path), Some(stem)) = (parent, file_stem) {
            // Insert each module into its parent
            let stem = stem.to_str().unwrap().to_owned();
            self.modules.entry(path.into()).or_default().insert(stem);

            parent = path.parent();
            file_stem = path.file_stem();

            // Stop at the root directory
            if parent == Some(&self.root) {
                break;
            }
        }
    }

    /// Create all the mod.rs files needed to import
    /// all the modules in the index up to the root
    fn print(self) {
        for (path, imports) in self.modules {
            let mut content = String::new();

            let stem = path.file_stem().unwrap().to_str().unwrap();
            for import in imports {
                // Clippy complains about child modules having the same
                // names as their parent, eg. js/name/name.rs
                if import == stem {
                    content.push_str("#[allow(clippy::module_inception)]\n");
                }

                content.push_str("mod ");
                content.push_str(&import);
                content.push_str(";\n");
            }

            let content = xtask::reformat(content).unwrap();

            let path = path.join("mod.rs");
            let mut file = File::create(&path).unwrap();
            file.write_all(content.as_bytes()).unwrap();
        }
    }
}

enum NodeKind {
    Node,
    List { separated: bool },
    Unknown,
    Union { variants: Vec<String> },
}

pub fn generate_formatter() {
    let mut index = NodeIndex::default();

    if true {
        traverse_directory(
            &mut index,
            &project_root().join("crates/rome_formatter/src/old"),
        )
        .ok();
    }

    let ast = load_ast();

    // Store references to all the files created by the codegen
    // script to build the module import files
    let mut modules = ModuleIndex::new(project_root().join("crates/rome_formatter/src"));

    // Build an unified iterator over all the AstNode types
    let names = ast
        .nodes
        .into_iter()
        .map(|node| (NodeKind::Node, node.name))
        .chain(ast.lists.into_iter().map(|(name, node)| {
            (
                NodeKind::List {
                    separated: node.separator.is_some(),
                },
                name,
            )
        }))
        .chain(
            ast.unknowns
                .into_iter()
                .map(|name| (NodeKind::Unknown, name)),
        )
        .chain(ast.unions.into_iter().map(|node| {
            (
                NodeKind::Union {
                    variants: node.variants,
                },
                node.name,
            )
        }));

    // Create a default implementation for theses nodes only if
    // the file doesn't already exist
    for (kind, name) in names {
        let path = name_to_path(&kind, &name);
        modules.insert(&path);

        // Union nodes except for AnyFunction and AnyClass have a generated
        // implementation, the codegen will always overwrite any existing file
        let allow_overwrite = matches!(kind, NodeKind::Union { .. })
            && name != "JsAnyFunction"
            && name != "JsAnyClass";

        if !allow_overwrite && path.exists() {
            continue;
        }

        let dir = path.parent().unwrap();
        create_dir_all(dir).unwrap();

        let tokens = match index.entries.remove(&name) {
            Some(entry) if !allow_overwrite => {
                let use_items = entry.use_items;
                let impl_item = entry.impl_item;

                entry.file.borrow_mut().impl_items.remove(&name);

                format!("{}\n{}", use_items.join("\n"), impl_item)
            }
            _ => {
                let id = Ident::new(&name, Span::call_site());

                // Generate a default implementation of ToFormatElement using format_list on
                // non-separated lists, to_format_element on the wrapped node for unions and
                // format_verbatim for all the other nodes
                let tokens = match kind {
                    NodeKind::List { separated: false } => quote! {
                        use crate::{FormatElement, FormatResult, Formatter, ToFormatElement};
                        use rslint_parser::ast::#id;

                        impl ToFormatElement for #id {
                            fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
                                Ok(formatter.format_list(self.clone()))
                            }
                        }
                    },
                    NodeKind::Node | NodeKind::Unknown | NodeKind::List { separated: true } => {
                        quote! {
                            use crate::{FormatElement, FormatResult, Formatter, ToFormatElement};
                            use rslint_parser::{ast::#id, AstNode};

                            impl ToFormatElement for #id {
                                fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
                                    Ok(formatter.format_verbatim(self.syntax()))
                                }
                            }
                        }
                    }
                    NodeKind::Union { variants } => {
                        // For each variant of the union call to_format_element on the wrapped node
                        let match_arms: Vec<_> = variants
                            .into_iter()
                            .map(|variant| {
                                let variant = Ident::new(&variant, Span::call_site());
                                quote! { Self::#variant(node) => node.to_format_element(formatter), }
                            })
                            .collect();

                        quote! {
                            use crate::{FormatElement, FormatResult, Formatter, ToFormatElement};
                            use rslint_parser::ast::#id;

                            impl ToFormatElement for #id {
                                fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
                                    match self {
                                        #( #match_arms )*
                                    }
                                }
                            }
                        }
                    }
                };

                if allow_overwrite {
                    xtask::reformat(tokens).unwrap()
                } else {
                    xtask::reformat_without_preamble(tokens).unwrap()
                }
            }
        };

        let mut file = File::create(&path).unwrap();
        file.write_all(tokens.as_bytes()).unwrap();
    }

    for (path, entry) in index.files {
        let entry = entry.borrow();
        if entry.impl_items.is_empty() {
            remove_file(path).unwrap();
        } else {
            let impl_items: Vec<_> = entry.impl_items.values().cloned().collect();
            let tokens = format!("{}\n{}", entry.use_items.join("\n"), impl_items.join("\n"));

            let mut file = File::create(&path).unwrap();
            file.write_all(tokens.as_bytes()).unwrap();
        }
    }

    modules.print();
}

enum NodeLanguage {
    Js,
    Ts,
}

impl NodeLanguage {
    fn as_str(&self) -> &'static str {
        match self {
            NodeLanguage::Js => "js",
            NodeLanguage::Ts => "ts",
        }
    }
}

enum NodeConcept {
    Expression,
    Statement,
    Object,
    Class,
    Assignment,
    Binding,
    Type,
    Module,
    Unknown,
    List,
    Union,
    Auxiliary,
}

impl NodeConcept {
    fn as_str(&self) -> &'static str {
        match self {
            NodeConcept::Expression => "expressions",
            NodeConcept::Statement => "statements",
            NodeConcept::Object => "objects",
            NodeConcept::Class => "classes",
            NodeConcept::Assignment => "assignments",
            NodeConcept::Binding => "bindings",
            NodeConcept::Type => "types",
            NodeConcept::Module => "module",
            NodeConcept::Unknown => "unknown",
            NodeConcept::List => "lists",
            NodeConcept::Union => "any",
            NodeConcept::Auxiliary => "auxiliary",
        }
    }
}

/// Convert an AstNode name to a path / Rust module name
///
/// Nodes are classified within the following concepts:
/// - expressions
/// - statements
/// - objects
/// - classes
/// - assignments
/// - bindings
/// - types
/// - module (import /export)
/// - unknown
/// - lists
/// - unions
/// - auxiliary (everything else)
fn name_to_path(kind: &NodeKind, in_name: &str) -> PathBuf {
    // Detect language prefix
    let (prefix, mut name) = in_name.split_at(2);
    let language = match prefix {
        "Js" => NodeLanguage::Js,
        "Ts" => NodeLanguage::Ts,
        _ => {
            eprintln!("missing prefix {}", in_name);
            name = in_name;
            NodeLanguage::Js
        }
    };

    // Classify nodes by concept
    let concept = match name {
        // JavaScript
        _ if matches!(kind, NodeKind::Unknown) => NodeConcept::Unknown,
        _ if matches!(kind, NodeKind::List { .. }) => NodeConcept::List,
        _ if matches!(kind, NodeKind::Union { .. }) => {
            if name.starts_with("Any") {
                name = &name[3..];
            }

            NodeConcept::Union
        }

        _ if name.ends_with("Statement") => NodeConcept::Statement,

        _ if name.ends_with("Expression")
            || name.ends_with("Argument")
            || name.ends_with("Arguments")
            || name.starts_with("Template") =>
        {
            NodeConcept::Expression
        }

        _ if name.ends_with("Binding")
            || name.starts_with("BindingPattern")
            || name.starts_with("ArrayBindingPattern")
            || name.starts_with("ObjectBindingPattern")
            || name.ends_with("Parameter")
            || name.ends_with("Parameters") =>
        {
            NodeConcept::Binding
        }

        _ if name.ends_with("Assignment")
            || name.starts_with("ArrayAssignmentPattern")
            || name.starts_with("ObjectAssignmentPattern") =>
        {
            NodeConcept::Assignment
        }
        "AssignmentWithDefault" => NodeConcept::Assignment,

        _ if name.ends_with("ImportSpecifier")
            || name.ends_with("ImportSpecifiers")
            || name.starts_with("Export")
            || name.starts_with("Import") =>
        {
            NodeConcept::Module
        }
        "Export" | "Import" | "ModuleSource" | "LiteralExportName" => NodeConcept::Module,

        _ if name.ends_with("ClassMember") => NodeConcept::Class,
        "ExtendsClause" => NodeConcept::Class,

        _ if name.ends_with("ObjectMember") | name.ends_with("MemberName") => NodeConcept::Object,

        // TypeScript
        "Assertion" | "ConstAssertion" | "NonNull" | "TypeArgs" | "ExprWithTypeArgs" => {
            NodeConcept::Expression
        }

        "ExternalModuleRef" | "ModuleRef" => NodeConcept::Module,

        _ if name.ends_with("Type") => NodeConcept::Type,

        // Default to auxiliary
        _ => NodeConcept::Auxiliary,
    };

    // Convert the names from CamelCase to snake_case
    let mut stem = String::new();
    for (index, char) in name.chars().enumerate() {
        if char.is_lowercase() {
            stem.push(char);
        } else {
            if index > 0 {
                stem.push('_');
            }
            for char in char.to_lowercase() {
                stem.push(char);
            }
        }
    }

    // "type" and "enum" are Rust keywords, add the "ts_"
    // prefix to these modules to avoid parsing errors
    let mut stem = match stem.as_str() {
        "type" => String::from("ts_type"),
        "enum" => String::from("ts_enum"),
        _ => stem,
    };

    stem.push_str(".rs");

    project_root()
        .join("crates/rome_formatter/src")
        .join(language.as_str())
        .join(concept.as_str())
        .join(stem)
}
