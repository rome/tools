//! This crate provides a RegEx parser which targets the [RegEx syntax] specified
//! by [EcmaScript]
//!
//! [EcmaScript]: https://tc39.es/ecma262
//! [RegEx syntax]: https://tc39.es/ecma262/#sec-patterns

#![deny(rust_2018_idioms)]

mod ir;
#[allow(clippy::range_plus_one)]
mod parser;
#[cfg(test)]
mod tests;
mod unicode;

pub use parser::*;

pub use ir::*;
use std::ops::Range;
pub use unicode::EcmaVersion;

pub type Result<T, E = Error> = std::result::Result<T, E>;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Span {
    /// The offset in the whole file to calculate the absolute position.
    pub offset: usize,
    /// The relative start of this `Span` inside a pattern.
    pub start: usize,
    /// The relative end of this `Span` inside a pattern.
    pub end: usize,
}

impl Span {
    /// Create a new `Span`
    pub fn new(offset: usize, start: usize, end: usize) -> Self {
        Self { offset, start, end }
    }

    /// Calculates the absolute start using `self.offset + self.start`.
    pub fn abs_start(&self) -> usize {
        self.offset + self.start
    }

    /// Calculates the absolute end using `self.offset + self.end`.
    pub fn abs_end(&self) -> usize {
        self.offset + self.end
    }

    pub fn as_range(&self) -> Range<usize> {
        self.abs_start()..self.abs_end()
    }
}

impl From<Range<usize>> for Span {
    fn from(range: Range<usize>) -> Self {
        Span::new(0, range.start, range.end)
    }
}

