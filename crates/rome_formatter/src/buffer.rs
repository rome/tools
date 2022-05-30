use super::{write, Arguments, FormatElement};
use crate::format_element::List;
use crate::group_id::UniqueGroupIdBuilder;
#[cfg(debug_assertions)]
use crate::printed_tokens::PrintedTokens;
use crate::{FormatResult, GroupId};
use rome_rowan::{Language, SyntaxNode, SyntaxToken};
use std::fmt;
use std::fmt::Debug;
use std::ops::{Deref, DerefMut};

pub trait Buffer {
    type Context;

    fn write_element(&mut self, element: FormatElement) -> FormatResult<()>;

    fn write_fmt(mut self: &mut Self, arguments: &Arguments<Self::Context>) -> FormatResult<()> {
        write(&mut self, arguments)
    }

    fn state(&self) -> &FormatState<Self::Context>;

    fn state_mut(&mut self) -> &mut FormatState<Self::Context>;
}

#[derive(Default)]
pub struct FormatState<Context> {
    context: Context,
    group_id_builder: UniqueGroupIdBuilder,
    // This is using a RefCell as it only exists in debug mode,
    // the Formatter is still completely immutable in release builds
    #[cfg(debug_assertions)]
    pub printed_tokens: PrintedTokens,
}

impl<Context> fmt::Debug for FormatState<Context>
where
    Context: Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("FormatContext")
            .field("options", &self.context)
            .finish()
    }
}

impl<Context> FormatState<Context> {
    pub fn new(context: Context) -> Self {
        Self {
            context,
            group_id_builder: Default::default(),
            #[cfg(debug_assertions)]
            printed_tokens: Default::default(),
        }
    }

    /// Returns the [FormatContext] specifying how to format the current CST
    pub fn context(&self) -> &Context {
        &self.context
    }

    /// Creates a new group id that is unique to this document. The passed debug name is used in the
    /// [std::fmt::Debug] of the document if this is a debug build.
    /// The name is unused for production builds and has no meaning on the equality of two group ids.
    pub fn group_id(&self, debug_name: &'static str) -> GroupId {
        self.group_id_builder.group_id(debug_name)
    }

    /// Tracks the given token as formatted
    #[inline]
    pub fn track_token<L: Language>(&mut self, #[allow(unused_variables)] token: &SyntaxToken<L>) {
        cfg_if::cfg_if! {
            if #[cfg(debug_assertions)] {
                self.printed_tokens.track_token(token);
            }
        }
    }

    #[inline]
    pub fn assert_formatted_all_tokens<L: Language>(
        &self,
        #[allow(unused_variables)] root: &SyntaxNode<L>,
    ) {
        cfg_if::cfg_if! {
            if #[cfg(debug_assertions)] {
                self.printed_tokens.assert_all_tracked(root);
            }
        }
    }
}

impl<W: Buffer<Context = Context> + ?Sized, Context> Buffer for &mut W {
    type Context = Context;

    fn write_element(&mut self, element: FormatElement) -> FormatResult<()> {
        (**self).write_element(element)
    }

    fn write_fmt(&mut self, args: &Arguments<Context>) -> FormatResult<()> {
        (**self).write_fmt(args)
    }

    fn state(&self) -> &FormatState<Self::Context> {
        (**self).state()
    }

    fn state_mut(&mut self) -> &mut FormatState<Self::Context> {
        (**self).state_mut()
    }
}

// TODO use Smallvec internally
#[derive(Debug)]
pub struct VecBuffer<'a, Context> {
    context: &'a mut FormatState<Context>,
    elements: Vec<FormatElement>,
}

impl<'a, Context> VecBuffer<'a, Context> {
    pub fn new(context: &'a mut FormatState<Context>) -> Self {
        Self {
            context,
            elements: vec![],
        }
    }

    pub fn with_capacity(capacity: usize, context: &'a mut FormatState<Context>) -> Self {
        Self {
            context,
            elements: Vec::with_capacity(capacity),
        }
    }

    /// Writes the elements from this buffer into the passed buffer
    pub fn write_into(
        &mut self,
        buffer: &mut dyn Buffer<Context = Context>,
    ) -> super::FormatResult<()> {
        for element in self.drain(..) {
            buffer.write_element(element)?;
        }

        Ok(())
    }

    pub fn into_document(self) -> Document {
        Document(self.elements)
    }

    pub fn into_vec(self) -> Vec<FormatElement> {
        self.elements
    }
}

impl<Context> Deref for VecBuffer<'_, Context> {
    type Target = Vec<FormatElement>;

    fn deref(&self) -> &Self::Target {
        &self.elements
    }
}

impl<Context> DerefMut for VecBuffer<'_, Context> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.elements
    }
}

impl<Context> Buffer for VecBuffer<'_, Context> {
    type Context = Context;

    fn write_element(&mut self, element: FormatElement) -> FormatResult<()> {
        match element {
            FormatElement::List(list) => {
                if self.elements.is_empty() {
                    self.elements = list.into_vec()
                } else {
                    self.elements.extend(list.into_vec())
                }
            }
            FormatElement::Empty => {}
            element => self.elements.push(element),
        }
        Ok(())
    }

    fn state(&self) -> &FormatState<Self::Context> {
        self.context
    }

    fn state_mut(&mut self) -> &mut FormatState<Self::Context> {
        &mut self.context
    }
}

#[derive(Clone, Eq, PartialEq)]
pub struct Document(Vec<FormatElement>);

impl Document {
    pub fn into_vec(self) -> Vec<FormatElement> {
        self.0
    }

    pub fn into_element(mut self) -> FormatElement {
        if self.is_empty() {
            FormatElement::Empty
        } else if self.0.len() == 1 {
            self.0.pop().unwrap()
        } else {
            FormatElement::List(List::new(self.0))
        }
    }
}

impl FromIterator<FormatElement> for Document {
    fn from_iter<T: IntoIterator<Item = FormatElement>>(iter: T) -> Self {
        Document(Vec::from_iter(iter))
    }
}

impl fmt::Debug for Document {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_list().entries(&self.0).finish()
    }
}

impl Deref for Document {
    type Target = [FormatElement];

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
