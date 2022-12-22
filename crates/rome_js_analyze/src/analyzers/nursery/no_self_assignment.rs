use rome_analyze::{context::RuleContext, declare_rule, Ast, Rule, RuleDiagnostic};
use rome_console::markup;
use rome_js_syntax::{
    AnyJsArrayAssignmentPatternElement, AnyJsArrayElement, AnyJsAssignment, AnyJsAssignmentPattern,
    AnyJsExpression, AnyJsName, AnyJsObjectAssignmentPatternMember, AnyJsObjectMember,
    JsAssignmentExpression, JsAssignmentOperator, JsIdentifierAssignment, JsLanguage, JsName,
    JsPrivateName, JsReferenceIdentifier, JsStaticMemberAssignment, JsStaticMemberExpression,
    JsSyntaxToken,
};
use rome_rowan::{
    AstNode, AstSeparatedList, AstSeparatedListNodesIterator, SyntaxError, TextRange,
};
use std::collections::VecDeque;
use std::iter::FusedIterator;

declare_rule! {
    /// Put your description here
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```js,expect_diagnostic
    /// var a = 1;
    /// a = 2;
    /// ```
    ///
    /// ## Valid
    ///
    /// ```js
    /// var a = 1;
    /// ```
    ///
    pub(crate) NoSelfAssignment {
        version: "12.0.0",
        name: "noSelfAssignment",
        recommended: false,
    }
}

/// A convenient iterator that continues to return the nested [JsStaticMemberExpression]
#[derive(Debug, Clone)]
struct JsStaticMemberAssignmentIterator {
    source: JsStaticMemberAssignment,
    current_member_expression: Option<JsStaticMemberExpression>,
    drained: bool,
}

impl JsStaticMemberAssignmentIterator {
    fn new(source: JsStaticMemberAssignment) -> Self {
        Self {
            source,
            current_member_expression: None,
            drained: false,
        }
    }
}

impl Iterator for JsStaticMemberAssignmentIterator {
    type Item = (AnyJsName, Option<JsReferenceIdentifier>);

    fn next(&mut self) -> Option<Self::Item> {
        if self.drained {
            return None;
        }

        let (name, object) =
            if let Some(current_member_expression) = self.current_member_expression.as_ref() {
                (
                    current_member_expression.member().ok(),
                    current_member_expression.object().ok(),
                )
            } else {
                (self.source.member().ok(), self.source.object().ok())
            };

        let name = name?;
        let reference = match object {
            Some(AnyJsExpression::JsStaticMemberExpression(expression)) => {
                self.current_member_expression = Some(expression);
                None
            }
            Some(AnyJsExpression::JsIdentifierExpression(identifier)) => {
                // the left side of the static member expression is an identifier, which means that we can't
                // go any further and we should mark the iterator and drained
                self.drained = true;
                Some(identifier.name().ok()?)
            }
            _ => return None,
        };
        Some((name, reference))
    }
}

impl FusedIterator for JsStaticMemberAssignmentIterator {}

/// A convenient iterator that continues to return the nested [JsStaticMemberExpression]
#[derive(Debug, Clone)]
struct JsStaticMemberExpressionIterator {
    source: JsStaticMemberExpression,
    current_member_expression: Option<JsStaticMemberExpression>,
    drained: bool,
}

impl JsStaticMemberExpressionIterator {
    fn new(source: JsStaticMemberExpression) -> Self {
        Self {
            source,
            current_member_expression: None,
            drained: false,
        }
    }
}

impl Iterator for JsStaticMemberExpressionIterator {
    type Item = (AnyJsName, Option<JsReferenceIdentifier>);

    fn next(&mut self) -> Option<Self::Item> {
        if self.drained {
            return None;
        }

        let current_expression =
            if let Some(current_member_expression) = self.current_member_expression.as_ref() {
                current_member_expression
            } else {
                &self.source
            };

        let name = current_expression.member().ok()?;
        let object = current_expression.object().ok();
        let reference = match object {
            Some(AnyJsExpression::JsStaticMemberExpression(expression)) => {
                self.current_member_expression = Some(expression);
                None
            }
            Some(AnyJsExpression::JsIdentifierExpression(identifier)) => {
                // the left side of the static member expression is an identifier, which means that we can't
                // go any further and we should mark the iterator and drained
                self.drained = true;
                Some(identifier.name().ok()?)
            }
            _ => return None,
        };
        Some((name, reference))
    }
}

impl FusedIterator for JsStaticMemberExpressionIterator {}

