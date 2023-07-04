use rome_analyze::{context::RuleContext, declare_rule, ActionCategory, Ast, Rule, RuleDiagnostic};
use rome_console::markup;
use rome_diagnostics::Applicability;
use rome_js_factory::make;
use rome_js_syntax::{
    AnyJsExpression, AnyJsMemberExpression, AnyJsName, JsLogicalExpression, JsLogicalOperator,
    OperatorPrecedence, T,
};
use rome_rowan::{AstNode, AstNodeExt, BatchMutationExt, SyntaxResult};
use std::cmp::Ordering;
use std::collections::VecDeque;
use std::iter;

use crate::JsRuleAction;

declare_rule! {
    /// Enforce using concise optional chain instead of chained logical expressions.
    ///
    /// TypeScript 3.7 added support for the optional chain operator.
    /// This operator allows you to safely access properties and methods on objects when they are potentially `null` or `undefined`.
    /// The optional chain operator only chains when the property value is `null` or `undefined`.
    /// It is much safer than relying upon logical operator chaining; which chains on any truthy value.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```js,expect_diagnostic
    /// foo && foo.bar && foo.bar.baz && foo.bar.baz.buzz
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// foo.bar && foo.bar.baz.buzz
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// foo !== undefined && foo.bar != undefined && foo.bar.baz !== null && foo.bar.baz.buzz
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// ((foo || {}).bar || {}).baz;
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// (await (foo1 || {}).foo2 || {}).foo3;
    /// ```
    ///
    /// ```ts,expect_diagnostic
    /// (((typeof x) as string) || {}).bar;
    /// ```
    ///
    /// ### Valid
    ///
    /// ```js
    /// foo && bar;
    ///```
    /// ```js
    /// foo || {};
    ///```
    ///
    /// ```js
    /// (foo = 2 || {}).bar;
    ///```
    ///
    /// ```js
    /// foo || foo.bar;
    ///```
    ///
    /// ```js
    /// foo["some long"] && foo["some long string"].baz
    ///```
    ///
    pub(crate) UseOptionalChain {
        version: "0.10.0",
        name: "useOptionalChain",
        recommended: true,
    }
}

pub(crate) enum UseOptionalChainState {
    LogicalAnd(VecDeque<AnyJsExpression>),
    LogicalOrLike(LogicalOrLikeChain),
}

impl Rule for UseOptionalChain {
    type Query = Ast<JsLogicalExpression>;
    type State = UseOptionalChainState;
    type Signals = Option<Self::State>;
    type Options = ();

    fn run(ctx: &RuleContext<Self>) -> Option<Self::State> {
        let logical = ctx.query();
        let operator = logical.operator().ok()?;
        match operator {
            JsLogicalOperator::LogicalAnd => {
                let head = logical.right().ok()?;
                let chain = LogicalAndChain::from_expression(head).ok()?;
                if chain.is_inside_another_chain().ok()? {
                    return None;
                }
                let optional_chain_expression_nodes = chain.optional_chain_expression_nodes()?;
                Some(UseOptionalChainState::LogicalAnd(
                    optional_chain_expression_nodes,
                ))
            }
            JsLogicalOperator::NullishCoalescing | JsLogicalOperator::LogicalOr => {
                let chain = LogicalOrLikeChain::from_expression(logical)?;

                if chain.is_inside_another_chain() {
                    return None;
                }
                Some(UseOptionalChainState::LogicalOrLike(chain))
            }
        }
    }

    fn diagnostic(ctx: &RuleContext<Self>, state: &Self::State) -> Option<RuleDiagnostic> {
        let range = match state {
            UseOptionalChainState::LogicalAnd(_) => ctx.query().range(),
            UseOptionalChainState::LogicalOrLike(state) => state.member.range(),
        };
        Some(RuleDiagnostic::new(
            rule_category!(),
            range,
            markup! {
                "Change to an optional chain."
            }
            .to_owned(),
        ))
    }

