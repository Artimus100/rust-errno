//! Cross-platform interface to the `errno` variable.

#[cfg(unix)] extern crate libc;
#[cfg(windows)] extern crate winapi;
#[cfg(windows)] extern crate kernel32;

#[cfg(unix)] mod unix;
#[cfg(unix)] pub use unix::{Errno, errno, set_errno};

#[cfg(windows)] mod windows;
#[cfg(windows)] pub use windows::{Errno, errno, set_errno};

#[test]
fn it_works() {
    let x = errno();
    set_errno(x);
    let _ = x.to_string();
}

#[test]
fn check_description() {
    let expect = if cfg!(windows) {
        "Incorrect function."
    } else {
        "Operation not permitted"
    };

    set_errno(Errno(1));

    assert_eq!(errno().to_string(), expect);
}
