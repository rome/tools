//! The intermediate representation of a RegEx
//! in a tree based structure.

use crate::{Parser, Span};
use bitflags::bitflags;

bitflags! {
    pub struct Flags: u8 {
        /// With this flag the search looks for all matches, without this flag
        /// only the first match is returned
        const G = 0b00000001;
        /// Multiline mode
        const M = 0b00000010;
        /// Case-insensitive search
        const I = 0b00000100;
        /// "dotall" mode, that allows `.` to match newlines (`\n`)
        const S = 0b00001000;
        /// Enables full unicode support
        const U = 0b00010000;
        /// "Sticky" mode
        const Y = 0b00100000;
    }
}

/// The structure that represents a regular expression.
///
/// It contains the actual RegEx node, and the flags for this expression.
#[derive(Debug, Clone)]
pub struct Regex {
    pub node: Node,
    pub flags: Flags,
}

/// The tree structure that is used to represent parsed
/// RegEx patterns.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Node {
    /// An empty regex node.
    Empty,
    /// An "either or". (e.g. `a|b`)
    Disjunction(Span, Vec<Node>),
    /// A single assertion.
    Assertion(Span, AssertionKind),
    /// A concatenation of regex nodes. (e.g. `ab`)
    Alternative(Span, Vec<Node>),
    /// A single character literal. The String represents the raw string representing the literal
    /// which is used to turn the node back into a string without writing explicit newlines for example.
    Literal(Span, char, String),
    /// Matches a character class (e.g. `\d` or `\w`).
    ///
    /// The bool argument indicates if this perl class is negated.
    PerlClass(Span, ClassPerlKind, bool),
    /// A back reference to a previous group (`\1`, `\2`, ...).
    BackReference(Span, u32),
    /// A `.` that matches everything.
    Dot(Span),
    /// A class of multiple characters such as `[A-Z0-9]`
    CharacterClass(Span, CharacterClass),
    /// A grouped pattern
    Group(Span, Group),
    /// A quantifier which optionally matches or matches multiple times.
    /// `bool` indicates whether a lazy quantifier (`?`) is present after it.
    Quantifier(Span, Box<Node>, QuantifierKind, bool),
    /// A reference to a group using a name
    NamedBackReference(Span, String),
}

impl Node {
    /// if this node is an alternative, yield an iterator over those nodes, otherwise yield the node itself.
    pub fn expanded_nodes(&mut self) -> Box<dyn Iterator<Item = &mut Node> + '_> {
        if let Node::Alternative(_, nodes) = self {
            Box::new((*nodes).iter_mut())
        } else {
            Box::new(Some(self).into_iter())
        }
    }

    /// get the span of this node, returns [`None`] if the node is an empty node.
    pub fn span(&self) -> Option<Span> {
        Some(
            match self {
                Node::Empty => return None,
                Node::Disjunction(s, _) => s,
                Node::Assertion(s, _) => s,
                Node::Alternative(s, _) => s,
                Node::Literal(s, _, _) => s,
                Node::PerlClass(s, _, _) => s,
                Node::BackReference(s, _) => s,
                Node::Dot(s) => s,
                Node::CharacterClass(s, _) => s,
                Node::Group(s, _) => s,
                Node::Quantifier(s, _, _, _) => s,
                Node::NamedBackReference(s, _) => s,
            }
            .to_owned(),
        )
    }

    /// check if this node is equal to some text
    pub fn is(&self, src: impl AsRef<str>, text: impl AsRef<str>) -> bool {
        if let Some(span) = self.span() {
            &src.as_ref()[span.as_range()] == text.as_ref()
        } else {
            text.as_ref().is_empty()
        }
    }

    pub fn text<'a>(&self, src: &'a str) -> &'a str {
        if let Some(span) = self.span() {
            &src[span.as_range()]
        } else {
            ""
        }
    }

    /// create a new node from a string. This method is mostly just used for making simple nodes
    /// for replacement.
    pub fn from_string(string: impl AsRef<str>) -> Option<Self> {
        Parser::new_from_pattern_and_flags(
            string.as_ref(),
            0,
            0,
            crate::EcmaVersion::ES2021,
            false,
            Flags::empty(),
        )
        .parse()
        .ok()
        .map(|x| x.node)
    }
}

