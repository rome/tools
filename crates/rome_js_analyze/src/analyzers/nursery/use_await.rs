use rome_analyze::{
    context::RuleContext, declare_rule, AddVisitor, Phases, QueryMatch, Queryable, Rule,
    RuleDiagnostic, ServiceBag, Visitor, VisitorContext,
};
use rome_console::markup;
use rome_js_syntax::{JsAwaitExpression, JsForOfStatement, JsLanguage, TextRange, WalkEvent};
use rome_rowan::{AstNode, AstNodeList, Language, SyntaxNode};

use super::use_yield::AnyFunctionLike;

declare_rule! {
    /// Disallow `async` functions that have no `await`.
    ///
    /// Asynchronous functions in _JavaScript_ behave differently than other functions:
    ///
    /// 1. The return value is always a _Promise_.
    /// 2. You can use the `await` operator inside of them.
    ///
    /// The primary reason to use asynchronous functions is typically to use the
    /// `await` operator.
    ///
    /// Asynchronous functions that don’t use `await` might not need to be asynchronous
    /// functions and could be the unintentional result of refactoring.
    ///
    /// Note: this rule ignores asynchronous generator functions.
    /// This is because generators `yield` rather than `return` a value and `async` generators might
    /// `yield` all the values of another `async` generator without ever actually needing to use `await`.
    ///
    /// Source: https://eslint.org/docs/latest/rules/require-await
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```js,expect_diagnostic
    /// async function foo() {
    ///     doSomething();
    /// }
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// bar(async () => {
    ///     doSomething();
    /// });
    /// ```
    ///
    /// ## Valid
    ///
    /// ```js
    /// async function foo() {
    ///     await doSomething();
    /// }
    /// ```
    ///
    /// ```js
    /// bar(async () => {
    ///     await doSomething();
    /// });
    /// ```
    ///
    /// ```js
    /// function foo() {
    ///     doSomething();
    /// }
    /// ```
    ///
    /// ```js
    /// bar(() => {
    ///     doSomething();
    /// });
    /// ```
    ///
    /// ```js
    /// // Allow empty functions.
    /// async function noop() {}
    /// ```
    pub(crate) UseAwait {
        version: "next",
        name: "useAwait",
        recommended: false,
    }
}

impl Rule for UseAwait {
    type Query = MissingAwait;
    type State = ();
    type Signals = Option<Self::State>;
    type Options = ();

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let node = ctx.query();
        // Only report non-empty async functions.
        // Async generators are skipped (cf rule description).
        if !node.is_async()
            || node.is_generator()
            || node.statements().filter(|x| x.is_empty()).is_some()
        {
            return None;
        }
        Some(())
    }

    fn diagnostic(ctx: &RuleContext<Self>, _: &Self::State) -> Option<RuleDiagnostic> {
        Some(RuleDiagnostic::new(
            rule_category!(),
            ctx.query().range(),
            markup! {"This "<Emphasis>"async"</Emphasis>" function doesn't use "<Emphasis>"await"</Emphasis>"."},
        ).note(markup! {"Remove the "<Emphasis>"async"</Emphasis>" modifier or use "<Emphasis>"await"</Emphasis>"."}))
    }
}

pub(crate) struct MissingAwait(AnyFunctionLike);

impl QueryMatch for MissingAwait {
    fn text_range(&self) -> TextRange {
        self.0.range()
    }
}

impl Queryable for MissingAwait {
    type Input = Self;
    type Language = JsLanguage;
    type Output = AnyFunctionLike;
    type Services = ();

    fn build_visitor(
        analyzer: &mut impl AddVisitor<Self::Language>,
        _: &<Self::Language as Language>::Root,
    ) {
        analyzer.add_visitor(Phases::Syntax, MissingAwaitVisitor::default);
    }

    fn unwrap_match(_: &ServiceBag, query: &Self::Input) -> Self::Output {
        query.0.clone()
    }
}

#[derive(Default)]
struct MissingAwaitVisitor {
    /// Vector to hold a function node and a boolean indicating whether the function
    /// contains an `await` expression or not.
    stack: Vec<(AnyFunctionLike, bool)>,
}

impl Visitor for MissingAwaitVisitor {
    type Language = JsLanguage;

    fn visit(
        &mut self,
        event: &WalkEvent<SyntaxNode<Self::Language>>,
        mut ctx: VisitorContext<Self::Language>,
    ) {
        match event {
            WalkEvent::Enter(node) => {
                // When the visitor enters a function node, push a new entry on the stack
                if let Some(node) = AnyFunctionLike::cast_ref(node) {
                    self.stack.push((node, false));
                }
                if JsAwaitExpression::can_cast(node.kind()) {
                    // When the visitor enters a `await` expression, set the
                    // `has_await` flag for the top entry on the stack to `true`
                    if let Some((_, has_await)) = self.stack.last_mut() {
                        *has_await = true;
                    }
                }
                if let Some(node) = JsForOfStatement::cast_ref(node) {
                    if node.await_token().is_some() {
                        // When the visitor enters a `for await`, set the
                        // `has_await` flag for the top entry on the stack to `true`
                        if let Some((_, has_await)) = self.stack.last_mut() {
                            *has_await = true;
                        }
                    }
                }
            }
            WalkEvent::Leave(node) => {
                // When the visitor exits a function, if it matches the node of the top-most
                // entry of the stack and the `has_await` flag is `false`, emit a query match
                if let Some(exit_node) = AnyFunctionLike::cast_ref(node) {
                    if let Some((enter_node, has_await)) = self.stack.pop() {
                        if enter_node == exit_node && !has_await {
                            ctx.match_query(MissingAwait(enter_node));
                        }
                    }
                }
            }
        }
    }
}
