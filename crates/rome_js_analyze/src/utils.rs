use rome_js_factory::make;
use rome_js_syntax::{JsAnyRoot, JsAnyStatement, JsLanguage, JsModuleItemList, JsStatementList, T};
use rome_rowan::{AstNode, BatchMutation};
use std::borrow::Cow;

pub mod rename;
#[cfg(test)]
pub mod tests;

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
pub(crate) fn remove_statement<N>(
    mutation: &mut BatchMutation<JsLanguage, JsAnyRoot>,
    node: &N,
) -> Option<()>
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