/// Convenient type to map assignments that have similar arms
#[derive(Debug, Clone)]
enum AnyAssignmentLike {
    /// No assignments
    None,
    /// To track identifiers that will be compared and check if they are the same
    Identifiers(IdentifiersLike),
    /// To track assignments like
    /// ```js
    /// [a] = [a]
    /// ```
    Arrays {
        left: AstSeparatedListNodesIterator<JsLanguage, AnyJsArrayAssignmentPatternElement>,
        right: AstSeparatedListNodesIterator<JsLanguage, AnyJsArrayElement>,
    },
    /// To track assignments like
    /// ```js
    /// {a} = {a}
    /// ```
    Object {
        left: AstSeparatedListNodesIterator<JsLanguage, AnyJsObjectAssignmentPatternMember>,
        right: AstSeparatedListNodesIterator<JsLanguage, AnyJsObjectMember>,
    },
    /// To track static expressions
    /// ```js
    /// a.b = a.b
    /// ```
    StaticExpression {
        left: JsStaticMemberAssignmentIterator,
        right: JsStaticMemberExpressionIterator,
    },
}

impl AnyAssignmentLike {
    const fn has_sub_structures(&self) -> bool {
        matches!(
            self,
            AnyAssignmentLike::Arrays { .. } | AnyAssignmentLike::Object { .. }
        )
    }
}

impl TryFrom<(AnyJsAssignmentPattern, AnyJsExpression)> for AnyAssignmentLike {
    type Error = SyntaxError;

    fn try_from(
        (left, right): (AnyJsAssignmentPattern, AnyJsExpression),
    ) -> Result<Self, Self::Error> {
        Ok(match (left, right) {
            (
                AnyJsAssignmentPattern::JsArrayAssignmentPattern(left),
                AnyJsExpression::JsArrayExpression(right),
            ) => AnyAssignmentLike::Arrays {
                left: left.elements().iter(),
                right: right.elements().iter(),
            },

            (
                AnyJsAssignmentPattern::JsObjectAssignmentPattern(left),
                AnyJsExpression::JsObjectExpression(right),
            ) => AnyAssignmentLike::Object {
                left: left.properties().iter(),
                right: right.members().iter(),
            },

            (
                AnyJsAssignmentPattern::AnyJsAssignment(AnyJsAssignment::JsIdentifierAssignment(
                    left,
                )),
                AnyJsExpression::JsIdentifierExpression(right),
            ) => AnyAssignmentLike::Identifiers(IdentifiersLike::IdentifierAndReference(
                left,
                right.name()?,
            )),
            (
                AnyJsAssignmentPattern::AnyJsAssignment(AnyJsAssignment::JsStaticMemberAssignment(
                    left,
                )),
                AnyJsExpression::JsStaticMemberExpression(right),
            ) => AnyAssignmentLike::StaticExpression {
                left: JsStaticMemberAssignmentIterator::new(left),
                right: JsStaticMemberExpressionIterator::new(right),
            },

            _ => AnyAssignmentLike::None,
        })
    }
}

/// Convenient type to loop though all the identifiers that can be found
/// inside an assignment expression.
struct SameIdentifiers {
    /// To current assignment like that is being inspected
    current_assignment_like: AnyAssignmentLike,
    /// A queue of assignments that are inspected during the traversal
    assignment_queue: VecDeque<AnyAssignmentLike>,
}

#[derive(Debug, Clone)]
pub(crate) enum IdentifiersLike {
    IdentifierAndReference(JsIdentifierAssignment, JsReferenceIdentifier),
    References(JsReferenceIdentifier, JsReferenceIdentifier),
    Name(JsName, JsName),
    PrivateName(JsPrivateName, JsPrivateName),
}

impl TryFrom<(AnyJsName, AnyJsName)> for IdentifiersLike {
    type Error = ();

    fn try_from((left, right): (AnyJsName, AnyJsName)) -> Result<Self, Self::Error> {
        match (left, right) {
            (AnyJsName::JsName(left), AnyJsName::JsName(right)) => Ok(Self::Name(left, right)),
            (AnyJsName::JsPrivateName(left), AnyJsName::JsPrivateName(right)) => {
                Ok(Self::PrivateName(left, right))
            }
            _ => Err(()),
        }
    }
}

impl IdentifiersLike {
    fn left_range(&self) -> TextRange {
        match self {
            IdentifiersLike::IdentifierAndReference(left, _) => left.range(),
            IdentifiersLike::Name(left, _) => left.range(),
            IdentifiersLike::PrivateName(left, _) => left.range(),
            IdentifiersLike::References(left, _) => left.range(),
        }
    }

    fn right_range(&self) -> TextRange {
        match self {
            IdentifiersLike::IdentifierAndReference(_, right) => right.range(),
            IdentifiersLike::Name(_, right) => right.range(),
            IdentifiersLike::PrivateName(_, right) => right.range(),
            IdentifiersLike::References(_, right) => right.range(),
        }
    }

