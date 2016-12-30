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
//! Does not require `std`.

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


#[cfg(test)]
mod tests {
    #[test]
    fn works_when_ok() {
        let x = safe_unwrap!("this comment is meaningless", Some(42));
        assert_eq!(x, 42);
    }

    #[test]
    #[should_panic]
    fn doesnt_work_when_err() {
        let x: Option<()> = safe_unwrap!("should fail", None);

    }
}
