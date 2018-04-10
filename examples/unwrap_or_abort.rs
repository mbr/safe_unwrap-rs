extern crate safe_unwrap;

#[cfg(feature = "std")]
use safe_unwrap::SafeUnwrap;

#[cfg(feature = "std")]
fn main() {
    let foo: Option<usize> = None;
    foo.unwrap_or_abort("a reason why this should never fail can be specified here");
}

#[cfg(not(feature = "std"))]
fn main() {}
