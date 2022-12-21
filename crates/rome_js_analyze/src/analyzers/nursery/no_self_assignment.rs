use rome_analyze::{context::RuleContext, declare_rule, Ast, Rule, RuleDiagnostic};
use rome_console::markup;
use rome_js_syntax::{
    AnyJsArrayAssignmentPatternElement, AnyJsArrayElement, AnyJsAssignment, AnyJsAssignmentPattern,
    AnyJsExpression, AnyJsObjectAssignmentPatternMember, AnyJsObjectMember, JsAssignmentExpression,
    JsAssignmentOperator, JsIdentifierAssignment, JsLanguage, JsReferenceIdentifier,
};
use rome_rowan::{AstNode, AstSeparatedList, AstSeparatedListNodesIterator, SyntaxError};
use std::collections::VecDeque;

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

#[derive(Debug, Clone)]
enum LeftRightKind {
    None,
    Identifiers {
        left: JsIdentifierAssignment,
        right: JsReferenceIdentifier,
    },

    Arrays {
        left: AstSeparatedListNodesIterator<JsLanguage, AnyJsArrayAssignmentPatternElement>,
        right: AstSeparatedListNodesIterator<JsLanguage, AnyJsArrayElement>,
    },

    Object {
        left: AstSeparatedListNodesIterator<JsLanguage, AnyJsObjectAssignmentPatternMember>,
        right: AstSeparatedListNodesIterator<JsLanguage, AnyJsObjectMember>,
    },
}

impl LeftRightKind {
    const fn is_none(&self) -> bool {
        matches!(self, LeftRightKind::None)
    }

    const fn has_sub_structures(&self) -> bool {
        matches!(
            self,
            LeftRightKind::Arrays { .. } | LeftRightKind::Object { .. }
        )
    }
}

impl TryFrom<(AnyJsAssignmentPattern, AnyJsExpression)> for LeftRightKind {
    type Error = SyntaxError;

    fn try_from(
        (left, right): (AnyJsAssignmentPattern, AnyJsExpression),
    ) -> Result<Self, Self::Error> {
        Ok(match (left, right) {
            (
                AnyJsAssignmentPattern::JsArrayAssignmentPattern(left),
                AnyJsExpression::JsArrayExpression(right),
            ) => LeftRightKind::Arrays {
                left: left.elements().iter(),
                right: right.elements().iter(),
            },

            (
                AnyJsAssignmentPattern::JsObjectAssignmentPattern(left),
                AnyJsExpression::JsObjectExpression(right),
            ) => LeftRightKind::Object {
                left: left.properties().iter(),
                right: right.members().iter(),
            },

            (
                AnyJsAssignmentPattern::AnyJsAssignment(AnyJsAssignment::JsIdentifierAssignment(
                    left,
                )),
                AnyJsExpression::JsIdentifierExpression(right),
            ) => LeftRightKind::Identifiers {
                left,
                right: right.name()?,
            },

            _ => LeftRightKind::None,
        })
    }
}

struct Identifiers {
    current_pair: LeftRightKind,
    pair_queue: VecDeque<LeftRightKind>,
}

