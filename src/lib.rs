#![no_std]

//! Annotate unwraps as manually checked
//!
//! The `safe_unwrap` macros allows unwrapping and annotating that the unwrap
//! will never fail.
//!
//! An example:
//!
//! ```
//! #[macro_use]
//! extern crate safe_unwrap;
//!
//! fn main() {
//!    let res = Some(42);
//!
//!    // we know that unwrapping res will never fail, so it is safe to call unwrap
//!    let val = safe_unwrap!("is constant value", res);
//!
//!    assert_eq!(val, 42);
//! }
//! ```
//!
//! In release builds, `safe_unwrap!(expr)` is equivalent to `expr.unwrap()`;
//! in debug builds, `expect()` will be called with a message indicating that
//! the assumed invariant has been violated.
//!
//! Alternative, for `Result` and `Option` types, you can risk a small bit of
//! overhead in exchange for nicer syntax:
//!
//! ```
//! extern crate safe_unwrap;
//! use safe_unwrap::SafeUnwrap;
//!
//! fn main() {
//!     let res = Some(42);
//!
//!     // works only for Result and Option types
//!     let val = res.safe_unwrap("is constant value");
//!
//!     assert_eq!(val, 42);
//!
//!     #[cfg(feature = "std")]
//!     {
//!         // With `std`, two additional methods are available.
//!         let val = res.unwrap_or_abort("is constant value");
//!         assert_eq!(val, 42);
//!         let val = res.unwrap_or_exit("is constant value");
//!         assert_eq!(val, 42);
//!     }
//! }
//! ```
//!
//! The semantics of `.safe_unwrap` are otherwise the same as the
//! `safe_unwrap!` macro. The tradeoff here is that you are at the mercy of the
//! LLVM optimizer to remove the unused static string `"is constant value"`
//! from the resulting executable (often works in release mode).
//!
//!
//! ## `std` support
//!
//! By default, `no_std` is supported. With the `std` feature, `SafeUnwrap` has
//! two additional methods, which require the standard library. They work the
//! same way as `safe_unwrap`, but:
//!
//! * `unwrap_or_abort` aborts the process instead of panicking.
//! * `unwrap_or_exit` exits with code 1 instead of panicking.

#[cfg(feature = "std")]
extern crate std;

#[cfg(feature = "std")]
use std::io::Write;

// TODO: replace `cfg(debug_assertions)` with something cleaner using a build
//       script
#[macro_export]
#[cfg(not(debug_assertions))]
macro_rules! safe_unwrap {
    ($reason:expr, $e:expr) => ($e.unwrap())
}

#[macro_export]
#[cfg(debug_assertions)]
macro_rules! safe_unwrap {
    ($reason:expr, $e:expr) => (
        $e.expect(concat!("[BUG] violated: ",
        $reason))
    )
}

pub trait SafeUnwrap<T> {
    fn safe_unwrap(self, msg: &'static str) -> T;
    #[cfg(feature = "std")]
    fn unwrap_or_abort(self, msg: &'static str) -> T;
    #[cfg(feature = "std")]
    fn unwrap_or_exit(self, msg: &'static str) -> T;
}

#[cfg(not(debug_assertions))]
impl<T, E: core::fmt::Debug> SafeUnwrap<T> for Result<T, E> {
    #[inline]
    fn safe_unwrap(self, _: &'static str) -> T {
        self.unwrap()
    }

    #[cfg(feature = "std")]
    #[inline]
    fn unwrap_or_abort(self, _: &'static str) -> T {
        self.unwrap_or_else(|_| std::process::abort())
    }

    #[cfg(feature = "std")]
    #[inline]
    fn unwrap_or_exit(self, _: &'static str) -> T {
        self.unwrap_or_else(|_| std::process::exit(1))
    }
}

#[cfg(not(debug_assertions))]
impl<T> SafeUnwrap<T> for Option<T> {
    #[inline]
    fn safe_unwrap(self, _: &'static str) -> T {
        self.unwrap()
    }

    #[cfg(feature = "std")]
    #[inline]
    fn unwrap_or_abort(self, _: &'static str) -> T {
        self.unwrap_or_else(std::process::abort)
    }

    #[cfg(feature = "std")]
    #[inline]
    fn unwrap_or_exit(self, _: &'static str) -> T {
        self.unwrap_or_else(|| std::process::exit(1))
    }
}

#[cfg(debug_assertions)]
impl<T, E: core::fmt::Debug> SafeUnwrap<T> for Result<T, E> {
    #[inline]
    fn safe_unwrap(self, msg: &'static str) -> T {
        self.expect(msg)
    }

    #[cfg(feature = "std")]
    #[inline]
    fn unwrap_or_abort(self, msg: &'static str) -> T {
        self.unwrap_or_else(|_| {
            let _ = writeln!(std::io::stderr(), "{}", msg);
            std::process::abort()
        })
    }

    #[cfg(feature = "std")]
    #[inline]
    fn unwrap_or_exit(self, msg: &'static str) -> T {
        self.unwrap_or_else(|_| {
            let _ = writeln!(std::io::stderr(), "{}", msg);
            std::process::exit(1)
        })
    }
}

#[cfg(debug_assertions)]
impl<T> SafeUnwrap<T> for Option<T> {
    #[inline]
    fn safe_unwrap(self, msg: &'static str) -> T {
        self.expect(msg)
    }

    #[cfg(feature = "std")]
    #[inline]
    fn unwrap_or_abort(self, msg: &'static str) -> T {
        self.unwrap_or_else(|| {
            let _ = writeln!(std::io::stderr(), "{}", msg);
            std::process::abort()
        })
    }

    #[cfg(feature = "std")]
    #[inline]
    fn unwrap_or_exit(self, msg: &'static str) -> T {
        self.unwrap_or_else(|| {
            let _ = writeln!(std::io::stderr(), "{}", msg);
            std::process::exit(1)
        })
    }
}

#[cfg(test)]
mod tests {
    use super::SafeUnwrap;

    #[test]
    fn works_when_ok() {
        let x = safe_unwrap!("this comment is meaningless", Some(42));
        assert_eq!(x, 42);
    }

    #[test]
    #[should_panic]
    fn doesnt_work_when_err() {
        let _: Option<()> = safe_unwrap!("should fail", None);
    }

    #[test]
    fn trait_works_when_ok() {
        let x = Some(42).safe_unwrap("meaningless comment");
        assert_eq!(x, 42);

        let r: Result<usize, ()> = Ok(42);
        let y = r.safe_unwrap("meaningless comment");
        assert_eq!(y, 42);
    }

    #[test]
    #[should_panic]
    fn trait_doesnt_work_when_err() {
        let _: Option<()> = None.safe_unwrap("should fail");
        let _: Result<(), ()> = Err(()).safe_unwrap("should fail");
    }

}
