#[macro_use]
extern crate safe_unwrap;

fn main() {
    let inv: Option<()> = None;
    safe_unwrap!("inv will always have some value", inv);
}
