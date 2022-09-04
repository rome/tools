/// A school book stack. Allows adding, removing, and inspecting elements at the back.
pub(super) trait Stack<T> {
    /// Removes the last element if any and returns it
    fn pop(&mut self) -> Option<T>;

    /// Pushes a new element at the back
    fn push(&mut self, value: T);

    /// Returns the last element if any
    fn top(&self) -> Option<&T>;

    /// Returns `true` if the stack is empty
    fn is_empty(&self) -> bool;
}

impl<T> Stack<T> for Vec<T> {
    fn pop(&mut self) -> Option<T> {
        self.pop()
    }

    fn push(&mut self, value: T) {
        self.push(value)
    }

    fn top(&self) -> Option<&T> {
        self.last()
    }

    fn is_empty(&self) -> bool {
        self.is_empty()
    }
}

/// A Stack that is stacked on top of another stack. Guarantees that the underlying stack remains unchanged.
#[derive(Debug, Clone)]
pub(super) struct StackedStack<'a, T> {
    /// The content of the original stack.
    original: &'a [T],

    /// The index of the "top" element in the original stack.
    original_top: usize,

    /// Items that have been pushed since the creation of this stack and aren't part of the `original` stack.
    stack: Vec<T>,
}

impl<'a, T> StackedStack<'a, T> {
    #[cfg(test)]
    pub(super) fn new(original: &'a [T]) -> Self {
        Self::with_vec(original, Vec::new())
    }

    /// Creates a new stack that uses `stack` for storing its elements.
    pub(super) fn with_vec(original: &'a [T], stack: Vec<T>) -> Self {
        Self {
            original_top: original.len(),
            original,
            stack,
        }
    }

    /// Returns the underlying `stack` vector.
    pub(super) fn into_vec(self) -> Vec<T> {
        self.stack
    }
}

impl<T> Stack<T> for StackedStack<'_, T>
where
    T: Copy,
{
    fn pop(&mut self) -> Option<T> {
        self.stack.pop().or_else(|| {
            if self.original_top == 0 {
                None
            } else {
                self.original_top -= 1;
                Some(self.original[self.original_top])
            }
        })
    }

    fn push(&mut self, value: T) {
        self.stack.push(value);
    }

    fn top(&self) -> Option<&T> {
        self.stack.last().or_else(|| {
            if self.original_top == 0 {
                None
            } else {
                Some(&self.original[self.original_top - 1])
            }
        })
    }

    fn is_empty(&self) -> bool {
        self.original_top == 0 && self.stack.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use crate::printer::stack::{Stack, StackedStack};

    #[test]
    fn restore_consumed_stack() {
        let original = vec![1, 2, 3];
        let mut restorable = StackedStack::new(&original);

        restorable.push(4);

        assert_eq!(restorable.pop(), Some(4));
        assert_eq!(restorable.pop(), Some(3));
        assert_eq!(restorable.pop(), Some(2));
        assert_eq!(restorable.pop(), Some(1));
        assert_eq!(restorable.pop(), None);

        assert_eq!(original, vec![1, 2, 3]);
    }

    #[test]
    fn restore_partially_consumed_stack() {
        let original = vec![1, 2, 3];
        let mut restorable = StackedStack::new(&original);

        restorable.push(4);

        assert_eq!(restorable.pop(), Some(4));
        assert_eq!(restorable.pop(), Some(3));
        assert_eq!(restorable.pop(), Some(2));
        restorable.push(5);
        restorable.push(6);
        restorable.push(7);

        assert_eq!(original, vec![1, 2, 3]);
    }

    #[test]
    fn restore_stack() {
        let original = vec![1, 2, 3];
        let mut restorable = StackedStack::new(&original);

        restorable.push(4);
        restorable.push(5);
        restorable.push(6);
        restorable.push(7);

        assert_eq!(restorable.pop(), Some(7));
        assert_eq!(restorable.pop(), Some(6));
        assert_eq!(restorable.pop(), Some(5));

        assert_eq!(original, vec![1, 2, 3]);
    }
}
