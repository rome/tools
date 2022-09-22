//! The `error` module contains the implementation of [Error], a dynamic
//! container struct for any type implementing [Diagnostic].
//!
//! The `Error` struct is essentially a manual implementation of
//! `Box<dyn Diagnostic>` that is only a single word in size (thin pointer)
//! instead of two for Rust's `dyn` types (wide pointer). This is done by
//! manually managing how the "vtable" (the function pointer table used for
//! dynamic dispatch) for the type is laid out and accessed in memory, at the
//! cost of requiring the use of unsafe code.
//!
//! While reducing the size of `Error` is the main reason this type is using
//! unsafe code (as it makes returning a `Result<T, Error>` more efficient),
//! manually managing the vtable for `Error` opens additional possibilities for
//! future extensions to the type like requiring disjoint trait bounds or
//! implementing dynamic dispatch to traits that require handling owned
//! instances of an object (the main use case for this would be implementing
//! `Clone` for `Error` since `dyn Clone` is not allowed in Rust)

use std::{
    fmt::{Debug, Formatter},
    io,
    marker::PhantomData,
    ptr::NonNull,
};

use rome_console::fmt;

use super::{
    diagnostic::internal::AsDiagnostic, Category, Diagnostic, DiagnosticTags, Location, Severity,
    Visit,
};

/// The `Error` struct wraps any type implementing [Diagnostic] into a single
/// dynamic type.
pub struct Error {
    inner: NonNull<ErrorImpl>,
}

/// Implement the [Diagnostic] trait as inherent methods on the [Error] type.
impl Error {
    /// Calls [Diagnostic::category] on the [Diagnostic] wrapped by this [Error].
    pub fn category(&self) -> Option<&Category> {
        self.as_diagnostic().category()
    }

    /// Calls [Diagnostic::severity] on the [Diagnostic] wrapped by this [Error].
    pub fn severity(&self) -> Severity {
        self.as_diagnostic().severity()
    }

    /// Calls [Diagnostic::description] on the [Diagnostic] wrapped by this [Error].
    pub fn description(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.as_diagnostic().description(fmt)
    }

    /// Calls [Diagnostic::message] on the [Diagnostic] wrapped by this [Error].
    pub fn message(&self, fmt: &mut fmt::Formatter<'_>) -> io::Result<()> {
        self.as_diagnostic().message(fmt)
    }

    /// Calls [Diagnostic::advices] on the [Diagnostic] wrapped by this [Error].
    pub fn advices(&self, visitor: &mut dyn Visit) -> io::Result<()> {
        self.as_diagnostic().advices(visitor)
    }

    /// Calls [Diagnostic::verbose_advices] on the [Diagnostic] wrapped by this [Error].
    pub fn verbose_advices(&self, visitor: &mut dyn Visit) -> io::Result<()> {
        self.as_diagnostic().verbose_advices(visitor)
    }

    /// Calls [Diagnostic::location] on the [Diagnostic] wrapped by this [Error].
    pub fn location(&self) -> Option<Location<'_>> {
        self.as_diagnostic().location()
    }

    /// Calls [Diagnostic::tags] on the [Diagnostic] wrapped by this [Error].
    pub fn tags(&self) -> DiagnosticTags {
        self.as_diagnostic().tags()
    }

    /// Calls [Diagnostic::source] on the [Diagnostic] wrapped by this [Error].
    pub fn source(&self) -> Option<&dyn Diagnostic> {
        self.as_diagnostic().source()
    }
}

impl Error {
    /// Returns the vtable for an error instance.
    fn vtable(&self) -> &'static ErrorVTable {
        // SAFETY: This assumes `inner` is a valid pointer to an `ErrorImpl`
        unsafe { self.inner.as_ref().vtable }
    }

    /// Returns a [Ref] to the wrapped error.
    fn inner(&'_ self) -> Ref<'_, ErrorImpl> {
        Ref {
            ptr: self.inner,
            _lt: PhantomData,
        }
    }
}

// SAFETY: `ErrorImpl::new` requires `Send + Sync`, and `inner` cannot be
// cloned or moved out of `Error`
unsafe impl Send for Error {}
unsafe impl Sync for Error {}

/// Implement [From] for all types implementing [Diagnostic], [Send], [Sync]
/// and outlives the `'static` lifetime.
impl<T> From<T> for Error
where
    T: Diagnostic + Send + Sync + 'static,
{
    fn from(diag: T) -> Self {
        Self {
            inner: ErrorImpl::new(diag),
        }
    }
}

impl AsDiagnostic for Error {
    type Diagnostic = dyn Diagnostic;

