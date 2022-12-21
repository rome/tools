use rome_analyze::{context::RuleContext, declare_rule, Ast, Rule, RuleDiagnostic};
use rome_console::markup;
use rome_js_syntax::{
    AnyJsArrayAssignmentPatternElement, AnyJsArrayElement, AnyJsAssignment, AnyJsAssignmentPattern,
    AnyJsExpression, AnyJsObjectAssignmentPatternMember, AnyJsObjectMember, JsAssignmentExpression,
    JsAssignmentOperator, JsIdentifierAssignment, JsLanguage, JsReferenceIdentifier,
};
use rome_rowan::{AstNode, AstSeparatedList, AstSeparatedListNodesIterator, SyntaxError};
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

/// Convenient type to map assignments that have similar arms
#[derive(Debug, Clone)]
enum AnyAssignmentLike {
    /// No assignments
    None,
    /// To track assignments like
    /// ```js
    /// a = a
    /// ```
    Identifiers {
        left: JsIdentifierAssignment,
        right: JsReferenceIdentifier,
    },
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
            ) => AnyAssignmentLike::Identifiers {
                left,
                right: right.name()?,
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

impl Iterator for SameIdentifiers {
    type Item = (JsIdentifierAssignment, JsReferenceIdentifier);

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
                                AnyAssignmentLike::Identifiers {
                                    left: left.identifier().ok()?, right: right.name().ok()?
                                }
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
                                        AnyAssignmentLike::Identifiers {
                                            left,
                                            right: right.name().ok()?
                                        }

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
                _ => self.current_assignment_like.clone(),
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
                AnyAssignmentLike::Identifiers { left, right } => {
                    return Some((left, right));
                }
                // we have a sub structure, which means we queue the current assignment,
                // and inspect the sub structure
                AnyAssignmentLike::Object { .. } | AnyAssignmentLike::Arrays { .. } => {
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
fn with_same_identifiers(
    left: &JsIdentifierAssignment,
    right: &JsReferenceIdentifier,
) -> Option<(JsIdentifierAssignment, JsReferenceIdentifier)> {
    let left_value = left.name_token().ok()?;
    let right_value = right.value_token().ok()?;
    if left_value.text_trimmed() == right_value.text_trimmed() {
        return Some((left.clone(), right.clone()));
    }

    None
}

/// It traverses an [AnyAssignmentLike] and tracks the identifiers that have the same name
fn compare_assignment_like(
    pair_kind: AnyAssignmentLike,
    incorrect_identifiers: &mut Vec<(JsIdentifierAssignment, JsReferenceIdentifier)>,
) {
    let mut same_identifiers = SameIdentifiers {
        current_assignment_like: pair_kind.clone(),
        assignment_queue: VecDeque::new(),
    };

    while let Some((left, right)) = same_identifiers.next() {
        if let Some(pair) = with_same_identifiers(&left, &right) {
            incorrect_identifiers.push(pair);
        }
    }
}

impl Rule for NoSelfAssignment {
    type Query = Ast<JsAssignmentExpression>;
    type State = (JsIdentifierAssignment, JsReferenceIdentifier);
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

    fn diagnostic(_: &RuleContext<Self>, (left, right): &Self::State) -> Option<RuleDiagnostic> {
        let name = right.value_token().ok()?;
        Some(
            RuleDiagnostic::new(
                rule_category!(),
                right.range(),
                markup! {
                    {{name.text_trimmed()}}" is assigned to itself."
                },
            )
            .detail(
                left.range(),
                markup! {
                    "This is where is assigned."
                },
            ),
        )
    }
}
