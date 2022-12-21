use rome_analyze::{context::RuleContext, declare_rule, Ast, Rule, RuleDiagnostic};
use rome_console::markup;
use rome_js_syntax::{
    AnyJsArrayAssignmentPatternElement, AnyJsArrayElement, AnyJsAssignment, AnyJsAssignmentPattern,
    AnyJsExpression, AnyJsObjectAssignmentPatternMember, AnyJsObjectMember,
    JsArrayAssignmentPattern, JsArrayExpression, JsAssignmentExpression, JsAssignmentOperator,
    JsIdentifierAssignment, JsIdentifierExpression, JsObjectAssignmentPattern, JsObjectExpression,
    JsReferenceIdentifier,
};

// fn handle_array_assignment() {}
//
// fn compare_elements(
//     left: &AnyJsAssignmentPattern,
//     right: &AnyJsExpression,
//     incorrect_bindings: &mut Vec<(JsIdentifierAssignment, JsReferenceIdentifier)>,
// ) -> Option<()> {
//     match (left, right) {
//         (
//             AnyJsAssignmentPattern::JsArrayAssignmentPattern(left),
//             AnyJsExpression::JsArrayExpression(right),
//         ) => {
//             let mut left_elements = left.elements().iter();
//             let mut right_elements = right.elements().iter();
//
//             while let (Some(left_element), Some(right_element)) =
//                 (left_elements.next(), right_elements.next())
//             {
//                 let left_element = left_element.ok();
//                 let right_element = right_element.ok();
//
//                 match (left_element, right_element) {
//                     (
//                         Some(AnyJsArrayAssignmentPatternElement::AnyJsAssignmentPattern(left)),
//                         Some(AnyJsArrayElement::AnyJsExpression(right)),
//                     ) => {
//                         if let Some((left, right)) = track_same_identifiers(&left, &right) {
//                             incorrect_bindings.push((left, right));
//                         }
//                         continue 'inner;
//                     }
//
//                     _ => break,
//                 }
//             }
//         }
//         (
//             AnyJsAssignmentPattern::JsObjectAssignmentPattern(left),
//             AnyJsExpression::JsObjectExpression(right),
//         ) => {
//             let mut left_elements = left.properties().iter();
//             let mut right_elements = right.members().iter();
//
//             while let (Some(left_element), Some(right_element)) =
//                 (left_elements.next(), right_elements.next())
//             {
//                 let left_element = left_element.ok();
//                 let right_element = right_element.ok();
//
//                 match (left_element, right_element) {
//                     (
//                         Some(AnyJsObjectAssignmentPatternMember::JsObjectAssignmentPatternShorthandProperty(left)),
//                         Some(AnyJsObjectMember::JsShorthandPropertyObjectMember(right)),
//                     ) => {
//                         let left_identifier = left.identifier().ok()?;
//                         let right_identifier = right.name().ok()?;
//                         if let Some((left, right)) = track_same_identifiers(&left_identifier, &right_identifier) {
//                             incorrect_bindings.push((left, right));
//                         }
//                         continue 'inner;
//                     }
//
//                     _ => break,
//                 }
//             }
//         }
//         _ => return,
//     };
//
//     Some(())

// while let (Some(left_element), Some(right_element)) =
//     (left_elements.next(), right_elements.next())
// {
//     let left_element = left_element.ok();
//     let right_element = right_element.ok();
//
//     match (left_element, right_element) {
//         (
//             Some(AnyJsArrayAssignmentPatternElement::AnyJsAssignmentPattern(left)),
//             Some(AnyJsArrayElement::AnyJsExpression(right)),
//         ) => {
//             if let Some((left, right)) = track_same_identifiers(&left, &right) {
//                 incorrect_bindings.push((left, right));
//             }
//             continue 'inner;
//         }
//
//         _ => break,
//     }
// }
// }
use rome_rowan::{AstNode, AstSeparatedList};

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
    Ignore,
    Identifiers {
        left: JsIdentifierAssignment,
        right: JsIdentifierExpression,
    },

    Arrays {
        left: JsArrayAssignmentPattern,
        right: JsArrayExpression,
    },

    Object {
        left: JsObjectAssignmentPattern,
        right: JsObjectExpression,
    },
}

enum Left {
    JsIdentifierAssignment(JsIdentifierAssignment),
    JsArrayAssignmentPattern(JsArrayAssignmentPattern),
    JsObjectAssignmentPattern(JsObjectAssignmentPattern),
}

enum Right {
    JsIdentifierExpression(JsIdentifierExpression),
    JsArrayExpression(JsArrayExpression),
    JsObjectExpression(JsObjectExpression),
}

struct Identifiers {
    current_pair: LeftRightKind,
    previous_pair: LeftRightKind,
}

impl Iterator for Identifiers {
    type Item = (JsIdentifierAssignment, JsIdentifierExpression);

    fn next(&mut self) -> Option<Self::Item> {
        let result = match &self.current_pair {
            LeftRightKind::Ignore => None,
            LeftRightKind::Identifiers { left, right } => Some((left.clone(), right.clone())),
            LeftRightKind::Arrays { left, right } => {
                // left.elements().iter()
                None
            }
            LeftRightKind::Object { .. } => None,
        };

        if self.previous_pair.is_ignore() {
            self.current_pair = LeftRightKind::Ignore;
        }

        result
    }
}

impl LeftRightKind {
    const fn is_identifiers(&self) -> bool {
        matches!(self, LeftRightKind::Identifiers { .. })
    }

    const fn is_ignore(&self) -> bool {
        matches!(self, LeftRightKind::Ignore)
    }
}

impl From<(AnyJsAssignmentPattern, AnyJsExpression)> for LeftRightKind {
    fn from((left, right): (AnyJsAssignmentPattern, AnyJsExpression)) -> Self {
        match (left, right) {
            (
                AnyJsAssignmentPattern::JsArrayAssignmentPattern(left),
                AnyJsExpression::JsArrayExpression(right),
            ) => LeftRightKind::Arrays { left, right },

            (
                AnyJsAssignmentPattern::JsObjectAssignmentPattern(left),
                AnyJsExpression::JsObjectExpression(right),
            ) => LeftRightKind::Object { left, right },

            (
                AnyJsAssignmentPattern::AnyJsAssignment(AnyJsAssignment::JsIdentifierAssignment(
                    left,
                )),
                AnyJsExpression::JsIdentifierExpression(right),
            ) => LeftRightKind::Identifiers { left, right },

            _ => LeftRightKind::Ignore,
        }
    }
}

fn track_same_identifiers(
    left: &JsIdentifierAssignment,
    right: &JsIdentifierExpression,
) -> Option<(JsIdentifierAssignment, JsReferenceIdentifier)> {
    let left_value = left.name_token().ok()?;
    let right = right.name().ok()?;
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
        previous_pair: LeftRightKind::Ignore,
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
                        compare_self(LeftRightKind::from((left, right)), &mut state);
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
