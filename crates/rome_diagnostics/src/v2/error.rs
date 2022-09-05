use std::{
    fmt::{Debug, Formatter},
    io,
    marker::PhantomData,
    ptr::NonNull,
};

use rome_console::fmt;

use self::internal::AsDiagnostic;
use super::{Category, Diagnostic, DiagnosticTags, Location, Severity, Visitor};

/// The `Error` struct wraps any type implementing [Diagnostic] into a single
/// dynamic type
pub struct Error {
    inner: NonNull<ErrorImpl>,
}

/// Implement the [Diagnostic] trait as inherent methods on the [Error] type
impl Error {
    /// Calls [Diagnostic::category] on the [Diagnostic] wrapped by this [Error]
    pub fn category(&self) -> Option<&Category> {
        self.as_diagnostic().category()
    }

    /// Calls [Diagnostic::severity] on the [Diagnostic] wrapped by this [Error]
    pub fn severity(&self) -> Severity {
        self.as_diagnostic().severity()
    }

    /// Calls [Diagnostic::description] on the [Diagnostic] wrapped by this [Error]
    pub fn description(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.as_diagnostic().description(fmt)
    }

    /// Calls [Diagnostic::message] on the [Diagnostic] wrapped by this [Error]
    pub fn message(&self, fmt: &mut fmt::Formatter<'_>) -> io::Result<()> {
        self.as_diagnostic().message(fmt)
    }

    /// Calls [Diagnostic::advices] on the [Diagnostic] wrapped by this [Error]
    pub fn advices(&self, visitor: &mut dyn Visitor) -> io::Result<()> {
        self.as_diagnostic().advices(visitor)
    }

    /// Calls [Diagnostic::verbose_advices] on the [Diagnostic] wrapped by this [Error]
    pub fn verbose_advices(&self, visitor: &mut dyn Visitor) -> io::Result<()> {
        self.as_diagnostic().verbose_advices(visitor)
    }

    /// Calls [Diagnostic::location] on the [Diagnostic] wrapped by this [Error]
    pub fn location(&self) -> Option<Location<'_>> {
        self.as_diagnostic().location()
    }

    /// Calls [Diagnostic::tags] on the [Diagnostic] wrapped by this [Error]
    pub fn tags(&self) -> DiagnosticTags {
        self.as_diagnostic().tags()
    }

    /// Calls [Diagnostic::source] on the [Diagnostic] wrapped by this [Error]
    pub fn source(&self) -> Option<&dyn Diagnostic> {
        self.as_diagnostic().source()
    }
}

impl Error {
    /// Returns the vtable for an error instance
    fn vtable(&self) -> &'static ErrorVTable {
        unsafe { self.inner.as_ref().vtable }
    }

    /// Returns a [Ref] to the wrapped error
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
/// and outlives the `'static` lifetime
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
/// manually implement dynamic dispatch
struct ErrorVTable {
    as_diagnostic: for<'a> fn(Ref<'a, ErrorImpl>) -> &'a (dyn Diagnostic + 'static),
    drop: fn(NonNull<ErrorImpl>),
}

/// Internal storage struct used to pack a vtable instance along with a
/// diagnostic object
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
    /// using [Box] and returns the corresponding raw pointer
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
/// reference and allow it to be cast while retaining lifetime informations
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
        unsafe { self.ptr.as_ref() }
    }
}

/// Generic implementation of `as_diagnostic` for the `Error` type
fn as_diagnostic<D: Diagnostic + 'static>(
    this: Ref<'_, ErrorImpl>,
) -> &'_ (dyn Diagnostic + 'static) {
    let this = this.downcast::<ErrorImpl<D>>();
    let this = this.into_ref();
    &this.diag
}

/// Generic implementation of `drop` for the `Error` type
fn drop_error<D: Diagnostic>(this: NonNull<ErrorImpl>) {
    let this = this.cast::<ErrorImpl<D>>();
    unsafe {
        Box::from_raw(this.as_ptr());
    }
}

/// Alias of [std::result::Result] with the `Err` type defaulting to [Error]
pub type Result<T, E = Error> = std::result::Result<T, E>;

pub(crate) mod internal {
    //! The `AsDiagnostic` trait needs to be declared as public as its referred
    //! to in the `where` clause of other public items, but as it's not part of
    //! the public API it's declared in a private module so it's not accessible
    //! outside of the crate

    use std::fmt::Debug;

    use crate::v2::diagnostic::*;

    use super::Error;

    /// Since [Error] must implement `From<T: Diagnostic>` to be used with the
    /// `?` operator, it cannot implement the [Diagnostic] trait (as that would
    /// conflict with the implementation of `From<T> for T` in the standard
    /// library). The [AsDiagnostic] exists as an internal implementation
    /// detail to bridge this gap and allow various types and functions in
    /// `rome_diagnostics` to be generic over all diagnostics + `Error`
    pub trait AsDiagnostic: Debug {
        type Diagnostic: Diagnostic + ?Sized;
        fn as_diagnostic(&self) -> &Self::Diagnostic;
        fn as_dyn(&self) -> &dyn Diagnostic;
    }

    impl<D: Diagnostic> AsDiagnostic for D {
        type Diagnostic = D;

        fn as_diagnostic(&self) -> &Self::Diagnostic {
            self
        }

        fn as_dyn(&self) -> &dyn Diagnostic {
            self
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
}

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
