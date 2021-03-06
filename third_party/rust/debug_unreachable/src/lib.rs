#![deny(missing_docs, warnings)]

//! `panic!()` in debug builds, optimization hint in release.

extern crate unreachable;

#[doc(hidden)]
pub use unreachable::unreachable as __unreachable;

#[macro_export]
/// `panic!()` in debug builds, optimization hint in release.
macro_rules! debug_unreachable {
    () => { debug_unreachable!("entered unreachable code") };
    ($e:expr) => {
        if cfg!(ndebug) {
            $crate::__unreachable()
        } else {
            panic!($e);
        }
    }
}