impl ToString for Node {
    fn to_string(&self) -> String {
        match self {
            Node::Alternative(_, nodes) => nodes
                .iter()
                .map(|x| x.to_string())
                .collect::<Vec<_>>()
                .join(""),
            Node::Empty => Default::default(),
            Node::Disjunction(_, nodes) => nodes
                .iter()
                .map(|x| x.to_string())
                .collect::<Vec<_>>()
                .join("|"),
            Node::Assertion(_, kind) => kind.to_string(),
            Node::Literal(_, _, string) => string.to_owned(),
            Node::Dot(_) => ".".to_string(),
            Node::NamedBackReference(_, string) => {
                format!("\\k<{}>", string)
            }
            Node::BackReference(_, num) => {
                format!("\\{}", num)
            }
            Node::CharacterClass(_, CharacterClass { members, negated }) => {
                format!(
                    "[{}{}]",
                    if *negated { "^" } else { "" },
                    members
                        .iter()
                        .map(|x| x.to_string())
                        .collect::<Vec<_>>()
                        .join("")
                )
            }
            Node::Quantifier(_, node, kind, lazy) => {
                let kind_string = match kind {
                    QuantifierKind::AtLeastOne => "+".to_string(),
                    QuantifierKind::Multiple => "*".to_string(),
                    QuantifierKind::Optional => "?".to_string(),
                    QuantifierKind::Number(num) => format!("{{{}}}", num),
                    QuantifierKind::Between(from, to) => format!(
                        "{{{},{}}}",
                        from,
                        to.map(|x| x.to_string()).unwrap_or_default()
                    ),
                };
                format!(
                    "{}{}{}",
                    node.to_string(),
                    kind_string,
                    if *lazy { "?" } else { "" }
                )
            }
            Node::Group(
                _,
                Group {
                    name,
                    noncapturing,
                    inner,
                },
            ) => {
                format!(
                    "({}{})",
                    if *noncapturing {
                        "?:".to_string()
                    } else if let Some(name) = name {
                        format!("\\<{}>", name)
                    } else {
                        "".to_string()
                    },
                    inner.to_string()
                )
            }
            Node::PerlClass(_, kind, negative) => match kind {
                ClassPerlKind::Digit if *negative => "\\D".to_string(),
                ClassPerlKind::Digit => "\\d".to_string(),
                ClassPerlKind::Space if *negative => "\\S".to_string(),
                ClassPerlKind::Space => "\\s".to_string(),
                ClassPerlKind::Word if *negative => "\\W".to_string(),
                ClassPerlKind::Word => "\\w".to_string(),
                ClassPerlKind::Unicode(a, b) => {
                    format!(
                        "\\{}{{{}{}}}",
                        if *negative { "P" } else { "p" },
                        if let Some(a) = a {
                            format!("{}=", a)
                        } else {
                            "".to_string()
                        },
                        b
                    )
                }
            },
        }
    }
}

impl ToString for AssertionKind {
    fn to_string(&self) -> String {
        match self {
            AssertionKind::StartOfLine => "^".to_string(),
            AssertionKind::EndOfLine => "$".to_string(),
            AssertionKind::WordBoundary => r"\b".to_string(),
            AssertionKind::NonWordBoundary => r"\B".to_string(),
            AssertionKind::Lookahead(node) => format!("(?={})", node.to_string()),
            AssertionKind::NegativeLookahead(node) => format!("(?!{})", node.to_string()),
            AssertionKind::Lookbehind(node) => format!("(?<={})", node.to_string()),
            AssertionKind::NegativeLookbehind(node) => format!("(?<!{})", node.to_string()),
        }
    }
}

impl ToString for CharacterClassMember {
    fn to_string(&self) -> String {
        match self {
            CharacterClassMember::Range(a, b) => format!("{}-{}", a.to_string(), b.to_string()),
            CharacterClassMember::Single(node) => node.to_string(),
        }
    }
}

/// A grouped pattern which can later be referred to
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Group {
    /// Whether this group cannot be later referred to with `$0` for example
    pub noncapturing: bool,
    pub inner: Box<Node>,
    pub name: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum QuantifierKind {
    /// `?`
    Optional,
    /// `*`
    Multiple,
    /// `+`
    AtLeastOne,
    /// `{number}`
    Number(u32),
    /// `{number,number}`. if the second option is None it is "between X and unlimited times"
    Between(u32, Option<u32>),
}

impl QuantifierKind {
    /// Returns `true` if the quantifier_kind is [`QuantifierKind::AtLeastOne`].
    pub fn is_at_least_one(&self) -> bool {
        matches!(self, Self::AtLeastOne)
    }

    /// Returns `true` if the quantifier_kind is [`QuantifierKind::Multiple`].
    pub fn is_multiple(&self) -> bool {
        matches!(self, Self::Multiple)
    }

    /// Returns `true` if the quantifier_kind is [`QuantifierKind::Optional`].
    pub fn is_optional(&self) -> bool {
        matches!(self, Self::Optional)
    }
}

/// A class matching multiple characters or ranges of characters
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CharacterClass {
    pub negated: bool,
    pub members: Vec<CharacterClassMember>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CharacterClassMember {
    Range(Node, Node),
    Single(Node),
}

impl CharacterClassMember {
    pub fn is(&self, src: impl AsRef<str>, text: impl AsRef<str>) -> bool {
        let src = src.as_ref();
        match self {
            CharacterClassMember::Range(a, b) => {
                format!("{}-{}", a.text(src), b.text(src)) == text.as_ref()
            }
            CharacterClassMember::Single(node) => node.text(src) == text.as_ref(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AssertionKind {
    /// `^`
    StartOfLine,
    /// `$`
    EndOfLine,
    /// `\b`
    WordBoundary,
    /// `\B`
    NonWordBoundary,
    /// `x(?=y)`
    Lookahead(Box<Node>),
    /// `x(?!y)`
    NegativeLookahead(Box<Node>),
    /// `(?<=y)x`
    Lookbehind(Box<Node>),
    /// `(?<!y)x`
    NegativeLookbehind(Box<Node>),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ClassPerlKind {
    Digit,
    Word,
    Space,
    Unicode(Option<String>, String),
}