    fn action(ctx: &RuleContext<Self>, state: &Self::State) -> Option<JsRuleAction> {
        match state {
            UseOptionalChainState::LogicalAnd(optional_chain_expression_nodes) => {
                let mut prev_expression = None;
                for expression in optional_chain_expression_nodes {
                    let next_expression = prev_expression
                        .take()
                        .and_then(|(prev_expression, new_expression)| {
                            expression
                                .clone()
                                .replace_node(prev_expression, new_expression)
                        })
                        .unwrap_or_else(|| expression.clone());
                    let next_expression = match next_expression {
                        AnyJsExpression::JsCallExpression(call_expression) => {
                            let mut call_expression_builder = make::js_call_expression(
                                call_expression.callee().ok()?,
                                call_expression.arguments().ok()?,
                            )
                            .with_optional_chain_token(make::token(T![?.]));
                            if let Some(type_arguments) = call_expression.type_arguments() {
                                call_expression_builder =
                                    call_expression_builder.with_type_arguments(type_arguments);
                            }
                            let call_expression = call_expression_builder.build();
                            AnyJsExpression::from(call_expression)
                        }
                        AnyJsExpression::JsStaticMemberExpression(member_expression) => {
                            let operator_token = member_expression.operator_token().ok()?;
                            AnyJsExpression::from(make::js_static_member_expression(
                                member_expression.object().ok()?,
                                make::token(T![?.])
                                    .with_leading_trivia_pieces(
                                        operator_token.leading_trivia().pieces(),
                                    )
                                    .with_trailing_trivia_pieces(
                                        operator_token.trailing_trivia().pieces(),
                                    ),
                                member_expression.member().ok()?,
                            ))
                        }
                        AnyJsExpression::JsComputedMemberExpression(member_expression) => {
                            let operator_token = match member_expression.optional_chain_token() {
                                Some(token) => make::token(T![?.])
                                    .with_leading_trivia_pieces(token.leading_trivia().pieces())
                                    .with_trailing_trivia_pieces(token.trailing_trivia().pieces()),
                                None => make::token(T![?.]),
                            };
                            AnyJsExpression::from(
                                make::js_computed_member_expression(
                                    member_expression.object().ok()?,
                                    member_expression.l_brack_token().ok()?,
                                    member_expression.member().ok()?,
                                    member_expression.r_brack_token().ok()?,
                                )
                                .with_optional_chain_token(operator_token)
                                .build(),
                            )
                        }
                        _ => return None,
                    };
                    prev_expression = Some((expression.clone(), next_expression));
                }
                let (prev_expression, new_expression) = prev_expression?;
                let logical = ctx.query();
                let next_right = logical
                    .right()
                    .ok()?
                    .replace_node(prev_expression, new_expression.clone())
                    .unwrap_or(new_expression);

                let mut mutation = ctx.root().begin();
                mutation.replace_node(AnyJsExpression::from(logical.clone()), next_right);
                Some(JsRuleAction {
                    category: ActionCategory::QuickFix,
                    applicability: Applicability::MaybeIncorrect,
                    message: markup! { "Change to an optional chain." }.to_owned(),
                    mutation,
                })
            }
            UseOptionalChainState::LogicalOrLike(chain) => {
                let chain = chain.optional_chain_expression_nodes();
                let mut prev_chain: Option<(AnyJsMemberExpression, AnyJsMemberExpression)> = None;
                for (mut left, member) in chain {
                    if let Some((prev_member, next_member)) = prev_chain.take() {
                        left = left
                            .replace_node(prev_member, next_member.clone())
                            .unwrap_or_else(|| next_member.into());
                    }
                    left = trim_trailing_space(left)?;
                    let need_parenthesis =
                        left.precedence().ok()? < OperatorPrecedence::LeftHandSide;
                    if need_parenthesis {
                        left = make::js_parenthesized_expression(
                            make::token(T!['(']),
                            left,
                            make::token(T![')']),
                        )
                        .into();
                    }
                    let next_member = match member.clone() {
                        AnyJsMemberExpression::JsStaticMemberExpression(expression) => {
                            let static_member_expression = make::js_static_member_expression(
                                left,
                                make::token(T![?.]),
                                expression.member().ok()?,
                            );
                            AnyJsMemberExpression::from(static_member_expression)
                        }
                        AnyJsMemberExpression::JsComputedMemberExpression(expression) => {
                            let computed_member_expression = make::js_computed_member_expression(
                                left,
                                expression.l_brack_token().ok()?,
                                expression.member().ok()?,
                                expression.r_brack_token().ok()?,
                            )
                            .with_optional_chain_token(make::token(T![?.]))
                            .build();
                            computed_member_expression.into()
                        }
                    };
                    prev_chain = Some((member, next_member));
                }
                let (prev_member, new_member) = prev_chain?;
                let mut mutation = ctx.root().begin();
                mutation.replace_node(prev_member, new_member);
                Some(JsRuleAction {
                    category: ActionCategory::QuickFix,
                    applicability: Applicability::MaybeIncorrect,
                    message: markup! { "Change to an optional chain." }.to_owned(),
                    mutation,
                })
            }
        }
    }
}

