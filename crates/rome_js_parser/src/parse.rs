//! Utilities for high level parsing of js code.

use crate::*;
use rome_js_syntax::{
    AnyJsRoot, JsLanguage, JsModule, JsScript, JsSyntaxNode, ModuleKind, SourceType,
};
use rome_parser::event::Event;
use rome_parser::token_source::Trivia;
use rome_parser::AnyParse;

use rome_rowan::{AstNode, NodeCache};
use std::marker::PhantomData;

/// A utility struct for managing the result of a parser job
#[derive(Debug)]
pub struct Parse<T> {
    root: JsSyntaxNode,
    errors: Vec<ParseDiagnostic>,
    _ty: PhantomData<T>,
}

impl<T> Parse<T> {
    pub fn new_module(root: JsSyntaxNode, errors: Vec<ParseDiagnostic>) -> Parse<T> {
        Self::new(root, errors)
    }

    pub fn new_script(root: JsSyntaxNode, errors: Vec<ParseDiagnostic>) -> Parse<T> {
        Self::new(root, errors)
    }

    pub fn new(root: JsSyntaxNode, errors: Vec<ParseDiagnostic>) -> Parse<T> {
        Parse {
            root,
            errors,
            _ty: PhantomData,
        }
    }

    pub fn cast<N: AstNode<Language = JsLanguage>>(self) -> Option<Parse<N>> {
        if N::can_cast(self.syntax().kind()) {
            Some(Parse::new(self.root, self.errors))
        } else {
            None
        }
    }

    /// The syntax node represented by this Parse result
    ///
    /// ```
    /// use rome_js_parser::parse_script;
    /// use rome_js_syntax::{JsIfStatement, JsSyntaxKind};
    /// use rome_rowan::{AstNode, AstNodeList};
    ///
    /// let parse = parse_script(
    ///     "
    ///     if (a > 5) {
    ///         /* something */
    ///     }
    /// ",
    /// );
    ///
    /// // The first stmt in the root syntax node (Script) is the if statement.
    /// let if_stmt = parse.tree().statements().first().unwrap();
    ///
    /// assert_eq!(if_stmt.syntax().kind(), JsSyntaxKind::JS_IF_STATEMENT);
    /// ```
    pub fn syntax(&self) -> JsSyntaxNode {
        self.root.clone()
    }

    /// Get the diagnostics which occurred when parsing
    pub fn diagnostics(&self) -> &[ParseDiagnostic] {
        self.errors.as_slice()
    }

    /// Get the diagnostics which occurred when parsing
    pub fn into_diagnostics(self) -> Vec<ParseDiagnostic> {
        self.errors
    }

    /// Returns [true] if the parser encountered some errors during the parsing.
    pub fn has_errors(&self) -> bool {
        self.errors.iter().any(|diagnostic| diagnostic.is_error())
    }
}

impl<T> From<Parse<T>> for AnyParse
where
    T: AstNode,
    T::Language: 'static,
{
    fn from(parse: Parse<T>) -> Self {
        let root = parse.syntax();
        let diagnostics = parse.into_diagnostics();

        AnyParse::new(
            // SAFETY: the parser should always return a root node
            root.as_send().unwrap(),
            diagnostics,
        )
    }
}

impl<T: AstNode<Language = JsLanguage>> Parse<T> {
    /// Convert this parse result into a typed AST node.
    ///
    /// # Panics
    /// Panics if the node represented by this parse result mismatches.
    pub fn tree(&self) -> T {
        self.try_tree().unwrap_or_else(|| {
            panic!(
                "Expected tree to be a {} but root is:\n{:#?}",
                std::any::type_name::<T>(),
                self.syntax()
            )
        })
    }

    /// Try to convert this parse's untyped syntax node into an AST node.
    pub fn try_tree(&self) -> Option<T> {
        T::cast(self.syntax())
    }

    /// Convert this parse into a result
    pub fn ok(self) -> Result<T, Vec<ParseDiagnostic>> {
        if !self.errors.iter().any(|d| d.is_error()) {
            Ok(self.tree())
        } else {
            Err(self.errors)
        }
    }
}

fn parse_common(
    text: &str,
    source_type: SourceType,
) -> (Vec<Event<JsSyntaxKind>>, Vec<ParseDiagnostic>, Vec<Trivia>) {
    let mut parser = JsParser::new(text, source_type);
    syntax::program::parse(&mut parser);

    let (events, trivia, errors) = parser.finish();

    (events, errors, trivia)
}

