pub mod batch;
pub mod react;
pub mod rename;
#[cfg(test)]
pub mod tests;

use rome_js_factory::make;
use rome_js_semantic::SemanticModel;
use rome_js_syntax::JsSyntaxKind::JS_IMPORT;
use rome_js_syntax::{
    JsAnyStatement, JsIdentifierBinding, JsIdentifierExpression, JsLanguage, JsModuleItemList,
    JsStatementList, JsStaticMemberExpression, JsVariableDeclaration, JsVariableDeclarator,
    JsVariableDeclaratorList, JsVariableStatement, T,
};
use rome_rowan::{declare_node_union, AstNode, AstSeparatedList, BatchMutation};
use std::borrow::Cow;

#[derive(Debug, PartialEq)]
pub(crate) enum EscapeError {
    EscapeAtEndOfString,
    InvalidEscapedChar(char),
}

struct InterpretEscapedString<'a> {
    s: std::str::Chars<'a>,
}

impl<'a> Iterator for InterpretEscapedString<'a> {
    type Item = Result<char, EscapeError>;

    fn next(&mut self) -> Option<Self::Item> {
        self.s.next().map(|c| match c {
            '\\' => match self.s.next() {
                None => Err(EscapeError::EscapeAtEndOfString),
                Some('n') => Ok('\n'),
                Some('\\') => Ok('\\'),
                Some(c) => Err(EscapeError::InvalidEscapedChar(c)),
            },
            c => Ok(c),
        })
    }
}

/// unescape
///
pub(crate) fn escape_string(s: &str) -> Result<String, EscapeError> {
    (InterpretEscapedString { s: s.chars() }).collect()
}

pub trait ToCamelCase {
    /// Return the camel case form of the input parameter.
    /// If it is already in camel case, nothing is done.
    ///
    /// This method do not address abbreviations and acronyms.
    fn to_camel_case(&self) -> Cow<str>;
}

impl ToCamelCase for str {
    fn to_camel_case(&self) -> Cow<str> {
        to_camel_case(self)
    }
}

/// Return the camel case form of the input parameter.
/// If it is already in camel case, nothing is done.
///
/// This method do not address abbreviations and acronyms.
pub fn to_camel_case(input: &str) -> Cow<str> {
    pub enum ForceNext {
        Uppercase,
        Lowercase,
    }

    let mut force_next = None;
    let mut chars = input.char_indices();
    let mut last_i = input.len() - 1;

    while let Some((i, chr)) = chars.next() {
        if i == 0 && chr.is_uppercase() {
            chars = input.char_indices();
            force_next = Some(ForceNext::Lowercase);
            last_i = i;
            break;
        }

        if !chr.is_alphanumeric() {
            if i == 0 {
                force_next = Some(ForceNext::Lowercase);
            } else {
                force_next = Some(ForceNext::Uppercase);
            }
            last_i = i;
            break;
        }
    }

    if last_i >= (input.len() - 1) {
        Cow::Borrowed(input)
    } else {
        let mut output = Vec::with_capacity(input.len());
        output.extend_from_slice(input[..last_i].as_bytes());
        //SAFETY: bytes were already inside a valid &str
        let mut output = unsafe { String::from_utf8_unchecked(output) };

        for (_, chr) in chars {
            if !chr.is_alphanumeric() {
                force_next = Some(ForceNext::Uppercase);
                continue;
            }

            match force_next {
                Some(ForceNext::Uppercase) => {
                    output.extend(chr.to_uppercase());
                }
                Some(ForceNext::Lowercase) => {
                    output.extend(chr.to_lowercase());
                }
                None => {
                    output.push(chr);
                }
            }

            force_next = None;
        }

        Cow::Owned(output)
    }
}

/// Utility function to remove a statement node from a syntax tree, by either
/// removing the node from its parent if said parent is a statement list or
/// module item list, or by replacing the statement node with an empty statement
pub(crate) fn remove_statement<N>(mutation: &mut BatchMutation<JsLanguage>, node: &N) -> Option<()>
where
    N: AstNode<Language = JsLanguage> + Into<JsAnyStatement>,
{
    let parent = node.syntax().parent()?;

    if JsStatementList::can_cast(parent.kind()) || JsModuleItemList::can_cast(parent.kind()) {
        mutation.remove_node(node.clone());
    } else {
        mutation.replace_node(
            node.clone().into(),
            JsAnyStatement::JsEmptyStatement(make::js_empty_statement(make::token(T![;]))),
        );
    }

    Some(())
}