/// Normalize optional chain like.
/// E.g. `foo != null` is normalized to `foo`
fn normalized_optional_chain_like(expression: AnyJsExpression) -> SyntaxResult<AnyJsExpression> {
    if let AnyJsExpression::JsBinaryExpression(expression) = &expression {
        if expression.is_optional_chain_like()? {
            return expression.left();
        }
    }
    Ok(expression)
}

/// `LogicalAndChainOrdering` is the result of a comparison between two logical and chain.
enum LogicalAndChainOrdering {
    /// An ordering where a chain is a sub-chain of another.
    /// ```js
    /// (foo && foo.bar) /* is sub-chain of */ (foo && foo.bar && foo.bar.baz)
    /// ```
    SubChain,
    /// An ordering where a chain is equal to another.
    /// ```js
    /// (foo && foo.bar) /* is equal */ (foo && foo.bar)
    /// ```
    Equal,
    /// An ordering where a chain is different to another.
    /// ```js
    /// (foo && foo.bar) /* is different */ (bar && bar.bar && bar.bar.baz)
    /// ```
    Different,
}

/// `LogicalAndChain` handles cases with `JsLogicalExpression` which has `JsLogicalOperator::LogicalAnd` operator:
/// ```js
/// foo && foo.bar && foo.bar.baz && foo.bar.baz.buzz;
///
/// foo.bar && foo.bar.baz && foo.bar.baz.buzz;
///
/// foo !== undefined && foo.bar;
/// ```
/// The main idea of the `LogicalAndChain`:
/// 1. Check that the current chain isn't in another `LogicalAndChain`. We need to find the topmost logical expression which will be the head of the first current chain.
/// 2. Go down thought logical expressions and collect other chains and compare them with the current one.
/// 3. If a chain is a sub-chain of the current chain, we assign that sub-chain to new current one. Difference between current chain and sub-chain is a tail.
/// 4. Save the first tail `JsAnyExpression` to the buffer.
/// 5. Transform every `JsAnyExpression` from the buffer to optional expression.
///
/// E.g. `foo && foo.bar.baz && foo.bar.baz.zoo;`.
/// The logical expression `foo && foo.bar.baz` isn't the topmost. We skip it.
/// `foo && foo.bar.baz && foo.bar.baz.zoo;` is the topmost and it'll be a start point.
/// We start collecting a chain. We collect `JsAnyExpression` but for clarity let's use string identifiers.
/// `foo.bar.baz.zoo;` -> `[foo, bar, baz, zoo]`
/// Next step we take a next chain and also collect it.
/// `foo.bar.baz` -> `[foo, bar, baz]`
/// By comparing them we understand that one is a sub-chain of the other. `[foo, bar, baz]` is new current chain. `[zoo]` is a tail.
/// We save `zoo` expression to the buffer.
/// Next step we take a next chain and also collect it.
/// `foo` -> `[foo]`
/// By comparing them we understand that one is a sub-chain of the other. `[foo]` is new current chain. `[bar, baz]` is a tail.
/// We save `bar` expression to the buffer.
/// Iterate buffer `[bar, zoo]` we need to make every `JsAnyExpression` optional: `foo?.bar.baz?.zoo;`
///
#[derive(Debug)]
pub(crate) struct LogicalAndChain {
    head: AnyJsExpression,
    /// The buffer of `JsAnyExpression` which need to make optional chain.
    buf: VecDeque<AnyJsExpression>,
}

