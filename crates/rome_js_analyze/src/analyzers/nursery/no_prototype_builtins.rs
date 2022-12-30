use rome_analyze::{context::RuleContext, declare_rule, Ast, Rule, RuleDiagnostic};
use rome_console::markup;
use rome_js_syntax::{AnyJsExpression, JsCallExpression, JsCallExpressionFields, TextRange};
use rome_rowan::AstNodeList;

declare_rule! {
    /// Disallow direct use of `Object.prototype` builtins.
    ///
    /// ECMAScript 5.1 added `Object.create` which allows the creation of an object with a custom prototype.
    /// This pattern is often used for objects used as Maps. However, this pattern can lead to errors
    /// if something else relies on prototype properties/methods.
    /// Moreover, the methods could be shadowed, this can lead to random bugs and denial of service
    /// vulnerabilities. For example, calling `hasOwnProperty` directly on parsed JSON like `{"hasOwnProperty": 1}` could lead to vulnerabilities.
    /// To avoid subtle bugs like this, you should call these methods from `Object.prototype`.
    /// For example, `foo.isPrototypeof(bar)` should be replaced with `Object.prototype.isPrototypeof.call(foo, "bar")`
    /// As for the `hasOwn` method, `foo.hasOwn("bar")` should be replaced with `Object.hasOwn(foo, "bar")`.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```js,expect_diagnostic
    /// var invalid = foo.hasOwnProperty("bar");
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// var invalid = foo.isPrototypeOf(bar);
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// var invalid = foo.propertyIsEnumerable("bar");
    /// ```
    ///
    /// ## Valid
    ///
    /// ```js
    /// var valid = Object.hasOwn(foo, "bar");
    /// var valid = Object.prototype.isPrototypeOf.call(foo, bar);
    /// var valid = {}.propertyIsEnumerable.call(foo, "bar");
    /// ```
    ///
    pub(crate) NoPrototypeBuiltins {
        version: "next",
        name: "noPrototypeBuiltins",
        recommended: false,
    }
}

pub(crate) struct RuleState {
    prototype_builtins_method_name: String,
    text_range: TextRange,
}

impl Rule for NoPrototypeBuiltins {
    type Query = Ast<JsCallExpression>;
    type State = RuleState;
    type Signals = Option<Self::State>;
    type Options = ();

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let call_expr = ctx.query();
        let mut callee = call_expr.callee().ok()?;

        // We need to handle a parenthesized expression case e.g. `(foo?.hasOwnProperty)("bar");`
        if let AnyJsExpression::JsParenthesizedExpression(expr) = callee {
            callee = match expr.expression().ok()? {
                AnyJsExpression::JsStaticMemberExpression(expr) => {
                    AnyJsExpression::JsStaticMemberExpression(expr)
                }
                AnyJsExpression::JsComputedMemberExpression(expr) => {
                    AnyJsExpression::JsComputedMemberExpression(expr)
                }
                _ => return None,
            }
        }

        match callee {
            AnyJsExpression::JsComputedMemberExpression(expr) => {
                let expr = expr.member().ok()?;
                match expr {
                    AnyJsExpression::AnyJsLiteralExpression(expr) => {
                        let literal_expr = expr.as_js_string_literal_expression()?;
                        let token_text = literal_expr.inner_string_text().ok()?;
                        let token = literal_expr.value_token().ok()?;

                        is_prototype_builtins(token_text.text()).then_some(RuleState {
                            prototype_builtins_method_name: token_text.to_string(),
                            text_range: token.text_range(),
                        })
                    }
                    AnyJsExpression::JsTemplateExpression(expr) => {
                        let template_element = expr.as_fields().elements.first()?;
                        let template_chunk_token = template_element
                            .as_js_template_chunk_element()?
                            .template_chunk_token()
                            .ok()?;
                        let token_text = template_chunk_token.text();
                        is_prototype_builtins(token_text).then_some(RuleState {
                            prototype_builtins_method_name: token_text.to_string(),
                            text_range: template_chunk_token.text_trimmed_range(),
                        })
                    }
                    _ => None,
                }
            }
            AnyJsExpression::JsStaticMemberExpression(expr) => {
                let member = expr.as_fields().member.ok()?;
                let value_token = member.as_js_name()?.value_token().ok()?;
                let token_text = value_token.text();
                is_prototype_builtins(token_text).then_some(RuleState {
                    prototype_builtins_method_name: token_text.to_string(),
                    text_range: value_token.text_range(),
                })
            }
            _ => None,
        }
    }

    fn diagnostic(_ctx: &RuleContext<Self>, state: &Self::State) -> Option<RuleDiagnostic> {
        let diag = RuleDiagnostic::new(
            rule_category!(),
            state.text_range,
            markup! {
                "Do not access Object.prototype method '"{&state.prototype_builtins_method_name}"' from target object."
            },
        );

        if state.prototype_builtins_method_name == "hasOwnProperty" {
            Some(
                diag.note(markup! {
                    "It's recommended using "<Emphasis>"Object.hasOwn()"</Emphasis>" instead of using "<Emphasis>"Object.hasOwnProperty()"</Emphasis>"."
                })
                .note(markup! {
                    "See "<Hyperlink href="https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Object/hasOwn">"MDN web docs"</Hyperlink>" for more details."
                }),
            )
        } else {
            Some(diag)
        }
    }
}

/// Chekcks if the `Object.prototype` builtins called directly.
fn is_prototype_builtins(token_text: &str) -> bool {
return matches!(token_text, "hasOwnProperty" | "isPrototypeOf" | "propertyIsEnumerable")
    disallowed_methods.contains(&token_text)
}
