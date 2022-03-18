use std::{
    collections::{BTreeMap, BTreeSet, HashSet, VecDeque},
    env,
    fs::{create_dir_all, read_dir, remove_file, File},
    io::Write,
    path::{Path, PathBuf},
};

use git2::{Repository, Status, StatusOptions};
use proc_macro2::{Ident, Span};
use quote::quote;
use xtask::project_root;

use crate::ast::load_ast;

struct GitRepo {
    repo: Repository,
    allow_staged: bool,
    staged: HashSet<PathBuf>,
    dirty: HashSet<PathBuf>,
}

impl GitRepo {
    fn open() -> Self {
        let root = project_root();
        let repo = Repository::discover(&root).expect("failed to open git repo");

        let mut allow_staged = false;
        let mut allow_dirty = false;
        for arg in env::args() {
            match arg.as_str() {
                "--allow-staged" => {
                    allow_staged = true;
                }
                "--allow-dirty" => {
                    allow_dirty = true;
                }
                _ => {}
            }
        }

        let mut repo_opts = StatusOptions::new();
        repo_opts.include_ignored(false);

        let statuses = repo
            .statuses(Some(&mut repo_opts))
            .expect("failed to read repository status");

        let mut staged = HashSet::new();
        let mut dirty = HashSet::new();

        for status in statuses.iter() {
            if let Some(path) = status.path() {
                match status.status() {
                    Status::CURRENT => (),
                    Status::INDEX_NEW
                    | Status::INDEX_MODIFIED
                    | Status::INDEX_DELETED
                    | Status::INDEX_RENAMED
                    | Status::INDEX_TYPECHANGE => {
                        if !allow_staged {
                            staged.insert(root.join(path));
                        }
                    }
                    _ => {
                        if !allow_dirty {
                            dirty.insert(root.join(path));
                        }
                    }
                };
            }
        }

        drop(statuses);

        Self {
            repo,
            allow_staged,
            staged,
            dirty,
        }
    }

    fn check_path(&self, path: &Path) {
        if self.dirty.contains(path) {
            panic!("Codegen would overwrite '{}' but it has uncommited changes. Commit the file to git, or pass --allow-dirty to the command to proceed anyway", path.display());
        }
        if self.staged.contains(path) {
            panic!("Codegen would overwrite '{}' but it has uncommited changes. Commit the file to git, or pass --allow-staged to the command to proceed anyway", path.display());
        }
    }

    fn stage_paths(&self, paths: &[PathBuf]) {
        // Do not overwrite a version of the file
        // that's potentially already staged
        if self.allow_staged {
            return;
        }

        let root = project_root();
        self.repo
            .index()
            .expect("could not open index for git repository")
            .update_all(
                paths.iter().map(|path| {
                    path.strip_prefix(&root).unwrap_or_else(|err| {
                        panic!(
                            "path '{}' is not inside of project '{}': {}",
                            path.display(),
                            root.display(),
                            err,
                        )
                    })
                }),
                None,
            )
            .expect("failed to stage updated files");
    }
}

struct ModuleIndex {
    root: PathBuf,
    modules: BTreeMap<PathBuf, BTreeSet<String>>,
    unused_files: HashSet<PathBuf>,
}

impl ModuleIndex {
    fn new(root: PathBuf) -> Self {
        let mut unused_files = HashSet::new();
        let mut queue = VecDeque::new();

        queue.push_back(root.join("js"));
        queue.push_back(root.join("ts"));
        queue.push_back(root.join("jsx"));

        while let Some(dir) = queue.pop_front() {
            let iter = read_dir(&dir)
                .unwrap_or_else(|err| panic!("failed to read '{}': {}", dir.display(), err));

            for entry in iter {
                let entry = entry.expect("failed to read DirEntry");

                let path = entry.path();
                let file_type = entry.file_type().unwrap_or_else(|err| {
                    panic!("failed to read file type of '{}': {}", path.display(), err)
                });

                if file_type.is_dir() {
                    queue.push_back(path);
                    continue;
                }

                if file_type.is_file() {
                    unused_files.insert(path);
                }
            }
        }

        Self {
            root,
            modules: BTreeMap::default(),
            unused_files,
        }
    }

