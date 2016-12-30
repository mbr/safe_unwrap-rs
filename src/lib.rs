#![no_std]

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
