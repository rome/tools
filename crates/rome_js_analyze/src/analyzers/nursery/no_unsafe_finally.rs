use rome_analyze::context::RuleContext;
use rome_analyze::{declare_rule, Ast, Rule, RuleDiagnostic};
use rome_console::markup;
use rome_js_syntax::*;
use rome_rowan::{declare_node_union, AstNode};

declare_rule! {
    /// Disallow control flow statements in finally blocks.
    ///
    /// JavaScript suspends the control flow statements of `try` and `catch` blocks until
    /// the execution of finally block finishes. So, when `return`, `throw, `break`, or `continue`
    /// is used in finally, control flow statements inside `try` and `catch` are overwritten,
    /// which is considered as unexpected behavior.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```js,expect_diagnostic
    /// (() => {
    ///     try {
    ///         return 1; // 1 is returned but suspended until finally block ends
    ///     } catch(err) {
    ///         return 2;
    ///     } finally {
    ///         return 3; // 3 is returned before 1, which we did not expect
    ///     }
    /// })();
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// (() => {
    ///     try {
    ///         throw new Error("Try"); // error is thrown but suspended until finally block ends
    ///     } finally {
    ///         return 3; // 3 is returned before the error is thrown, which we did not expect
    ///     }
    /// })();
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// (() => {
    ///     try {
    ///         throw new Error("Try")
    ///     } catch(err) {
    ///         throw err; // The error thrown from try block is caught and re-thrown
    ///     } finally {
    ///         throw new Error("Finally"); // Finally(...) is thrown, which we did not expect
    ///     }
    /// })();
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// (() => {
    ///     label: try {
    ///       return 0; // 0 is returned but suspended until finally block ends
    ///     } finally {
    ///       break label; // It breaks out the try-finally block, before 0 is returned.
    ///     }
    ///     return 1;
    /// })();
    /// ```
    /// 
    /// ```js,expect_diagnostic
    /// function a() {
    ///   switch (condition) {
    ///     case 'a': {
    ///       try {
    ///         console.log('a');
    ///         return;
    ///       } finally {
    ///         break;
    ///       }
    ///     }
    ///     case 'b': {
    ///       console.log('b');
    ///     }
    ///   }
    /// }
    ///```
    ///
    /// ### Valid
    ///
    /// ```js
    /// let foo = function() {
    ///     try {
    ///         return 1;
    ///     } catch(err) {
    ///         return 2;
    ///     } finally {
    ///         console.log("hola!");
    ///     }
    /// };
    /// ```
    ///
    /// ```js
    /// let foo = function() {
    ///     try {
    ///         return 1;
    ///     } catch(err) {
    ///         return 2;
    ///     } finally {
    ///         let a = function() {
    ///             return "hola!";
    ///         }
    ///     }
    /// };
    /// ```
    ///
    /// ```js
    /// let foo = function(a) {
    ///     try {
    ///         return 1;
    ///     } catch(err) {
    ///         return 2;
    ///     } finally {
    ///         switch(a) {
    ///             case 1: {
    ///                 console.log("hola!")
    ///                 break;
    ///             }
    ///         }
    ///     }
    /// };
    /// ```
    ///
    pub(crate) NoUnsafeFinally {
        version: "11.0.0",
        name: "noUnsafeFinally",
        recommended: false,
    }
}

declare_node_union! {
    pub(crate) ControlFlowStatement = JsReturnStatement | JsThrowStatement | JsBreakStatement | JsContinueStatement
}

impl Rule for NoUnsafeFinally {
    type Query = Ast<ControlFlowStatement>;
    type State = ();
    type Signals = Option<Self::State>;
    type Options = ();

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let query = ctx.query();
        if query.in_finally_block()? {
            Some(())
        } else {
            None
        }
    }

    fn diagnostic(ctx: &RuleContext<Self>, _: &Self::State) -> Option<RuleDiagnostic> {
        let query = ctx.query();
        Some(RuleDiagnostic::new(
            rule_category!(),
            query.syntax().text_trimmed_range(),
            markup! {
                "Unsafe usage of '"{ query.description() }"'."
            },
        ))
    }
}

impl ControlFlowStatement {
    fn in_finally_block(&self) -> Option<bool> {
        let mut node = self.syntax().clone();
        let mut is_label_inside_finally = false;
        let check_sentinel = self.sentinel_check();
        let label = self.label_token();

        while !check_sentinel(node.kind()) {
            if let Some(label) = &label {
                if let Some(parent) = node.parent().and_then(JsLabeledStatement::cast) {
                    if parent
                        .label_token()
                        .ok()
                        .map_or(false, |it| it.text_trimmed() == label.text_trimmed())
                    {
                        is_label_inside_finally = true;
                    }
                }
            }
            if node.kind() == JsSyntaxKind::JS_FINALLY_CLAUSE {
                return Some(!is_label_inside_finally);
            }
            node = node.parent()?;
        }

        Some(false)
    }

    fn label_token(&self) -> Option<JsSyntaxToken> {
        match self {
            Self::JsReturnStatement(_) | Self::JsThrowStatement(_) => None,
            Self::JsBreakStatement(it) => it.label_token(),
            Self::JsContinueStatement(it) => it.label_token(),
        }
    }

    fn sentinel_check(&self) -> fn(JsSyntaxKind) -> bool {
        match self {
            Self::JsBreakStatement(it) if it.label_token().is_none() => sentinel_for_break,
            Self::JsContinueStatement(_) => sentinel_for_continue,
            _ => sentinel_for_throw_or_return,
        }
    }

    fn description(&self) -> &str {
        match self {
            Self::JsReturnStatement(_) => "return",
            Self::JsThrowStatement(_) => "throw",
            Self::JsBreakStatement(_) => "break",
            Self::JsContinueStatement(_) => "continue",
        }
    }
}

fn sentinel_for_break(kind: JsSyntaxKind) -> bool {
    use JsSyntaxKind::*;
    matches!(
        kind,
        JS_SCRIPT
            | JS_MODULE
            | JS_FUNCTION_DECLARATION
            | JS_CLASS_DECLARATION
            | JS_FUNCTION_EXPRESSION
            | JS_CLASS_EXPRESSION
            | JS_ARROW_FUNCTION_EXPRESSION
            | JS_DO_WHILE_STATEMENT
            | JS_WHILE_STATEMENT
            | JS_FOR_OF_STATEMENT
            | JS_FOR_IN_STATEMENT
            | JS_FOR_STATEMENT
            | JS_SWITCH_STATEMENT
    )
}

fn sentinel_for_continue(kind: JsSyntaxKind) -> bool {
    use JsSyntaxKind::*;
    matches!(
        kind,
        JS_SCRIPT
            | JS_MODULE
            | JS_FUNCTION_DECLARATION
            | JS_CLASS_DECLARATION
            | JS_FUNCTION_EXPRESSION
            | JS_CLASS_EXPRESSION
            | JS_ARROW_FUNCTION_EXPRESSION
            | JS_DO_WHILE_STATEMENT
            | JS_WHILE_STATEMENT
            | JS_FOR_OF_STATEMENT
            | JS_FOR_IN_STATEMENT
            | JS_FOR_STATEMENT
    )
}

fn sentinel_for_throw_or_return(kind: JsSyntaxKind) -> bool {
    use JsSyntaxKind::*;
    matches!(
        kind,
        JS_SCRIPT
            | JS_MODULE
            | JS_FUNCTION_DECLARATION
            | JS_CLASS_DECLARATION
            | JS_FUNCTION_EXPRESSION
            | JS_CLASS_EXPRESSION
            | JS_ARROW_FUNCTION_EXPRESSION
    )
}