/// Parse text into a [`Parse`](Parse) which can then be turned into an untyped root [`JsSyntaxNode`](JsSyntaxNode).
/// Or turned into a typed [`JsScript`](JsScript) with [`tree`](Parse::tree).
///
/// ```
/// use rome_js_parser::parse_script;
/// use rome_js_syntax::{JsSyntaxToken, SourceType, JsSyntaxList, JsComputedMemberExpression};
/// use rome_rowan::{AstNode, Direction};
///
/// let parse = parse_script("foo.bar[2]");
/// // Parse returns a JS Root which contains two lists, the directives and the statements, let's get the statements
/// let stmt = parse.syntax().children().nth(1).unwrap();
/// // The untyped syntax node of `foo.bar[2]`, the root node is `Script`.
/// let untyped_expr_node = stmt.first_child().unwrap();
///
/// // SyntaxNodes can be turned into a nice string representation.
/// println!("{:#?}", untyped_expr_node);
///
/// // You can then cast syntax nodes into a typed AST node.
/// let typed_ast_node = JsComputedMemberExpression::cast(untyped_expr_node.first_child().unwrap()).unwrap();
///
/// // Everything on every ast node is optional because of error recovery.
/// let prop = dbg!(typed_ast_node.member()).unwrap();
///
/// // You can then go back to an untyped SyntaxNode and get its range, text, parents, children, etc.
/// assert_eq!(prop.syntax().text(), "2");
///
/// // Util has a function for yielding all tokens of a node.
/// let tokens = untyped_expr_node.descendants_tokens(Direction::Next).map(|token| token.text_trimmed().to_string()).collect::<Vec<_>>();
///
/// assert_eq!(&tokens, &vec!["foo", ".", "bar", "[", "2", "]"]);
/// ```
pub fn parse_script(text: &str) -> Parse<JsScript> {
    parse(
        text,
        SourceType::js_module().with_module_kind(ModuleKind::Script),
    )
    .cast::<JsScript>()
    .unwrap()
}

/// Same as [parse_script] but configures the parser to parse an ECMAScript module instead of a script
///
/// ### Examples
///
/// Check the diagnostics emitted by the code
/// ```
/// use rome_js_parser::parse_module;
/// let source = r#"
/// import { someModule } from "./someModule.js";
///
/// someModule();
/// "#;
///
/// let parse = parse_module(source);
///
/// // Retrieve the diagnostics emitted
/// assert_eq!(parse.diagnostics().len(), 0);
/// ```
///
/// Retrieve the emitted AST and check its kind:
/// ```
/// use rome_js_parser::parse_module;
/// use rome_js_syntax::JsSyntaxKind;
/// use rome_rowan::AstNode;
/// let source = r#"
/// import { someModule } from "./someModule.js";
///
/// someModule();
/// "#;
/// let parse = parse_module(source);
///
/// let tree = parse.tree();
///
/// assert_eq!(tree.syntax().kind(), JsSyntaxKind::JS_MODULE);
/// ```
///
pub fn parse_module(text: &str) -> Parse<JsModule> {
    parse(text, SourceType::js_module())
        .cast::<JsModule>()
        .unwrap()
}

/// Parses the provided string as a EcmaScript program using the provided syntax features.
///
/// ### Examples
///
/// ```
/// use rome_js_parser::parse;
/// use rome_js_syntax::{LanguageVariant, LanguageVersion, ModuleKind, SourceType};
/// // parse source text as TypeScript
/// let mut module = SourceType::ts();
/// let mut parsed = parse("type F = {}", module);
/// assert_eq!(parsed.diagnostics().len(), 0);
/// // parse source text as JSX
/// module = SourceType::jsx();
/// parsed = parse("<Component></Component>", module);
/// assert_eq!(parsed.diagnostics().len(), 0);
/// // parse source text with granular control
/// module = SourceType::default()
/// 	.with_version(LanguageVersion::ESNext)
/// 	.with_module_kind(ModuleKind::Module)
/// 	.with_variant(LanguageVariant::Jsx);
/// parsed = parse("foo[bar]", module);
/// assert_eq!(parsed.diagnostics().len(), 0);
/// ```
pub fn parse(text: &str, source_type: SourceType) -> Parse<AnyJsRoot> {
    let mut cache = NodeCache::default();
    parse_js_with_cache(text, source_type, &mut cache)
}

/// Parses the provided string as a EcmaScript program using the provided syntax features and node cache.
///
/// ### Examples
///
/// ```
/// use rome_js_parser::parse_js_with_cache;
/// use rome_js_syntax::SourceType;
/// use rome_rowan::NodeCache;
///
/// let source_type = SourceType::js_module();
/// let mut cache = NodeCache::default();
/// let mut source = "function f() { return 2 }";
///
/// let parsed = parse_js_with_cache(source, source_type, &mut cache);
///	assert_eq!(parsed.diagnostics().len(), 0);
///
/// source = "function bar() { return 3 }";
/// let parsed  = parse_js_with_cache(source, source_type, &mut cache);
///	assert_eq!(parsed.diagnostics().len(), 0);
///
/// ```
pub fn parse_js_with_cache(
    text: &str,
    source_type: SourceType,
    cache: &mut NodeCache,
) -> Parse<AnyJsRoot> {
    tracing::debug_span!("parse").in_scope(move || {
        let (events, errors, tokens) = parse_common(text, source_type);
        let mut tree_sink = JsLosslessTreeSink::with_cache(text, &tokens, cache);
        rome_parser::event::process(&mut tree_sink, events, errors);
        let (green, parse_errors) = tree_sink.finish();
        Parse::new(green, parse_errors)
    })
}