    fn name(&self) -> Option<JsSyntaxToken> {
        match self {
            IdentifiersLike::IdentifierAndReference(_, right) => right.value_token().ok(),
            IdentifiersLike::Name(_, right) => right.value_token().ok(),
            IdentifiersLike::PrivateName(_, right) => right.value_token().ok(),
            IdentifiersLike::References(_, right) => right.value_token().ok(),
        }
    }
}

impl Iterator for SameIdentifiers {
    type Item = IdentifiersLike;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            let new_assignment_like = match &mut self.current_assignment_like {
                AnyAssignmentLike::Arrays { left, right } => {
                    if let (Some(left_element), Some(right_element)) = (left.next(), right.next()) {
                        let left_element = left_element.ok()?;
                        let right_element = right_element.ok()?;

                        match (left_element, right_element) {
                            // matches [a] = [a]
                            (
                                AnyJsArrayAssignmentPatternElement::AnyJsAssignmentPattern(left),
                                AnyJsArrayElement::AnyJsExpression(right),
                            ) => {
                                let new_pair = AnyAssignmentLike::try_from((left, right)).ok()?;
                                // In case we have nested array/object structures, we save the current
                                // pair and we restore it once this iterator is consumed
                                if new_pair.has_sub_structures() {
                                    self.assignment_queue
                                        .push_back(self.current_assignment_like.clone());
                                }
                                new_pair
                            }
                            _ => AnyAssignmentLike::None,
                        }
                    } else {
                        AnyAssignmentLike::None
                    }
                }
                AnyAssignmentLike::Object { left, right } => {
                    if let (Some(left_element), Some(right_element)) = (left.next(), right.next()) {
                        let left_element = left_element.ok()?;
                        let right_element = right_element.ok()?;

                        match (left_element, right_element) {
                            // matches {a} = {a}
                            (

                                AnyJsObjectAssignmentPatternMember::JsObjectAssignmentPatternShorthandProperty(
                                    left
                                ),
                                AnyJsObjectMember::JsShorthandPropertyObjectMember(right)
                            ) => {
                                AnyAssignmentLike::Identifiers(IdentifiersLike::IdentifierAndReference(
                                    left.identifier().ok()?,
                                    right.name().ok()?
                                ))
                            }

                            (
                                AnyJsObjectAssignmentPatternMember::JsObjectAssignmentPatternProperty(left),
                                AnyJsObjectMember::JsPropertyObjectMember(right)
                            ) => {
                                let left = left.pattern().ok()?;
                                let right = right.value().ok()?;
                                match (left, right) {
                                    // matches {a: b} = {a: b}
                                    (
                                        AnyJsAssignmentPattern::AnyJsAssignment(
                                            AnyJsAssignment::JsIdentifierAssignment(left)
                                        ),
                                        AnyJsExpression::JsIdentifierExpression(right)
                                    ) => {
                                        AnyAssignmentLike::Identifiers(IdentifiersLike::IdentifierAndReference(
                                            left,
                                            right.name().ok()?
                                        ))

                                    }
                                    // matches {a: [b]} = {a: [b]}
                                    (
                                        AnyJsAssignmentPattern::JsArrayAssignmentPattern(left),
                                        AnyJsExpression::JsArrayExpression(right)
                                    ) => {
                                        self.assignment_queue.push_back(self.current_assignment_like.clone());
                                        AnyAssignmentLike::Arrays {
                                            left: left.elements().iter(),
                                            right: right.elements().iter()
                                        }
                                    }
                                    // matches {a: {b}} = {a: {b}}
                                    (
                                        AnyJsAssignmentPattern::JsObjectAssignmentPattern(left),
                                        AnyJsExpression::JsObjectExpression(right)
                                    ) => {
                                        self.assignment_queue.push_back(self.current_assignment_like.clone());
                                        AnyAssignmentLike::Object {
                                            left: left.properties().iter(),
                                            right: right.members().iter()
                                        }
                                    }
                                    _ => AnyAssignmentLike::None
                                }
                            }
                            _ => {
                                AnyAssignmentLike::None
                            },
                        }
                    } else {
                        AnyAssignmentLike::None
                    }
                }
                AnyAssignmentLike::StaticExpression { left, right } => {
                    if let (Some(left), Some(right)) = (left.next(), right.next()) {
                        let (left_name, left_reference) = left;
                        let (right_name, right_reference) = right;
                        if let Ok(identifier_like) =
                            IdentifiersLike::try_from((left_name, right_name))
                        {
                            if with_same_identifiers(&identifier_like).is_some() {
                                if let (Some(left_reference), Some(right_reference)) =
                                    (left_reference, right_reference)
                                {
                                    if with_same_identifiers(&IdentifiersLike::References(
                                        left_reference,
                                        right_reference,
                                    ))
                                    .is_some()
                                    {
                                        AnyAssignmentLike::Identifiers(identifier_like)
                                    } else {
                                        AnyAssignmentLike::None
                                    }
                                } else {
                                    AnyAssignmentLike::None
                                }
                            } else {
                                AnyAssignmentLike::None
                            }
                        } else {
                            AnyAssignmentLike::None
                        }
                    } else {
                        AnyAssignmentLike::None
                    }
                }
                AnyAssignmentLike::None | AnyAssignmentLike::Identifiers { .. } => {
                    self.current_assignment_like.clone()
                }
            };

