//! Functions to interact with the environment.

extern "C" {
    fn env_print(ptr: i32, len: i32);
    fn env_now() -> i32;
    fn env_ud() -> i32;
    fn env_var(ptr: i32, len: i32) -> i32;
    fn env_random() -> i32;
}

#[doc(hidden)]
pub fn print_it(s: String) {
    let (ptr, len) = ft_sys::memory::string_to_bytes_ptr(s);
    unsafe { env_print(ptr, len) }
}

/// Print some data to the server log.
///
/// This is similar to `println!` in Rust.
#[macro_export]
macro_rules! println {
    ($($t:tt)*) => {{
        $crate::env::print_it(format!($($t)*))
    }};
}

/// Get the current time in UTC.
pub fn now() -> chrono::DateTime<chrono::Utc> {
    let ptr = unsafe { env_now() };
    ft_sys::memory::json_from_ptr(ptr)
}

/// Get the currently logged in user making the http request. If the user is
/// not logged in this returns `None`
pub fn ud() -> Option<ft_sys::UserData> {
    let ptr = unsafe { env_ud() };
    ft_sys::memory::json_from_ptr(ptr)
}

/// Read an environment variable. If the environment variable is not set, this
/// returns `None`.
pub fn var(name: String) -> Option<String> {
    let (ptr, len) = ft_sys::memory::string_to_bytes_ptr(name);
    let ptr = unsafe { env_var(ptr, len) };
    ft_sys::memory::json_from_ptr(ptr)
}

/// Generate a random number between 0 and 1.
pub fn random() -> f64 {
    let ptr = unsafe { env_random() };
    ft_sys::memory::json_from_ptr(ptr)
}