impl LogicalAndChain {
    fn from_expression(head: AnyJsExpression) -> SyntaxResult<LogicalAndChain> {
        /// Iterate over `JsAnyExpression` and collect every expression which is a part of the chain:
        /// ```js
        /// foo.bar[baz];
        /// ```
        /// `[JsReferenceIdentifier, JsStaticMemberExpression, JsComputedMemberExpression]`
        fn collect_chain(expression: AnyJsExpression) -> SyntaxResult<VecDeque<AnyJsExpression>> {
            let mut buf = VecDeque::new();
            let mut current_expression = Some(expression);
            while let Some(expression) = current_expression.take() {
                current_expression = match &expression {
                    AnyJsExpression::JsStaticMemberExpression(member_expression) => {
                        let object = member_expression.object()?;
                        buf.push_front(expression);
                        Some(object)
                    }
                    AnyJsExpression::JsComputedMemberExpression(member_expression) => {
                        let object = member_expression.object()?;
                        buf.push_front(expression);
                        Some(object)
                    }
                    AnyJsExpression::JsCallExpression(call_expression) => {
                        let callee = call_expression.callee()?;
                        buf.push_front(expression);
                        Some(callee)
                    }
                    AnyJsExpression::JsIdentifierExpression(_) => {
                        buf.push_front(expression);
                        return Ok(buf);
                    }
                    _ => return Ok(buf),
                };
            }
            Ok(buf)
        }
        let buf = collect_chain(head.clone())?;
        Ok(LogicalAndChain { head, buf })
    }

    /// This function checks if `LogicalAndChain` is inside another parent `LogicalAndChain`
    /// and the chain is sub-chain of parent chain.
    fn is_inside_another_chain(&self) -> SyntaxResult<bool> {
        // Because head of the chain is right expression of logical expression we need to take a parent and a grand-parent.
        // E.g. `foo && foo.bar && foo.bar.baz`
        // The head of the sub-chain is `foo.bar`.
        // The parent of the head is logical expression `foo && foo.bar`
        // The grand-parent of the head is logical expression `foo && foo.bar && foo.bar.baz`
        if let Some(parent) = self.head.parent::<JsLogicalExpression>() {
            if let Some(grand_parent) = parent.parent::<JsLogicalExpression>() {
                let grand_parent_operator = grand_parent.operator()?;
                if !matches!(grand_parent_operator, JsLogicalOperator::LogicalAnd) {
                    return Ok(false);
                }
                let grand_parent_logical_left = grand_parent.left()?;
                // Here we check that we came from the left side of the logical expression.
                // Because only the left-hand parts can be sub-chains.
                if grand_parent_logical_left.as_js_logical_expression() == Some(&parent) {
                    let grand_parent_right_chain = LogicalAndChain::from_expression(
                        normalized_optional_chain_like(grand_parent.right()?)?,
                    )?;
                    let result = grand_parent_right_chain.cmp_chain(self)?;
                    return match result {
                        LogicalAndChainOrdering::SubChain | LogicalAndChainOrdering::Equal => {
                            Ok(true)
                        }
                        LogicalAndChainOrdering::Different => Ok(false),
                    };
                }
            }
        }
        Ok(false)
    }

    /// This function compares two `LogicalAndChain` and returns `LogicalAndChainOrdering`
    /// by comparing their `token_text_trimmed` for every `JsAnyExpression` node.
    fn cmp_chain(&self, other: &LogicalAndChain) -> SyntaxResult<LogicalAndChainOrdering> {
        let chain_ordering = match self.buf.len().cmp(&other.buf.len()) {
            Ordering::Less => return Ok(LogicalAndChainOrdering::Different),
            Ordering::Equal => LogicalAndChainOrdering::Equal,
            Ordering::Greater => LogicalAndChainOrdering::SubChain,
        };
        for (main_expression, branch_expression) in self.buf.iter().zip(&other.buf) {
            let (main_expression, branch_expression) = match (&main_expression, &branch_expression)
            {
                (
                    AnyJsExpression::JsCallExpression(main_expression),
                    AnyJsExpression::JsCallExpression(branch_expression),
                ) => (main_expression.callee()?, branch_expression.callee()?),
                _ => (main_expression.clone(), branch_expression.clone()),
            };
            let (main_value_token, branch_value_token) = match (main_expression, branch_expression)
            {
                (
                    AnyJsExpression::JsComputedMemberExpression(main_expression),
                    AnyJsExpression::JsComputedMemberExpression(branch_expression),
                ) => match (main_expression.member()?, branch_expression.member()?) {
                    (
                        AnyJsExpression::JsIdentifierExpression(main_identifier),
                        AnyJsExpression::JsIdentifierExpression(branch_identifier),
                    ) => (
                        main_identifier.name()?.value_token()?,
                        branch_identifier.name()?.value_token()?,
                    ),
                    (
                        AnyJsExpression::AnyJsLiteralExpression(main_expression),
                        AnyJsExpression::AnyJsLiteralExpression(branch_expression),
                    ) => (
                        main_expression.value_token()?,
                        branch_expression.value_token()?,
                    ),
                    _ => return Ok(LogicalAndChainOrdering::Different),
                },
                (
                    AnyJsExpression::JsStaticMemberExpression(main_expression),
                    AnyJsExpression::JsStaticMemberExpression(branch_expression),
                ) => match (main_expression.member()?, branch_expression.member()?) {
                    (AnyJsName::JsName(main_name), AnyJsName::JsName(branch_name)) => {
                        (main_name.value_token()?, branch_name.value_token()?)
                    }
                    (
                        AnyJsName::JsPrivateName(main_name),
                        AnyJsName::JsPrivateName(branch_name),
                    ) => (main_name.value_token()?, branch_name.value_token()?),
                    _ => return Ok(LogicalAndChainOrdering::Different),
                },
                (
                    AnyJsExpression::JsIdentifierExpression(main_expression),
                    AnyJsExpression::JsIdentifierExpression(branch_expression),
                ) => (
                    main_expression.name()?.value_token()?,
                    branch_expression.name()?.value_token()?,
                ),
                _ => return Ok(LogicalAndChainOrdering::Different),
            };
            if main_value_token.token_text_trimmed() != branch_value_token.token_text_trimmed() {
                return Ok(LogicalAndChainOrdering::Different);
            }
        }
        Ok(chain_ordering)
    }