/// Removes the declarator, and:
/// 1 - removes the statement if the declaration only has one declarator;
/// 2 - removes commas around the declarator to keep the declaration list valid.
pub(crate) fn remove_declarator(
    batch: &mut BatchMutation<JsLanguage>,
    declarator: &JsVariableDeclarator,
) -> Option<()> {
    let list = declarator.parent::<JsVariableDeclaratorList>()?;
    let declaration = list.parent::<JsVariableDeclaration>()?;

    if list.syntax_list().len() == 1 {
        let statement = declaration.parent::<JsVariableStatement>()?;
        batch.remove_node(statement);
    } else {
        let mut elements = list.elements();

        // Find the declarator we want to remove
        // remove its trailing comma, if there is one
        let mut previous_element = None;
        for element in elements.by_ref() {
            if let Ok(node) = element.node() {
                if node == declarator {
                    batch.remove_node(node.clone());
                    if let Some(comma) = element.trailing_separator().ok().flatten() {
                        batch.remove_token(comma.clone());
                    }
                    break;
                }
            }
            previous_element = Some(element);
        }

        // if it is the last declarator of the list
        // removes the comma before this element
        let is_last = elements.next().is_none();
        if is_last {
            if let Some(element) = previous_element {
                if let Some(comma) = element.trailing_separator().ok().flatten() {
                    batch.remove_token(comma.clone());
                }
            }
        }
    }

    Some(())
}

#[test]
fn ok_to_camel_case() {
    assert_eq!(to_camel_case("camelCase"), Cow::Borrowed("camelCase"));
    assert_eq!(
        to_camel_case("longCamelCase"),
        Cow::Borrowed("longCamelCase")
    );

    assert!(matches!(
        to_camel_case("CamelCase"),
        Cow::Owned(s) if s.as_str() == "camelCase"
    ));
    assert!(matches!(
        to_camel_case("_camelCase"),
        Cow::Owned(s) if s.as_str() == "camelCase"
    ));
    assert!(matches!(
        to_camel_case("_camelCase_"),
        Cow::Owned(s) if s.as_str() == "camelCase"
    ));
    assert!(matches!(
        to_camel_case("_camel_Case_"),
        Cow::Owned(s) if s.as_str() == "camelCase"
    ));
    assert!(matches!(
        to_camel_case("_camel_case_"),
        Cow::Owned(s) if s.as_str() == "camelCase"
    ));
    assert!(matches!(
        to_camel_case("_camel_case"),
        Cow::Owned(s) if s.as_str() == "camelCase"
    ));
    assert!(matches!(
        to_camel_case("camel_case"),
        Cow::Owned(s) if s.as_str() == "camelCase"
    ));

    assert!(matches!(
        to_camel_case("LongCamelCase"),
        Cow::Owned(s) if s.as_str() == "longCamelCase"
    ));
    assert!(matches!(
        to_camel_case("long_camel_case"),
        Cow::Owned(s) if s.as_str() == "longCamelCase"
    ));
}

declare_node_union! {
    pub(crate) PossibleCreateElement = JsStaticMemberExpression | JsIdentifierExpression
}

/// Checks if the current node is a possible `createElement` call.
///
/// There are two cases:
///
/// First case
/// ```js
/// React.createElement()
/// ```
/// We check if the node is a static member expression with the specific members. Also, if `React`
/// has been imported in the current scope, we make sure that the binding `React` has been imported
/// from the `"react"` module.
///
/// Second case
///
/// ```js
/// createElement()
/// ```
///
/// The logic of this second case is very similar to the previous one, simply the node that we have
/// to inspect is different.
pub(crate) fn is_react_create_element(
    node: PossibleCreateElement,
    model: &SemanticModel,
) -> Option<bool> {
    let result = match node {
        PossibleCreateElement::JsStaticMemberExpression(node) => {
            let object = node.object().ok()?;
            let member = node.member().ok()?;
            let member = member.as_js_name()?;
            let identifier = object.as_js_identifier_expression()?.name().ok()?;

            let maybe_from_react = identifier.syntax().text_trimmed() == "React"
                && member.syntax().text_trimmed() == "createElement";

            if maybe_from_react {
                let identifier_binding = model.declaration(&identifier);
                if let Some(binding_identifier) = identifier_binding {
                    let binding_identifier =
                        JsIdentifierBinding::cast_ref(binding_identifier.syntax())?;
                    for ancestor in binding_identifier.syntax().ancestors() {
                        if ancestor.kind() == JS_IMPORT {
                            return Some(
                                binding_identifier.syntax().text_trimmed()
                                    == identifier.syntax().text_trimmed(),
                            );
                        }
                    }
                }
            }
            maybe_from_react
        }
        PossibleCreateElement::JsIdentifierExpression(identifier) => {
            let name = identifier.name().ok()?;
            let maybe_from_react = identifier.syntax().text_trimmed() == "createElement";
            if maybe_from_react {
                let identifier_binding = model.declaration(&name);
                if let Some(identifier_binding) = identifier_binding {
                    let binding_identifier =
                        JsIdentifierBinding::cast_ref(identifier_binding.syntax())?;
                    for ancestor in binding_identifier.syntax().ancestors() {
                        if ancestor.kind() == JS_IMPORT {
                            return Some(
                                binding_identifier.syntax().text_trimmed()
                                    == identifier.syntax().text_trimmed(),
                            );
                        }
                    }
                }
            }

            maybe_from_react
        }
    };

    Some(result)
}
