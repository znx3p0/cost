extern {
    fn start_profile() -> u64;
    fn end_profile(fd: u64) -> u64;
}

/// get the cost of running a function
pub fn cost<T>(f: impl Fn() -> T) -> (u64, T) {
    unsafe {
        __inner_profile(f)
    }
}

unsafe fn __inner_profile<T>(f: impl Fn() -> T) -> (u64, T) {
    let fd = start_profile();
    let res = f();
    (end_profile(fd), res)
}