    /// This function returns a list of `JsAnyExpression` which we need to transform into an option chain expression.
    fn optional_chain_expression_nodes(mut self) -> Option<VecDeque<AnyJsExpression>> {
        let mut optional_chain_expression_nodes = VecDeque::with_capacity(self.buf.len());
        // Take a head of a next sub-chain
        // E.g. `foo && foo.bar && foo.bar.baz`
        // The head is `foo.bar.baz` expression.
        // The parent of the head is a logical expression `foo && foo.bar && foo.bar.baz`.
        // The next chain head is a left part of the logical expression `foo && foo.bar`
        let mut next_chain_head = self.head.parent::<JsLogicalExpression>()?.left().ok();
        while let Some(expression) = next_chain_head.take() {
            let expression = match expression {
                // Extract a left `JsAnyExpression` from `JsBinaryExpression` if it's optional chain like
                // ```js
                // (foo === undefined) && foo.bar;
                // ```
                // is roughly equivalent to
                // ```js
                // foo && foo.bar;
                // ```
                AnyJsExpression::JsBinaryExpression(expression) => expression
                    .is_optional_chain_like()
                    .ok()?
                    .then_some(expression.left().ok()?)?,
                expression => expression,
            };
            let head = match expression {
                AnyJsExpression::JsLogicalExpression(logical) => {
                    if matches!(logical.operator().ok()?, JsLogicalOperator::LogicalAnd) {
                        // Here we move our sub-chain head over the chains of logical expression
                        next_chain_head = logical.left().ok();
                        logical.right().ok()?
                    } else {
                        return None;
                    }
                }
                AnyJsExpression::JsIdentifierExpression(_)
                | AnyJsExpression::JsStaticMemberExpression(_)
                | AnyJsExpression::JsComputedMemberExpression(_)
                | AnyJsExpression::JsCallExpression(_) => expression,
                _ => return None,
            };
            let branch =
                LogicalAndChain::from_expression(normalized_optional_chain_like(head).ok()?)
                    .ok()?;
            match self.cmp_chain(&branch).ok()? {
                LogicalAndChainOrdering::SubChain => {
                    // Here we reduce our main `JsAnyExpression` buffer by splitting the main buffer.
                    // Let's say that we have two buffers:
                    // The main is `[foo, bar, baz]` and a branch is `[foo]`
                    // After splitting the main buffer will be `[foo]` and the tail will be `[bar, baz]`.
                    // It means that we need to transform `bar` (first tail expression) into the optional one.
                    let mut tail = self.buf.split_off(branch.buf.len());
                    if let Some(part) = tail.pop_front() {
                        optional_chain_expression_nodes.push_front(part)
                    };
                }
                LogicalAndChainOrdering::Equal => continue,
                LogicalAndChainOrdering::Different => return None,
            }
        }
        if optional_chain_expression_nodes.is_empty() {
            return None;
        }
        Some(optional_chain_expression_nodes)
    }
}