#[cfg(feature = "rslint_errors")]
impl rslint_errors::Span for Span {
    fn as_range(&self) -> Range<usize> {
        self.abs_start()..self.abs_end()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Error {
    pub span: Span,
    pub message: String,
}

impl Error {
    pub fn new(message: impl ToString, span: Span) -> Self {
        Self {
            span,
            message: message.to_string(),
        }
    }

    pub(crate) fn primary(self, span: impl Into<Span>, _msg: &str) -> Self {
        Self {
            span: span.into(),
            message: self.message,
        }
    }
}

/// A visitor trait for [`Regex`]
///
/// ⚠️ overriding functions may cause inner nodes to not be visited,
/// to avoid this, use `VisitAll` instead.
#[allow(unused_variables)]
pub trait Visit {
    fn visit_regex(&mut self, regex: &Regex) {
        self.visit_node(&regex.node);
    }

    fn visit_node(&mut self, node: &Node) {
        match node {
            Node::Empty => self.visit_empty_node(),
            Node::Disjunction(span, nodes) => self.visit_disjunction(span, nodes),
            Node::Assertion(span, kind) => self.visit_assertion(span, kind),
            Node::Alternative(span, nodes) => self.visit_alternative(span, nodes),
            Node::Literal(span, literal, _) => self.visit_literal(span, *literal),
            Node::PerlClass(span, kind, negated) => {
                self.visit_perl_class(span, kind.to_owned(), *negated)
            }
            Node::BackReference(span, reference) => self.visit_backreference(span, *reference),
            Node::Dot(span) => self.visit_dot(span),
            Node::CharacterClass(span, class) => self.visit_character_class(span, class),
            Node::Group(span, group) => self.visit_group(span, group),
            Node::Quantifier(span, node, kind, lazy) => {
                self.visit_quantifier(span, node, kind.to_owned(), *lazy)
            }
            Node::NamedBackReference(span, backreference) => {
                self.visit_named_backreference(span, backreference)
            }
        }
    }

    fn visit_empty_node(&mut self) {}

    fn visit_disjunction(&mut self, span: &Span, nodes: &[Node]) {
        for node in nodes {
            self.visit_node(node);
        }
    }

    fn visit_assertion(&mut self, span: &Span, kind: &AssertionKind) {
        match kind {
            AssertionKind::Lookahead(node)
            | AssertionKind::Lookbehind(node)
            | AssertionKind::NegativeLookahead(node)
            | AssertionKind::NegativeLookbehind(node) => self.visit_node(node),
            _ => {}
        }
    }

    fn visit_alternative(&mut self, span: &Span, nodes: &[Node]) {
        for node in nodes {
            self.visit_node(node);
        }
    }

    fn visit_literal(&mut self, span: &Span, literal: char) {}

    fn visit_perl_class(&mut self, span: &Span, kind: ClassPerlKind, negated: bool) {}

    fn visit_backreference(&mut self, span: &Span, reference: u32) {}

    fn visit_dot(&mut self, span: &Span) {}

    fn visit_character_class(&mut self, span: &Span, class: &CharacterClass) {
        for member in &class.members {
            match member {
                CharacterClassMember::Range(l, r) => {
                    self.visit_node(l);
                    self.visit_node(r);
                }
                CharacterClassMember::Single(n) => self.visit_node(n),
            }
        }
    }

    fn visit_group(&mut self, span: &Span, group: &Group) {
        self.visit_node(&group.inner)
    }

    fn visit_quantifier(&mut self, span: &Span, node: &Node, kind: QuantifierKind, lazy: bool) {
        self.visit_node(node);
    }

    fn visit_named_backreference(&mut self, span: &Span, backreference: &str) {}
}

#[allow(unused_variables)]
pub trait VisitMut {
    fn visit_regex(&mut self, regex: &mut Regex) {
        self.visit_node(&mut regex.node);
    }

    fn visit_node(&mut self, node: &mut Node) {
        match node {
            Node::Empty => self.visit_empty_node(),
            Node::Disjunction(span, nodes) => self.visit_disjunction(span, nodes),
            Node::Assertion(span, kind) => self.visit_assertion(span, kind),
            Node::Alternative(span, nodes) => self.visit_alternative(span, nodes),
            Node::Literal(span, literal, _) => self.visit_literal(span, literal),
            Node::PerlClass(span, kind, negated) => self.visit_perl_class(span, kind, negated),
            Node::BackReference(span, reference) => self.visit_backreference(span, reference),
            Node::Dot(span) => self.visit_dot(span),
            Node::CharacterClass(span, class) => self.visit_character_class(span, class),
            Node::Group(span, group) => self.visit_group(span, group),
            Node::Quantifier(span, node, kind, lazy) => {
                self.visit_quantifier(span, node, kind, lazy)
            }
            Node::NamedBackReference(span, backreference) => {
                self.visit_named_backreference(span, backreference)
            }
        }
    }

    fn visit_empty_node(&mut self) {}

    fn visit_disjunction(&mut self, span: &Span, nodes: &mut [Node]) {
        for node in nodes {
            self.visit_node(node);
        }
    }

    fn visit_assertion(&mut self, span: &Span, kind: &mut AssertionKind) {
        match kind {
            AssertionKind::Lookahead(node)
            | AssertionKind::Lookbehind(node)
            | AssertionKind::NegativeLookahead(node)
            | AssertionKind::NegativeLookbehind(node) => self.visit_node(node),
            _ => {}
        }
    }

    fn visit_alternative(&mut self, span: &Span, nodes: &mut [Node]) {
        for node in nodes {
            self.visit_node(node);
        }
    }

    fn visit_literal(&mut self, span: &Span, literal: &mut char) {}

    fn visit_perl_class(&mut self, span: &Span, kind: &mut ClassPerlKind, negated: &mut bool) {}

    fn visit_backreference(&mut self, span: &Span, reference: &mut u32) {}

    fn visit_dot(&mut self, span: &Span) {}

    fn visit_character_class(&mut self, span: &Span, class: &mut CharacterClass) {
        for member in &mut class.members {
            match member {
                CharacterClassMember::Range(l, r) => {
                    self.visit_node(l);
                    self.visit_node(r);
                }
                CharacterClassMember::Single(n) => self.visit_node(n),
            }
        }
    }

    fn visit_group(&mut self, span: &Span, group: &mut Group) {
        self.visit_node(&mut *group.inner)
    }

    fn visit_quantifier(
        &mut self,
        span: &Span,
        node: &mut Node,
        kind: &mut QuantifierKind,
        lazy: &mut bool,
    ) {
        self.visit_node(node);
    }

    fn visit_named_backreference(&mut self, span: &Span, backreference: &mut str) {}
}

/// A visitor trait for [`Regex`] which visits all nodes regardless of function overrides.
#[allow(unused_variables)]
pub trait VisitAll {
    #[doc(hidden)]
    fn _visit_node(&mut self, node: &Node) {
        match node {
            Node::Empty => self.visit_empty_node(),
            Node::Disjunction(span, nodes) => {
                self._visit_disjunction(span, nodes);
                self._visit_disjunction(span, nodes)
            }
            Node::Assertion(span, kind) => {
                self.visit_assertion(span, kind);
                self._visit_assertion(span, kind)
            }
            Node::Alternative(span, nodes) => {
                self.visit_alternative(span, nodes);
                self._visit_alternative(span, nodes)
            }
            Node::Literal(span, literal, _) => self.visit_literal(span, *literal),
            Node::PerlClass(span, kind, negated) => {
                self.visit_perl_class(span, kind.to_owned(), *negated)
            }
            Node::BackReference(span, reference) => self.visit_backreference(span, *reference),
            Node::Dot(span) => self.visit_dot(span),
            Node::CharacterClass(span, class) => {
                self.visit_character_class(span, class);
                self._visit_character_class(span, class);
            }
            Node::Group(span, group) => {
                self.visit_group(span, group);
                self._visit_group(span, group)
            }

            Node::Quantifier(span, node, kind, lazy) => {
                self.visit_quantifier(span, node, kind.to_owned(), *lazy);
                self._visit_quantifier(span, node, kind.to_owned(), *lazy);
            }
            Node::NamedBackReference(span, backreference) => {
                self.visit_named_backreference(span, backreference)
            }
        }
    }

    #[doc(hidden)]
    fn _visit_disjunction(&mut self, span: &Span, nodes: &[Node]) {
        for node in nodes {
            self._visit_node(node);
        }
    }

    #[doc(hidden)]
    fn _visit_assertion(&mut self, span: &Span, kind: &AssertionKind) {
        match kind {
            AssertionKind::Lookahead(node)
            | AssertionKind::Lookbehind(node)
            | AssertionKind::NegativeLookahead(node)
            | AssertionKind::NegativeLookbehind(node) => self._visit_node(node),
            _ => {}
        }
    }

    #[doc(hidden)]
    fn _visit_alternative(&mut self, span: &Span, nodes: &[Node]) {
        for node in nodes {
            self.visit_node(node);
            self._visit_node(node);
        }
    }

    #[doc(hidden)]
    fn _visit_character_class(&mut self, span: &Span, class: &CharacterClass) {
        for member in &class.members {
            match member {
                CharacterClassMember::Range(l, r) => {
                    self.visit_node(l);
                    self._visit_node(l);
                    self.visit_node(r);
                    self._visit_node(r);
                }
                CharacterClassMember::Single(n) => {
                    self.visit_node(n);
                    self._visit_node(n)
                }
            }
        }
    }

    #[doc(hidden)]
    fn _visit_group(&mut self, span: &Span, group: &Group) {
        self.visit_node(&group.inner);
        self._visit_node(&group.inner)
    }

    #[doc(hidden)]
    fn _visit_quantifier(&mut self, span: &Span, node: &Node, kind: QuantifierKind, lazy: bool) {
        self.visit_node(node);
        self._visit_node(node);
    }

    fn visit_regex(&mut self, regex: &Regex) {
        self.visit_node(&regex.node);
        self._visit_node(&regex.node);
    }
    fn visit_named_backreference(&mut self, span: &Span, backreference: &str) {}
    fn visit_alternative(&mut self, span: &Span, nodes: &[Node]) {}
    fn visit_literal(&mut self, span: &Span, literal: char) {}
    fn visit_perl_class(&mut self, span: &Span, kind: ClassPerlKind, negated: bool) {}
    fn visit_backreference(&mut self, span: &Span, reference: u32) {}
    fn visit_dot(&mut self, span: &Span) {}
    fn visit_node(&mut self, node: &Node) {}
    fn visit_empty_node(&mut self) {}
    fn visit_group(&mut self, span: &Span, group: &Group) {}
    fn visit_quantifier(&mut self, span: &Span, node: &Node, kind: QuantifierKind, lazy: bool) {}
    fn visit_character_class(&mut self, span: &Span, class: &CharacterClass) {}
    fn visit_assertion(&mut self, span: &Span, kind: &AssertionKind) {}
}

#[allow(unused_variables)]
pub trait VisitAllMut {
    #[doc(hidden)]
    fn _visit_node(&mut self, node: &mut Node) {
        match node {
            Node::Empty => self.visit_empty_node(),
            Node::Disjunction(span, nodes) => {
                self._visit_disjunction(span, nodes);
                self._visit_disjunction(span, nodes)
            }
            Node::Assertion(span, kind) => {
                self.visit_assertion(span, kind);
                self._visit_assertion(span, kind)
            }
            Node::Alternative(span, nodes) => {
                self.visit_alternative(span, nodes);
                self._visit_alternative(span, nodes)
            }
            Node::Literal(span, literal, _) => self.visit_literal(span, literal),
            Node::PerlClass(span, kind, negated) => self.visit_perl_class(span, kind, negated),
            Node::BackReference(span, reference) => self.visit_backreference(span, reference),
            Node::Dot(span) => self.visit_dot(span),
            Node::CharacterClass(span, class) => {
                self.visit_character_class(span, class);
                self._visit_character_class(span, class);
            }
            Node::Group(span, group) => {
                self.visit_group(span, group);
                self._visit_group(span, group)
            }

            Node::Quantifier(span, node, kind, lazy) => {
                self.visit_quantifier(span, node, kind, lazy);
                self._visit_quantifier(span, node, kind, lazy);
            }
            Node::NamedBackReference(span, backreference) => {
                self.visit_named_backreference(span, backreference)
            }
        }
    }

    #[doc(hidden)]
    fn _visit_disjunction(&mut self, span: &Span, nodes: &mut [Node]) {
        for node in nodes {
            self._visit_node(node);
        }
    }

    #[doc(hidden)]
    fn _visit_assertion(&mut self, span: &Span, kind: &mut AssertionKind) {
        match kind {
            AssertionKind::Lookahead(node)
            | AssertionKind::Lookbehind(node)
            | AssertionKind::NegativeLookahead(node)
            | AssertionKind::NegativeLookbehind(node) => self._visit_node(node),
            _ => {}
        }
    }

    #[doc(hidden)]
    fn _visit_alternative(&mut self, span: &Span, nodes: &mut [Node]) {
        for node in nodes {
            self.visit_node(node);
            self._visit_node(node);
        }
    }

    #[doc(hidden)]
    fn _visit_character_class(&mut self, span: &Span, class: &mut CharacterClass) {
        for member in &mut class.members {
            match member {
                CharacterClassMember::Range(l, r) => {
                    self.visit_node(l);
                    self._visit_node(l);
                    self.visit_node(r);
                    self._visit_node(r);
                }
                CharacterClassMember::Single(n) => {
                    self.visit_node(n);
                    self._visit_node(n)
                }
            }
        }
    }

    #[doc(hidden)]
    fn _visit_group(&mut self, span: &Span, group: &mut Group) {
        self.visit_node(&mut group.inner);
        self._visit_node(&mut group.inner)
    }

    #[doc(hidden)]
    fn _visit_quantifier(
        &mut self,
        span: &Span,
        node: &mut Node,
        kind: &mut QuantifierKind,
        lazy: &mut bool,
    ) {
        self.visit_node(node);
        self._visit_node(node);
    }

    fn visit_regex(&mut self, regex: &mut Regex) {
        self.visit_node(&mut regex.node);
        self._visit_node(&mut regex.node);
    }
    fn visit_named_backreference(&mut self, span: &Span, backreference: &mut str) {}
    fn visit_alternative(&mut self, span: &Span, nodes: &mut [Node]) {}
    fn visit_literal(&mut self, span: &Span, literal: &mut char) {}
    fn visit_perl_class(&mut self, span: &Span, kind: &mut ClassPerlKind, negated: &mut bool) {}
    fn visit_backreference(&mut self, span: &Span, reference: &mut u32) {}
    fn visit_dot(&mut self, span: &Span) {}
    fn visit_node(&mut self, node: &mut Node) {}
    fn visit_empty_node(&mut self) {}
    fn visit_group(&mut self, span: &Span, group: &mut Group) {}
    fn visit_quantifier(
        &mut self,
        span: &Span,
        node: &mut Node,
        kind: &mut QuantifierKind,
        lazy: &mut bool,
    ) {
    }
    fn visit_character_class(&mut self, span: &Span, class: &mut CharacterClass) {}
    fn visit_assertion(&mut self, span: &Span, kind: &mut AssertionKind) {}
}
