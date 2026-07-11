#[cfg(unix)]
pub fn current_uid() -> u32 {
    unsafe extern "C" {
        fn getuid() -> u32;
    }
    // getuid is a read-only libc query and has no ownership or allocation obligations.
    unsafe { getuid() }
}