/// `LogicalOrLikeChain` handles cases with `JsLogicalExpression` which has `JsLogicalOperator::NullishCoalescing` or `JsLogicalOperator::LogicalOr` operator:
/// ```js
/// (foo || {}).bar;
/// (foo ?? {}).bar;
/// ((foo ?? {}).bar || {}).baz;
/// ```
/// The main idea of the `LogicalOrLikeChain`:
/// 1. Check that the current member expressions isn't in another `LogicalOrLikeChain`. We need to find the topmost member expression.
/// 2. Go down thought logical expressions and collect left and member expressions to buffer.
/// 3. Transform every left `JsAnyExpression` and member `JsAnyMemberExpression` expressions into optional `JsAnyMemberExpression`.
///
/// E.g. `((foo ?? {}).bar || {}).baz;`.
/// The member expression `(foo ?? {}).bar` isn't the topmost. We skip it.
/// `((foo ?? {}).bar || {}).baz;` is the topmost and it'll be a start point.
/// We start collecting pairs of a left and member expressions to buffer.
/// First expression is `((foo ?? {}).bar || {}).baz;`:
/// Buffer is `[((foo ?? {}).bar, ((foo ?? {}).bar || {}).baz;)]`
/// Next expressions is `((foo ?? {}).bar || {}).baz;`:
/// Buffer is `[(foo, (foo ?? {}).bar), ((foo ?? {}).bar, ((foo ?? {}).bar || {}).baz;)]`
/// Iterate buffer, take member expressions and replace object with left parts and make the expression optional chain:
/// `foo?.bar?.baz;`
///
#[derive(Debug)]
pub(crate) struct LogicalOrLikeChain {
    member: AnyJsMemberExpression,
}

impl LogicalOrLikeChain {
    /// Create a `LogicalOrLikeChain` if `JsLogicalExpression` is optional chain like and the `JsLogicalExpression` is inside member expression.
    /// ```js
    /// (foo || {}).bar;
    /// ```
    fn from_expression(logical: &JsLogicalExpression) -> Option<LogicalOrLikeChain> {
        let is_right_empty_object = logical
            .right()
            .ok()?
            // Handle case when a right expression is inside parentheses
            // E.g. (foo || (({}))).bar;
            .omit_parentheses()
            .as_js_object_expression()?
            .is_empty();
        if !is_right_empty_object {
            return None;
        }
        let member =
            LogicalOrLikeChain::get_chain_parent_member(AnyJsExpression::from(logical.clone()))?;
        Some(LogicalOrLikeChain { member })
    }

    /// This function checks if `LogicalOrLikeChain` is inside another parent `LogicalOrLikeChain`.
    /// E.g.
    /// `(foo ?? {}).bar` is inside `((foo ?? {}).bar || {}).baz;`
    fn is_inside_another_chain(&self) -> bool {
        LogicalOrLikeChain::get_chain_parent(self.member.clone()).map_or(false, |parent| {
            parent
                .as_js_logical_expression()
                .filter(|parent_expression| {
                    matches!(
                        parent_expression.operator(),
                        Ok(JsLogicalOperator::NullishCoalescing | JsLogicalOperator::LogicalOr)
                    )
                })
                .and_then(LogicalOrLikeChain::from_expression)
                .is_some()
        })
    }

    /// This function returns a list of pairs `(JsAnyExpression, JsAnyMemberExpression)` which we need to transform into an option chain expression.
    fn optional_chain_expression_nodes(
        &self,
    ) -> VecDeque<(AnyJsExpression, AnyJsMemberExpression)> {
        let mut chain = VecDeque::new();
        // Start from the topmost member expression
        let mut next_member_chain = Some(self.member.clone());
        while let Some(member) = next_member_chain.take() {
            let object = match member.object() {
                Ok(object) => object,
                _ => return chain,
            };
            // Handle case when a object expression is inside parentheses
            // E.g. (((foo || {}))).bar;
            let object = object.omit_parentheses();
            if let AnyJsExpression::JsLogicalExpression(logical) = object {
                let is_valid_operator = logical.operator().map_or(false, |operator| {
                    matches!(
                        operator,
                        JsLogicalOperator::NullishCoalescing | JsLogicalOperator::LogicalOr
                    )
                });
                if !is_valid_operator {
                    return chain;
                }
                let is_right_empty_object = logical
                    .right()
                    .ok()
                    .and_then(|right| {
                        right
                            // Handle case when a right expression is inside parentheses
                            // E.g. (foo || (({}))).bar;
                            .omit_parentheses()
                            .as_js_object_expression()
                            .map(|object| object.is_empty())
                    })
                    .unwrap_or(false);
                if !is_right_empty_object {
                    return chain;
                }
                let left = match logical.left() {
                    Ok(left) => left,
                    Err(_) => return chain,
                };
                // Set next member expression from the left part
                // Find next member expression
                // E.g. `((foo || {}).baz() || {}).bar`
                // If current member chain is `bar` the next member chain is baz.
                // Need to downward traversal to find first `JsAnyExpression` which we can't include in chain
                next_member_chain = LogicalOrLikeChain::get_member(left.clone());
                chain.push_front((left, member))
            }
        }
        chain
    }

