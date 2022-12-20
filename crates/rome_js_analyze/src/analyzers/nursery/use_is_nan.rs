use rome_analyze::context::RuleContext;
use rome_analyze::{declare_rule, Ast, Rule, RuleDiagnostic};
use rome_js_syntax::{
    AnyJsExpression, JsBinaryExpression, JsCaseClause, JsSwitchStatement, TextRange,
};
use rome_rowan::{declare_node_union, AstNode};

declare_rule! {
    /// Require calls to `isNaN()` when checking for `NaN`.
    ///
    /// In JavaScript, `NaN` is a special value of the `Number` type. It’s used to represent any of the "not-a-number" values represented by the double-precision 64-bit format as specified by the IEEE Standard for Binary Floating-Point Arithmetic.
    ///
    /// Because `NaN` is unique in JavaScript by not being equal to anything, including itself, the results of comparisons to `NaN` are confusing:
    /// - `NaN` === `NaN` or `NaN` == `NaN` evaluate to false
    /// - `NaN` !== `NaN` or `NaN` != `NaN` evaluate to true
    ///
    /// Therefore, use `Number.isNaN()` or global `isNaN()`	 functions to test whether a value is `NaN`.
    ///
    /// Source: [use-isnan](https://eslint.org/docs/latest/rules/use-isnan).
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```js,expect_diagnostic
    /// 123 == NaN
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// 123 != NaN
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// switch(foo) { case (NaN): break; }
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// Number.NaN == "abc"
    /// ```
    ///
    /// ### Valid
    ///
    /// ```js
    /// if (isNaN(123) !== true) {}
    ///
    /// foo(Number.NaN / 2)
    ///
    /// switch(foo) {}
    /// ```
    ///
    pub(crate) UseIsNan {
        version: "next",
        name: "useIsNan",
        recommended: true,
    }
}

declare_node_union! {
    pub(crate) UseIsNanQuery = JsBinaryExpression | JsCaseClause | JsSwitchStatement
}

enum Message {
    BinaryExpression,
    CaseClause,
    SwitchCase,
}

pub struct RuleState {
    range: TextRange,
    message_id: Message,
}

impl Message {
    fn as_str(&self) -> &str {
        match self {
			Self::BinaryExpression => "Use the isNaN function to compare with NaN.",
			Self::CaseClause => "'case NaN' can never match. Use Number.isNaN before the switch.",
			Self::SwitchCase => "'switch(NaN)' can never match a case clause. Use Number.isNaN instead of the switch."
		}
    }
}

impl Rule for UseIsNan {
    type Query = Ast<UseIsNanQuery>;
    type State = RuleState;
    type Signals = Option<Self::State>;
    type Options = ();

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let node = ctx.query();

        match node {
            UseIsNanQuery::JsBinaryExpression(bin_expr) => {
                if bin_expr.is_comparison_operator()
                    && (has_nan(&bin_expr.left().ok()?)? || has_nan(&bin_expr.right().ok()?)?)
                {
                    return Some(RuleState {
                        message_id: Message::BinaryExpression,
                        range: bin_expr.range(),
                    });
                }
            }
            UseIsNanQuery::JsCaseClause(case_clause) => {
                let test = case_clause.test().ok()?;

                if has_nan(&test)? {
                    return Some(RuleState {
                        message_id: Message::CaseClause,
                        range: test.range(),
                    });
                }
            }
            UseIsNanQuery::JsSwitchStatement(switch_stmt) => {
                let discriminant = switch_stmt.discriminant().ok()?;

                if has_nan(&discriminant)? {
                    return Some(RuleState {
                        message_id: Message::SwitchCase,
                        range: discriminant.range(),
                    });
                }
            }
        }

        None
    }

    fn diagnostic(_: &RuleContext<Self>, state: &Self::State) -> Option<RuleDiagnostic> {
        Some(RuleDiagnostic::new(
            rule_category!(),
            state.range,
            state.message_id.as_str(),
        ))
    }
}

/// Checks whether an expression has `NaN`, `Number.NaN`, or `Number['NaN']`.
fn has_nan(expr: &AnyJsExpression) -> Option<bool> {
    Some(match expr.clone().omit_parentheses() {
        AnyJsExpression::JsIdentifierExpression(id_expr) => {
            id_expr.name().ok()?.value_token().ok()?.text_trimmed() == "NaN"
        }
        AnyJsExpression::JsStaticMemberExpression(member_expr) => {
            let is_number_object = member_expr
                .object()
                .ok()?
                .as_js_identifier_expression()?
                .name()
                .ok()?
                .value_token()
                .ok()?
                .text_trimmed()
                == "Number";
            let is_nan = member_expr
                .member()
                .ok()?
                .as_js_name()?
                .value_token()
                .ok()?
                .text_trimmed()
                == "NaN";

            is_number_object && is_nan
        }
        AnyJsExpression::JsComputedMemberExpression(member_expr) => {
            let is_number_object = member_expr
                .object()
                .ok()?
                .as_js_identifier_expression()?
                .name()
                .ok()?
                .value_token()
                .ok()?
                .text_trimmed()
                == "Number";
            let is_member_nan = member_expr.member().ok()?.is_string_constant("NaN");

            is_number_object && is_member_nan
        }
        _ => false,
    })
}
