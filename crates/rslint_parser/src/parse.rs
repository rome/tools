//! Utilities for high level parsing of js code.

use crate::token_source::Token;
use crate::*;
use rome_js_syntax::{AstNode, JsAnyRoot, JsExpressionSnipped, JsModule, JsScript, SyntaxNode};
use rslint_errors::Severity;
use std::marker::PhantomData;

/// A utility struct for managing the result of a parser job
#[derive(Debug, Clone)]
pub struct Parse<T> {
    root: SyntaxNode,
    errors: Vec<ParserError>,
    _ty: PhantomData<T>,
}

impl<T> Parse<T> {
    pub fn new_module(root: SyntaxNode, errors: Vec<ParserError>) -> Parse<T> {
        Self::new(root, errors)
    }

    pub fn new_script(root: SyntaxNode, errors: Vec<ParserError>) -> Parse<T> {
        Self::new(root, errors)
    }

    pub fn new(root: SyntaxNode, errors: Vec<ParserError>) -> Parse<T> {
        Parse {
            root,
            errors,
            _ty: PhantomData,
        }
    }

    pub fn cast<N: AstNode>(self) -> Option<Parse<N>> {
        if N::can_cast(self.syntax().kind()) {
            Some(Parse::new(self.root, self.errors))
        } else {
            None
        }
    }

    /// The syntax node represented by this Parse result
    ///
    /// ```
    /// use rslint_parser::parse_script;
    /// use rome_js_syntax::{JsIfStatement, SyntaxNodeExt, JsSyntaxKind, AstNode, AstNodeList};
    ///
    /// let parse = parse_script(
    /// "
    ///     if (a > 5) {
    ///         /* something */
    ///     }
    /// ", 0);
    ///
    /// // The first stmt in the root syntax node (Script) is the if statement.
    /// let if_stmt = parse.tree().statements().first().unwrap();
    ///
    /// assert_eq!(if_stmt.syntax().kind(), JsSyntaxKind::JS_IF_STATEMENT);
    /// ```
    pub fn syntax(&self) -> SyntaxNode {
        self.root.clone()
    }

    /// Get the diagnostics which occurred when parsing
    pub fn diagnostics(&self) -> &[Diagnostic] {
        self.errors.as_slice()
    }

    /// Get the diagnostics which occurred when parsing
    pub fn into_diagnostics(self) -> Vec<Diagnostic> {
        self.errors
    }

    /// Returns [true] if the parser encountered some errors during the parsing.
    pub fn has_errors(&self) -> bool {
        self.errors.iter().any(|diagnostic| diagnostic.is_error())
    }
}

impl<T: AstNode> Parse<T> {
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
    pub fn ok(self) -> Result<T, Vec<ParserError>> {
        if !self.errors.iter().any(|d| d.severity == Severity::Error) {
            Ok(self.tree())
        } else {
            Err(self.errors)
        }
    }
}

fn parse_common(
    text: &str,
    file_id: usize,
    source_type: SourceType,
) -> (Vec<Event>, Vec<ParserError>, Vec<Token>) {
    let mut parser = crate::Parser::new(text, file_id, source_type);
    crate::syntax::program::parse(&mut parser);

    let (events, tokens, errors) = parser.finish();

    (events, errors, tokens)
}

/// Parse text into a [`Parse`](Parse) which can then be turned into an untyped root [`SyntaxNode`](SyntaxNode).
/// Or turned into a typed [`Script`](Script) with [`tree`](Parse::tree).
///
/// ```
/// use rslint_parser::parse_script;
/// use rome_js_syntax::{AstNode, SyntaxToken, SyntaxNodeExt,  SyntaxList, util, JsComputedMemberExpression};
///
/// let parse = parse_script("foo.bar[2]", 0);
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
/// let tokens = untyped_expr_node.tokens();
///
/// assert_eq!(&util::concat_tokens(&tokens), "foo.bar[2]")
/// ```
pub fn parse_script(text: &str, file_id: usize) -> Parse<JsScript> {
    parse(
        text,
        file_id,
        SourceType::js_module().with_module_kind(ModuleKind::Script),
    )
    .cast::<JsScript>()
    .unwrap()
}