    /// Traversal by parent to find the parent member of a chain.
    fn get_chain_parent_member(expression: AnyJsExpression) -> Option<AnyJsMemberExpression> {
        iter::successors(expression.parent::<AnyJsExpression>(), |expression| {
            if matches!(expression, AnyJsExpression::JsParenthesizedExpression(_)) {
                expression.parent::<AnyJsExpression>()
            } else {
                None
            }
        })
        .last()
        .and_then(|parent| {
            let member = match parent {
                AnyJsExpression::JsComputedMemberExpression(expression) => {
                    AnyJsMemberExpression::from(expression)
                }
                AnyJsExpression::JsStaticMemberExpression(expression) => {
                    AnyJsMemberExpression::from(expression)
                }
                _ => return None,
            };
            Some(member)
        })
    }

    /// Traversal by parent to find the parent of a chain.
    /// This function is opposite to the `get_member` function.
    fn get_chain_parent(expression: AnyJsMemberExpression) -> Option<AnyJsExpression> {
        iter::successors(expression.parent::<AnyJsExpression>(), |expression| {
            if matches!(
                expression,
                AnyJsExpression::JsParenthesizedExpression(_)
                    | AnyJsExpression::JsAwaitExpression(_)
                    | AnyJsExpression::JsCallExpression(_)
                    | AnyJsExpression::JsNewExpression(_)
                    | AnyJsExpression::TsAsExpression(_)
                    | AnyJsExpression::TsSatisfiesExpression(_)
                    | AnyJsExpression::TsNonNullAssertionExpression(_)
                    | AnyJsExpression::TsTypeAssertionExpression(_)
            ) {
                expression.parent::<AnyJsExpression>()
            } else {
                None
            }
        })
        .last()
    }

    /// Downward traversal to find the member.
    /// E.g. `((foo || {}).baz() || {}).bar`
    /// If current member chain is `bar` the next member chain is baz.
    /// Need to downward traversal to find first `JsAnyExpression` which we can't include in chain.
    fn get_member(expression: AnyJsExpression) -> Option<AnyJsMemberExpression> {
        let expression = iter::successors(Some(expression), |expression| match expression {
            AnyJsExpression::JsParenthesizedExpression(expression) => expression.expression().ok(),
            AnyJsExpression::JsAwaitExpression(expression) => expression.argument().ok(),
            AnyJsExpression::JsCallExpression(expression) => expression.callee().ok(),
            AnyJsExpression::JsNewExpression(expression) => expression.callee().ok(),
            AnyJsExpression::TsAsExpression(expression) => expression.expression().ok(),
            AnyJsExpression::TsSatisfiesExpression(expression) => expression.expression().ok(),
            AnyJsExpression::TsNonNullAssertionExpression(expression) => {
                expression.expression().ok()
            }
            AnyJsExpression::TsTypeAssertionExpression(expression) => expression.expression().ok(),
            _ => None,
        })
        .last()?;
        let expression = match expression {
            AnyJsExpression::JsComputedMemberExpression(expression) => {
                AnyJsMemberExpression::from(expression)
            }
            AnyJsExpression::JsStaticMemberExpression(expression) => {
                AnyJsMemberExpression::from(expression)
            }
            _ => return None,
        };
        Some(expression)
    }
}

fn trim_trailing_space(node: AnyJsExpression) -> Option<AnyJsExpression> {
    let Some(last_token_of_left_syntax) = node.syntax().last_token() else { return Some(node) };
    let next_token_of_left_syntax =
        last_token_of_left_syntax.with_trailing_trivia(std::iter::empty());
    node.replace_token_discard_trivia(last_token_of_left_syntax, next_token_of_left_syntax)
}
