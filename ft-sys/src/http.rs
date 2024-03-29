//! Functions and types to work with HTTP requests and responses.
#[derive(Debug)]
pub enum Error {}

pub fn get(_r: http::Request<bytes::Bytes>) -> Result<http::Response<bytes::Bytes>, Error> {
    todo!()
}

pub fn post(_r: http::Request<bytes::Bytes>) -> Result<http::Response<bytes::Bytes>, Error> {
    todo!()
}

/// Get the current request.
pub fn current_request() -> http::Request<bytes::Bytes> {
    extern "C" {
        fn http_get_request() -> i32;
    }
    ft_sys::println!("current_request");
    let ptr = unsafe { http_get_request() };
    ft_sys::println!("current_request: {ptr}");
    let r: ft_sys_shared::Request = ft_sys::memory::json_from_ptr(ptr);
    r.into()
}

/// Send a http response to client.
///
/// This function must not be called more than once.
pub fn send_response(r: http::Response<bytes::Bytes>) {
    extern "C" {
        fn http_send_response(ptr: i32, len: i32);
    }
    let r: ft_sys_shared::Request = r.into();
    let (ptr, len) = ft_sys::memory::json_ptr(r);
    unsafe { http_send_response(ptr, len) }
}