    /// Add a new module to the index
    fn insert(&mut self, repo: &GitRepo, path: &Path) {
        self.unused_files.remove(path);

        // Walk up from the module file towards the root
        let mut parent = path.parent();
        let mut file_stem = path.file_stem();

        while let (Some(path), Some(stem)) = (parent, file_stem) {
            repo.check_path(&path.join("mod.rs"));

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
    fn print(mut self, stage: &mut Vec<PathBuf>) {
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
            drop(file);

            self.unused_files.remove(&path);
            stage.push(path);
        }

        for file in self.unused_files {
            remove_file(&file)
                .unwrap_or_else(|err| panic!("failed to delete '{}': {}", file.display(), err));
            stage.push(file);
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
    let repo = GitRepo::open();

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

    let mut stage = Vec::new();

    // Create a default implementation for theses nodes only if
    // the file doesn't already exist
    for (kind, name) in names {
        let path = name_to_path(&kind, &name);
        modules.insert(&repo, &path);

        // Union nodes except for AnyFunction and AnyClass have a generated
        // implementation, the codegen will always overwrite any existing file
        let allow_overwrite = matches!(kind, NodeKind::Union { .. })
            && name != "JsAnyFunction"
            && name != "JsAnyClass";

        if !allow_overwrite && path.exists() {
            continue;
        }

        repo.check_path(&path);

        let dir = path.parent().unwrap();
        create_dir_all(dir).unwrap();

        let id = Ident::new(&name, Span::call_site());

        // Generate a default implementation of ToFormatElement using format_list on
        // non-separated lists, to_format_element on the wrapped node for unions and
        // format_verbatim for all the other nodes
        let tokens = match kind {
            NodeKind::List { separated: false } => quote! {
                use crate::{FormatElement, FormatResult, Formatter, ToFormatElement};
                use rome_js_syntax::#id;

                impl ToFormatElement for #id {
                    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
                        Ok(formatter.format_list(self.clone()))
                    }
                }
            },
            NodeKind::Node | NodeKind::Unknown | NodeKind::List { separated: true } => {
                quote! {
                    use crate::{FormatElement, FormatResult, Formatter, ToFormatElement};
                    use rome_js_syntax::{#id, AstNode};

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
                    use rome_js_syntax::#id;

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

        let tokens = if allow_overwrite {
            xtask::reformat(tokens).unwrap()
        } else {
            xtask::reformat_without_preamble(tokens).unwrap()
        };

        let mut file = File::create(&path).unwrap();
        file.write_all(tokens.as_bytes()).unwrap();
        drop(file);

        stage.push(path);
    }

    modules.print(&mut stage);
    repo.stage_paths(&stage);
}

enum NodeLanguage {
    Js,
    Ts,
    JSX,
}

impl NodeLanguage {
    fn is_jsx(&self) -> bool {
        matches!(self, NodeLanguage::Jsx)
    }

    fn as_str(&self) -> &'static str {
        match self {
            NodeLanguage::Js => "js",
            NodeLanguage::Ts => "ts",
            NodeLanguage::JSX => "jsx",
        }
    }
}

enum NodeConcept {
    Expression,
    Statement,
    Declaration,
    Object,
    Class,
    Assignment,
    Binding,
    Type,
    Module,
    Unknown,
    List,
    Union,
    Tag,
    Attribute,
    Auxiliary,
}

impl NodeConcept {
    fn as_str(&self) -> &'static str {
        match self {
            NodeConcept::Expression => "expressions",
            NodeConcept::Statement => "statements",
            NodeConcept::Declaration => "declarations",
            NodeConcept::Object => "objects",
            NodeConcept::Class => "classes",
            NodeConcept::Assignment => "assignments",
            NodeConcept::Binding => "bindings",
            NodeConcept::Type => "types",
            NodeConcept::Module => "module",
            NodeConcept::Unknown => "unknown",
            NodeConcept::List => "lists",
            NodeConcept::Union => "any",
            NodeConcept::Tag => "tag",
            NodeConcept::Attribute => "attribute",
            NodeConcept::Auxiliary => "auxiliary",
        }
    }
}

/// Convert an AstNode name to a path / Rust module name
///
/// Nodes are classified within the following concepts:
/// - expressions
/// - statements
/// - declarations
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
    let mid_before_second_capital_letter = in_name
        .chars()
        .position({
            let mut uppercases = 0;
            move |c| {
                uppercases += if c.is_uppercase() { 1 } else { 0 };
                uppercases >= 2
            }
        })
        .expect("Node name malformed");
    let (prefix, mut name) = in_name.split_at(mid_before_second_capital_letter);
    let language = match prefix {
        "Jsx" => NodeLanguage::JSX,
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
        _ if name.ends_with("Declaration") => NodeConcept::Declaration,

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

        _ if language.is_jsx()
            && (name.ends_with("Element")
                || name.ends_with("Tag")
                || name.ends_with("Fragment")) =>
        {
            NodeConcept::Tag
        }
        _ if language.is_jsx() && name.contains("Attribute") => NodeConcept::Attribute,

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