    fn as_diagnostic(&self) -> &Self::Diagnostic {
        (self.vtable().as_diagnostic)(self.inner())
    }

    fn as_dyn(&self) -> &dyn Diagnostic {
        self.as_diagnostic()
    }
}

// Defer the implementation of `Debug` and `Drop` to the wrapped type
impl Debug for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Debug::fmt(self.as_diagnostic(), f)
    }
}

impl Drop for Error {
    fn drop(&mut self) {
        (self.vtable().drop)(self.inner);
    }
}

/// VTable struct for the [Error] type, contains function pointers used to
/// manually implement dynamic dispatch.
struct ErrorVTable {
    as_diagnostic: for<'a> fn(Ref<'a, ErrorImpl>) -> &'a (dyn Diagnostic + 'static),
    drop: fn(NonNull<ErrorImpl>),
}

/// Internal storage struct used to pack a vtable instance along with a
/// diagnostic object.
struct ErrorImpl<D = ()> {
    vtable: &'static ErrorVTable,
    diag: D,
}

impl<D> ErrorImpl<D>
where
    D: Diagnostic + Send + Sync + 'static,
{
    /// Create a new instance of [ErrorImpl] containing the provided diagnostic
    /// and a reference to the corresponding vtable, allocate it on the heap
    /// using [Box] and returns the corresponding raw pointer.
    fn new(diag: D) -> NonNull<ErrorImpl> {
        let vtable = &ErrorVTable {
            as_diagnostic: as_diagnostic::<D>,
            drop: drop_error::<D>,
        };

        let inner = Box::new(Self { vtable, diag });
        // SAFETY: The pointer returned by `Box::into_raw` is guaranteed to be
        // valid, so calling `NonNull::new_unchecked` on it is safe
        unsafe { NonNull::new_unchecked(Box::into_raw(inner)).cast() }
    }
}

/// This type is used to manually represent a reference as a raw pointer + an
/// associated lifetime, in order to decouple the underlying type of the
/// reference and allow it to be cast while retaining lifetime informations.
#[derive(Copy, Clone)]
struct Ref<'a, T> {
    ptr: NonNull<T>,
    _lt: PhantomData<&'a T>,
}

impl<'a, T> Ref<'a, T> {
    // Cast this reference to a different type
    fn downcast<R>(self) -> Ref<'a, R> {
        Ref {
            ptr: self.ptr.cast(),
            _lt: PhantomData,
        }
    }

    // Returns a reference to the value pointed to by this reference
    fn into_ref(self) -> &'a T {
        // SAFETY: This assumes `ptr` is a valid pointer to `T`
        unsafe { self.ptr.as_ref() }
    }
}

/// Generic implementation of `as_diagnostic` for the `Error` type.
fn as_diagnostic<D: Diagnostic + 'static>(
    this: Ref<'_, ErrorImpl>,
) -> &'_ (dyn Diagnostic + 'static) {
    let this = this.downcast::<ErrorImpl<D>>();
    let this = this.into_ref();
    &this.diag
}

/// Generic implementation of `drop` for the `Error` type.
fn drop_error<D: Diagnostic>(this: NonNull<ErrorImpl>) {
    let this = this.cast::<ErrorImpl<D>>();
    // SAFETY: This assumes `this` is a valid pointer to an `ErrorImpl<D>` that
    // was allocated with `Box::new`
    unsafe {
        drop(Box::from_raw(this.as_ptr()));
    }
}

/// Alias of [std::result::Result] with the `Err` type defaulting to [Error].
pub type Result<T, E = Error> = std::result::Result<T, E>;

#[cfg(test)]
mod tests {
    use std::{
        mem::size_of,
        sync::{
            atomic::{AtomicBool, Ordering},
            Arc,
        },
    };

    use crate::v2::{Diagnostic, Error, Result};

    #[derive(Debug)]
    struct TestDiagnostic(Arc<AtomicBool>);

    impl Diagnostic for TestDiagnostic {}

    impl Drop for TestDiagnostic {
        fn drop(&mut self) {
            let was_dropped = self.0.swap(true, Ordering::Relaxed);
            assert!(!was_dropped);
        }
    }

    #[test]
    fn test_drop() {
        let drop_flag = AtomicBool::new(false);
        let drop_flag = Arc::new(drop_flag);

        let diag = TestDiagnostic(drop_flag.clone());

        let error = Error::from(diag);
        drop(error);

        assert!(drop_flag.load(Ordering::Relaxed));
    }

    #[test]
    fn test_error_size() {
        assert_eq!(size_of::<Error>(), size_of::<usize>());
    }

    #[test]
    fn test_result_size() {
        assert_eq!(size_of::<Result<()>>(), size_of::<usize>());
    }
}
