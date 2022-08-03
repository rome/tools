use crate::js::bindings::parameters::ParameterLayout;
use crate::prelude::*;

use rome_js_syntax::{
    JsAnyConstructorParameter, JsAnyParameter, JsConstructorParameterList, JsLanguage,
    JsParameterList, JsSyntaxKind,
};
use rome_rowan::{declare_node_union, AstSeparatedListNodesIterator, SyntaxResult};

#[derive(Debug, Clone, Default)]
pub struct FormatJsParameterList;

impl FormatRule<JsParameterList> for FormatJsParameterList {
    type Context = JsFormatContext;

    fn fmt(&self, node: &JsParameterList, f: &mut JsFormatter) -> FormatResult<()> {
        FormatParameterList::with_layout(
            &AnyParameterList::from(node.clone()),
            ParameterLayout::Default,
        )
        .fmt(f)
    }
}

#[derive(Debug, Copy, Clone)]
pub(crate) struct FormatParameterList<'a> {
    list: &'a AnyParameterList,
    layout: Option<ParameterLayout>,
}

impl<'a> FormatParameterList<'a> {
    pub fn with_layout(list: &'a AnyParameterList, layout: ParameterLayout) -> Self {
        Self {
            list,
            layout: Some(layout),
        }
    }
}

impl Format<JsFormatContext> for FormatParameterList<'_> {
    fn fmt(&self, f: &mut Formatter<JsFormatContext>) -> FormatResult<()> {
        match self.layout {
            None | Some(ParameterLayout::Default) => {
                // The trailing separator is disallowed if the last element in the list is a rest parameter
                let has_trailing_rest = match self.list.last() {
                    Some(elem) => matches!(
                        elem?,
                        AnyParameter::JsAnyParameter(JsAnyParameter::JsRestParameter(_))
                            | AnyParameter::JsAnyConstructorParameter(
                                JsAnyConstructorParameter::JsRestParameter(_)
                            )
                    ),
                    None => false,
                };

                let trailing_separator = if has_trailing_rest {
                    TrailingSeparator::Disallowed
                } else {
                    TrailingSeparator::Allowed
                };

                let mut join = f.join_nodes_with_soft_line();

                match self.list {
                    AnyParameterList::JsParameterList(list) => {
                        let entries = list
                            .format_separated(JsSyntaxKind::COMMA)
                            .with_trailing_separator(trailing_separator)
                            .zip(list.iter());

                        for (format_entry, node) in entries {
                            join.entry(node?.syntax(), &format_entry);
                        }
                    }
                    AnyParameterList::JsConstructorParameterList(list) => {
                        let entries = list
                            .format_separated(JsSyntaxKind::COMMA)
                            .with_trailing_separator(trailing_separator)
                            .zip(list.iter());

                        for (format_entry, node) in entries {
                            join.entry(node?.syntax(), &format_entry);
                        }
                    }
                }

                join.finish()
            }
            Some(ParameterLayout::Hug) => {
                let mut join = f.join_with(space());

                match self.list {
                    AnyParameterList::JsParameterList(list) => join.entries(
                        list.format_separated(JsSyntaxKind::COMMA)
                            .with_trailing_separator(TrailingSeparator::Omit),
                    ),
                    AnyParameterList::JsConstructorParameterList(list) => join.entries(
                        list.format_separated(JsSyntaxKind::COMMA)
                            .with_trailing_separator(TrailingSeparator::Omit),
                    ),
                };

                join.finish()
            }
        }
    }
}

#[derive(Debug)]
pub(crate) enum AnyParameterList {
    JsParameterList(JsParameterList),
    JsConstructorParameterList(JsConstructorParameterList),
}

impl From<JsParameterList> for AnyParameterList {
    fn from(list: JsParameterList) -> Self {
        AnyParameterList::JsParameterList(list)
    }
}

impl From<JsConstructorParameterList> for AnyParameterList {
    fn from(list: JsConstructorParameterList) -> Self {
        AnyParameterList::JsConstructorParameterList(list)
    }
}

impl AnyParameterList {
    pub fn len(&self) -> usize {
        match self {
            AnyParameterList::JsParameterList(parameters) => parameters.len(),
            AnyParameterList::JsConstructorParameterList(parameters) => parameters.len(),
        }
    }

    pub fn first(&self) -> Option<SyntaxResult<AnyParameter>> {
        Some(match self {
            AnyParameterList::JsParameterList(parameters) => {
                parameters.first()?.map(|parameter| parameter.into())
            }
            AnyParameterList::JsConstructorParameterList(parameters) => {
                parameters.first()?.map(|parameter| parameter.into())
            }
        })
    }

    pub fn iter(&self) -> AnyParameterListNodeIter {
        match self {
            AnyParameterList::JsParameterList(list) => {
                AnyParameterListNodeIter::JsParameterList(list.iter())
            }
            AnyParameterList::JsConstructorParameterList(list) => {
                AnyParameterListNodeIter::JsConstructorParameterList(list.iter())
            }
        }
    }

    pub fn last(&self) -> Option<SyntaxResult<AnyParameter>> {
        Some(match self {
            AnyParameterList::JsParameterList(parameters) => {
                parameters.last()?.map(|parameter| parameter.into())
            }
            AnyParameterList::JsConstructorParameterList(parameters) => {
                parameters.last()?.map(|parameter| parameter.into())
            }
        })
    }
}

pub(crate) enum AnyParameterListNodeIter {
    JsParameterList(AstSeparatedListNodesIterator<JsLanguage, JsAnyParameter>),
    JsConstructorParameterList(
        AstSeparatedListNodesIterator<JsLanguage, JsAnyConstructorParameter>,
    ),
}

impl Iterator for AnyParameterListNodeIter {
    type Item = SyntaxResult<AnyParameter>;

    fn next(&mut self) -> Option<Self::Item> {
        Some(match self {
            AnyParameterListNodeIter::JsParameterList(inner) => {
                inner.next()?.map(AnyParameter::from)
            }
            AnyParameterListNodeIter::JsConstructorParameterList(inner) => {
                inner.next()?.map(AnyParameter::from)
            }
        })
    }
}

declare_node_union! {
    pub(crate) AnyParameter = JsAnyConstructorParameter | JsAnyParameter
}