            // if the queue is empty, we set the current assignment to `None`,
            // so the next iteration will stop
            if self.assignment_queue.is_empty() {
                self.current_assignment_like = AnyAssignmentLike::None;
            }
            match new_assignment_like {
                // if we are here, it's plausible that we consumed the current iterator and we have to
                // resume the previous one
                AnyAssignmentLike::None => {
                    // we still have assignments like to complete, so we continue the loop
                    if let Some(pair) = self.assignment_queue.pop_front() {
                        self.current_assignment_like = pair;
                        continue;
                    // the queue is empty
                    } else {
                        return None;
                    }
                }
                AnyAssignmentLike::Identifiers(identifier_like) => {
                    return Some(identifier_like);
                }

                // we have a sub structure, which means we queue the current assignment,
                // and inspect the sub structure
                AnyAssignmentLike::StaticExpression { .. }
                | AnyAssignmentLike::Object { .. }
                | AnyAssignmentLike::Arrays { .. } => {
                    self.assignment_queue
                        .push_back(self.current_assignment_like.clone());
                    self.current_assignment_like = new_assignment_like;
                    continue;
                }
            }
        }
    }
}

impl FusedIterator for SameIdentifiers {}

/// Checks if the left identifier and the right reference have the same name
fn with_same_identifiers(identifiers_like: &IdentifiersLike) -> Option<()> {
    let (left_value, right_value) = match &identifiers_like {
        IdentifiersLike::IdentifierAndReference(left, right) => {
            let left_value = left.name_token().ok()?;
            let right_value = right.value_token().ok()?;
            (left_value, right_value)
        }
        IdentifiersLike::Name(left, right) => {
            let left_value = left.value_token().ok()?;
            let right_value = right.value_token().ok()?;
            (left_value, right_value)
        }
        IdentifiersLike::PrivateName(left, right) => {
            let left_value = left.value_token().ok()?;
            let right_value = right.value_token().ok()?;
            (left_value, right_value)
        }
        IdentifiersLike::References(left, right) => {
            let left_value = left.value_token().ok()?;
            let right_value = right.value_token().ok()?;
            (left_value, right_value)
        }
    };

    if left_value.text_trimmed() == right_value.text_trimmed() {
        return Some(());
    } else {
        None
    }
}

/// It traverses an [AnyAssignmentLike] and tracks the identifiers that have the same name
fn compare_assignment_like(
    pair_kind: AnyAssignmentLike,
    incorrect_identifiers: &mut Vec<IdentifiersLike>,
) {
    let mut same_identifiers = SameIdentifiers {
        current_assignment_like: pair_kind.clone(),
        assignment_queue: VecDeque::new(),
    };

    while let Some(identifier_like) = same_identifiers.next() {
        if let Some(_) = with_same_identifiers(&identifier_like) {
            incorrect_identifiers.push(identifier_like);
        }
    }
}

impl Rule for NoSelfAssignment {
    type Query = Ast<JsAssignmentExpression>;
    type State = IdentifiersLike;
    type Signals = Vec<Self::State>;
    type Options = ();

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let node = ctx.query();
        let left = node.left().ok();
        let right = node.right().ok();
        let operator = node.operator().ok();

        let mut state = vec![];
        if let Some(operator) = operator {
            if matches!(
                operator,
                JsAssignmentOperator::Assign
                    | JsAssignmentOperator::LogicalAndAssign
                    | JsAssignmentOperator::LogicalOrAssign
                    | JsAssignmentOperator::NullishCoalescingAssign
            ) {
                match (left, right) {
                    (Some(left), Some(right)) => {
                        if let Ok(pair) = AnyAssignmentLike::try_from((left, right)) {
                            compare_assignment_like(pair, &mut state);
                        }
                    }
                    _ => {}
                }
            }
        }
        state
    }

    fn diagnostic(_: &RuleContext<Self>, identifier_like: &Self::State) -> Option<RuleDiagnostic> {
        let name = identifier_like.name()?;
        Some(
            RuleDiagnostic::new(
                rule_category!(),
                identifier_like.right_range(),
                markup! {
                    {{name.text_trimmed()}}" is assigned to itself."
                },
            )
            .detail(
                identifier_like.left_range(),
                markup! {
                    "This is where is assigned."
                },
            ),
        )
    }
}