impl Iterator for Identifiers {
    type Item = (JsIdentifierAssignment, JsReferenceIdentifier);

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            let new_pair = match &mut self.current_pair {
                LeftRightKind::Arrays { left, right } => {
                    if let (Some(left_element), Some(right_element)) = (left.next(), right.next()) {
                        let left_element = left_element.ok()?;
                        let right_element = right_element.ok()?;

                        match (left_element, right_element) {
                            // matches [a] = [a]
                            (
                                AnyJsArrayAssignmentPatternElement::AnyJsAssignmentPattern(left),
                                AnyJsArrayElement::AnyJsExpression(right),
                            ) => {
                                let new_pair = LeftRightKind::try_from((left, right)).ok()?;
                                // In case we have nested array/object structures, we save the current
                                // pair and we restore it once this iterator is consumed
                                if new_pair.has_sub_structures() {
                                    self.pair_queue.push_back(self.current_pair.clone());
                                }
                                new_pair
                            }
                            _ => LeftRightKind::None,
                        }
                    } else {
                        LeftRightKind::None
                    }
                }
                LeftRightKind::Object { left, right } => {
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
                                LeftRightKind::Identifiers {
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
                                        LeftRightKind::Identifiers {
                                            left,
                                            right: right.name().ok()?
                                        }

                                    }
                                    // (
                                    //     AnyJsAssignmentPattern::JsArrayAssignmentPattern(left),
                                    //     AnyJsExpression::JsArrayExpression(right)
                                    // ) => {
                                        // LeftRightKind::Arrays {
                                        //     left: left.elements().iter(),
                                        //     right: right.elements().iter()
                                        // }
                                    // }
                                    _ => LeftRightKind::None
                                }
                            }


                            _ => {
                                LeftRightKind::None
                            },
                        }
                    } else {
                        LeftRightKind::None
                    }
                }
                _ => self.current_pair.clone(),
            };

            if self.pair_queue.is_empty() {
                self.current_pair = LeftRightKind::None;
            }
            match new_pair {
                LeftRightKind::None => {
                    if let Some(pair) = self.pair_queue.pop_front() {
                        self.current_pair = pair;
                        continue;
                    } else {
                        return None;
                    }
                }
                LeftRightKind::Identifiers { left, right } => {
                    return Some((left, right));
                }
                LeftRightKind::Object { .. } | LeftRightKind::Arrays { .. } => {
                    self.pair_queue.push_back(self.current_pair.clone());
                    self.current_pair = new_pair;
                    continue;
                }
            }
        }
    }
}

fn track_same_identifiers(
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

fn compare_self(
    pair_kind: LeftRightKind,
    incorrect_bindings: &mut Vec<(JsIdentifierAssignment, JsReferenceIdentifier)>,
) {
    let mut identifiers = Identifiers {
        current_pair: pair_kind.clone(),
        pair_queue: VecDeque::new(),
    };

    while let Some((left, right)) = identifiers.next() {
        if let Some(pair) = track_same_identifiers(&left, &right) {
            incorrect_bindings.push(pair);
        }
    }

    // 'outer: loop {
    //     match (inner_left, inner_right) {
    //         (Some(inner_left), Some(inner_right)) => match (&outer_left, &outer_right) {
    //             (
    //                 AnyJsAssignmentPattern::JsArrayAssignmentPattern(left),
    //                 AnyJsExpression::JsArrayExpression(right),
    //             ) => {
    //                 let mut left_elements = left.elements().iter();
    //                 let mut right_elements = right.elements().iter();
    //
    //                 'inner: while let (Some(left_element), Some(right_element)) =
    //                     (left_elements.next(), right_elements.next())
    //                 {
    //                     let left_element = left_element.ok();
    //                     let right_element = right_element.ok();
    //
    //                     match (left_element, right_element) {
    //                         (
    //                             Some(AnyJsArrayAssignmentPatternElement::AnyJsAssignmentPattern(
    //                                 left,
    //                             )),
    //                             Some(AnyJsArrayElement::AnyJsExpression(right)),
    //                         ) => {
    //                             if let Some((left, right)) = track_same_identifiers(&left, &right) {
    //                                 incorrect_bindings.push((left, right));
    //                             }
    //                             continue 'inner;
    //                         }
    //
    //                         _ => break,
    //                     }
    //                 }
    //             }
    //             _ => {}
    //         },
    //         (None, None) => {
    //             if let Some(result) = track_same_identifiers(&outer_left, &outer_right) {
    //                 incorrect_bindings.push(result);
    //             }
    //         }
    //         _ => unreachable!("inner_left and inner_right need to have the same Option value"),
    //     }
    // }
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
                        if let Ok(pair) = LeftRightKind::try_from((left, right)) {
                            compare_self(pair, &mut state);
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
