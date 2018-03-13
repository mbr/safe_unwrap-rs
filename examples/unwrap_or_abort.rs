extern crate safe_unwrap;

use safe_unwrap::SafeUnwrap;

fn main() {
    let foo: Option<usize> = None;
    foo.unwrap_or_abort("a reason why this should never fail can be specified here");
}