/// Lossly parse text into a [`Parse`](Parse) which can then be turned into an untyped root [`SyntaxNode`](SyntaxNode).
/// Or turned into a typed [`Script`](Script) with [`tree`](Parse::tree).
///
/// Unlike [`parse_text`], the final parse result includes no whitespace, it does however include errors.
///
/// Note however that the ranges and text of nodes still includes whitespace! Therefore you should trim text before rendering it.
/// The [`util`](crate::util) module has utility functions for dealing with this easily.
///
/// ```
/// use rslint_parser::parse_script_lossy;
/// use rome_js_syntax::{JsComputedMemberExpression, AstNode, SyntaxToken, SyntaxNodeExt, util, SyntaxList};
///
/// let parse = parse_script_lossy("foo.bar[2]", 0);
/// // Parse returns a JS Root with two children, an empty list of directives and the list of statements, let's get the statements
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
/// let prop = typed_ast_node.member().unwrap();
///
/// // You can then go back to an untyped SyntaxNode and get its range, text, parents, children, etc.
/// assert_eq!(prop.syntax().text(), "2");
///
/// // Util has a function for yielding all tokens of a node.
/// let tokens = untyped_expr_node.tokens();
///
/// // End result does not include whitespace because the parsing is lossy in this case
/// assert_eq!(&util::concat_tokens(&tokens), "foo.bar[2]")
/// ```
pub fn parse_script_lossy(text: &str, file_id: usize) -> Parse<JsScript> {
    let (events, errors, tokens) = parse_common(
        text,
        file_id,
        SourceType::js_module().with_module_kind(ModuleKind::Script),
    );
    let mut tree_sink = LossyTreeSink::new(text, &tokens);
    crate::process(&mut tree_sink, events, errors);
    let (green, parse_errors) = tree_sink.finish();
    Parse::new_script(green, parse_errors)
}

/// Same as [`parse_text_lossy`] but configures the parser to parse an ECMAScript module instead of a Script
pub fn parse_module_lossy(text: &str, file_id: usize) -> Parse<JsModule> {
    let (events, errors, tokens) = parse_common(text, file_id, SourceType::js_module());
    let mut tree_sink = LossyTreeSink::new(text, &tokens);
    crate::process(&mut tree_sink, events, errors);
    let (green, parse_errors) = tree_sink.finish();
    Parse::new_module(green, parse_errors)
}

/// Same as [`parse_text`] but configures the parser to parse an ECMAScript module instead of a script
pub fn parse_module(text: &str, file_id: usize) -> Parse<JsModule> {
    parse(text, file_id, SourceType::js_module())
        .cast::<JsModule>()
        .unwrap()
}

/// Parses the provided string as a EcmaScript program using the provided syntax features.
pub fn parse(text: &str, file_id: usize, source_type: SourceType) -> Parse<JsAnyRoot> {
    let (events, errors, tokens) = parse_common(text, file_id, source_type);
    let mut tree_sink = LosslessTreeSink::new(text, &tokens);
    crate::process(&mut tree_sink, events, errors);
    let (green, parse_errors) = tree_sink.finish();
    Parse::new(green, parse_errors)
}

/// Losslessly Parse text into an expression [`Parse`](Parse) which can then be turned into an untyped root [`SyntaxNode`](SyntaxNode).
/// Or turned into a typed [`Expr`](Expr) with [`tree`](Parse::tree).
pub fn parse_expression(text: &str, file_id: usize) -> Parse<JsExpressionSnipped> {
    let mut parser = crate::Parser::new(text, file_id, SourceType::js_module());
    crate::syntax::expr::parse_expression_snipped(&mut parser).unwrap();
    let (events, tokens, errors) = parser.finish();

    let mut tree_sink = LosslessTreeSink::new(text, &tokens);
    crate::process(&mut tree_sink, events, errors);
    let (green, parse_errors) = tree_sink.finish();
    Parse::new_script(green, parse_errors)
}
