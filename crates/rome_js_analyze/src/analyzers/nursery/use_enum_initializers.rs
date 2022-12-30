use crate::JsRuleAction;
use rome_analyze::context::RuleContext;
use rome_analyze::{declare_rule, ActionCategory, Ast, Rule, RuleDiagnostic};
use rome_console::markup;
use rome_diagnostics::Applicability;
use rome_js_factory::make;
use rome_js_syntax::{AnyJsExpression, AnyJsLiteralExpression, JsSyntaxKind, TsEnumMember};
use rome_rowan::{AstNode, BatchMutationExt, Direction};

declare_rule! {
    /// Require that each enum member value be explicitly initialized.
    ///
    /// TypeScript enums are a practical way to organize semantically related constant values. Members of enums that don't have explicit values are by default given sequentially increasing numbers.
    ///
    /// When the value of enum members are important, allowing implicit values for enum members can cause bugs if enum declarations are modified over time.
    ///
    /// Source: https://typescript-eslint.io/rules/prefer-enum-initializers
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```ts,expect_diagnostic
    /// enum Version {
    ///     V1,
    /// }
    /// ```
    ///
    /// ```ts,expect_diagnostic
    /// enum Status {
    ///     Open = 1,
    ///     Close,
    /// }
    /// ```
    ///
    /// ```ts,expect_diagnostic
    /// enum Color {
    ///     Red = "Red",
    ///     Green = "Green",
    ///     Blue,
    /// }
    /// ```
    ///
    /// ### Valid
    ///
    /// ```ts
    /// enum Status {
    ///     Open = 1,
    ///     Close = 2,
    /// }
    /// ```
    ///
    /// ```ts
    /// enum Color {
    ///     Red = "Red",
    ///     Green = "Green",
    ///     Blue = "Blue",
    /// }
    /// ```
    ///
    pub(crate) UseEnumInitializers {
        version: "11.0.0",
        name: "useEnumInitializers",
        recommended: true,
    }
}

impl Rule for UseEnumInitializers {
    type Query = Ast<TsEnumMember>;
    type State = ();
    type Signals = Option<Self::State>;
    type Options = ();

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let enum_member = ctx.query();
        enum_member.initializer().is_none().then_some(())
    }

    fn diagnostic(ctx: &RuleContext<Self>, _: &Self::State) -> Option<RuleDiagnostic> {
        let enum_member = ctx.query();
        Some(RuleDiagnostic::new(
            rule_category!(),
            enum_member.range(),
            markup! {
                "The "<Emphasis>"enum member"</Emphasis>" should be initialized."
            },
        ).note(
            "Allowing implicit values for enum members can cause bugs if enum declarations are modified over time."
        ))
    }

    fn action(ctx: &RuleContext<Self>, _: &Self::State) -> Option<JsRuleAction> {
        let enum_member = ctx.query();
        let prev_enum_members = enum_member
            .syntax()
            .siblings(Direction::Prev)
            .into_iter()
            .skip(1) // consume enum_member
            .filter_map(TsEnumMember::cast);
        let mut count = 0;
        let mut prev_enum_member_info = None;
        for prev_enum_member in prev_enum_members {
            count += 1;
            if let Some(initializer) = prev_enum_member.initializer() {
                prev_enum_member_info = Some((initializer, prev_enum_member.name().ok()?));
                break;
            }
        }
        let expr = if let Some((prev_initializer, prev_name)) = prev_enum_member_info {
            let expr = prev_initializer.expression().ok()?;
            let expr = expr.as_any_js_literal_expression()?;
            match expr {
                AnyJsLiteralExpression::JsNumberLiteralExpression(expr) => {
                    Some(AnyJsLiteralExpression::JsNumberLiteralExpression(
                        make::js_number_literal_expression(make::js_number_literal(
                            expr.as_number()? + f64::from(count),
                        )),
                    ))
                }
                AnyJsLiteralExpression::JsStringLiteralExpression(expr) => {
                    let prev_enum_delim_val = expr.value_token().ok()?;
                    let prev_enum_delim_val = prev_enum_delim_val.text();
                    let prev_enum_val = &prev_enum_delim_val[1..(prev_enum_delim_val.len() - 1)];
                    if prev_name.text() == prev_enum_val {
                        let enum_name = enum_member.name().ok()?.text();
                        Some(AnyJsLiteralExpression::JsStringLiteralExpression(
                            make::js_string_literal_expression(make::js_string_literal(&enum_name)),
                        ))
                    } else {
                        None
                    }
                }
                _ => None,
            }
        } else {
            Some(AnyJsLiteralExpression::JsNumberLiteralExpression(
                make::js_number_literal_expression(make::js_number_literal(count)),
            ))
        };
        if let Some(expr) = expr {
            let new_enum_member =
                enum_member
                    .to_owned()
                    .with_initializer(Some(make::js_initializer_clause(
                        make::token_decorated_with_space(JsSyntaxKind::EQ),
                        AnyJsExpression::AnyJsLiteralExpression(expr),
                    )));
            let mut mutation = ctx.root().begin();
            mutation.replace_node_discard_trivia(enum_member.to_owned(), new_enum_member);
            Some(JsRuleAction {
                category: ActionCategory::QuickFix,
                applicability: Applicability::MaybeIncorrect,
                message: markup! { "" }.to_owned(),
                mutation,
            })
        } else {
            None
        }
    }
}
