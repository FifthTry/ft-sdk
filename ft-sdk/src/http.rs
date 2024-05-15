pub use ft_sys::http::{current_request, send, send_response};

#[derive(Debug)]
pub enum Output {
    Http(http::Response<bytes::Bytes>),
    Reload,
    Redirect(String),
    Data(ft_sdk::FrontendData),
    Json(serde_json::Value),
}

impl From<Output> for ::std::result::Result<http::Response<bytes::Bytes>, ft_sdk::Error> {
    fn from(o: Output) -> Self {
        match o {
            Output::Http(r) => Ok(r),
            Output::Reload => json_(serde_json::json!({"reload": true})),
            Output::Redirect(url) => json_(serde_json::json!({"redirect": url })),
            Output::Data(data) => json_(serde_json::json!({"data": data })),
            Output::Json(j) => json_(j),
        }
    }
}

// pub trait IntoCookie {
//     fn into_cookie(self) -> http::HeaderValue;
// }
//
// impl IntoCookie for http::HeaderValue {
//     fn into_cookie(self) -> http::HeaderValue {
//         self
//     }
// }
//
// impl<K: AsRef<str>, V: AsRef<str>> IntoCookie for (K, V) {
//     fn into_cookie(self) -> http::HeaderValue {
//         let (k, v) = self;
//         format!(
//             "{}={}; Secure; HttpOnly; SameSite=Strict; Max-Age=34560000",
//             k.as_ref(),
//             v.as_ref()
//         )
//         .parse()
//         .unwrap()
//     }
// }
//
// impl<K: AsRef<str>, V: AsRef<str>> IntoCookie for (K, V, i32) {
//     fn into_cookie(self) -> http::HeaderValue {
//         let (k, v, max_age) = self;
//         format!(
//             "{}={}; Secure; HttpOnly; SameSite=Strict; Max-Age={max_age}",
//             k.as_ref(),
//             v.as_ref()
//         )
//         .parse()
//         .unwrap()
//     }
// }
//
// impl Output {
//     pub fn with_cookie<C: IntoCookie>(self, c: C) -> Self {
//         let mut r: http::Response<bytes::Bytes> = self.into();
//         let cookie: http::HeaderValue = c.into_cookie();
//
//         match r.headers_mut().entry(http::header::SET_COOKIE) {
//             http::header::Entry::Vacant(entry) => {
//                 entry.insert(cookie);
//             }
//             http::header::Entry::Occupied(mut entry) => {
//                 entry.append(cookie);
//             }
//         };
//
//         Output::Http(r)
//     }
//
//     pub fn with_header<K: http::header::IntoHeaderName>(
//         self,
//         key: K,
//         value: http::HeaderValue,
//     ) -> Self {
//         let mut r: http::Response<bytes::Bytes> = self.into();
//         r.headers_mut().insert(key, value);
//         Output::Http(r)
//     }
// }

// #[cfg(test)]
// mod test {
//     #[test]
//     fn header() {
//         let r: http::Response<bytes::Bytes> = super::Output::Reload
//             .with_header(
//                 http::header::CONTENT_TYPE,
//                 http::HeaderValue::from_static("text/html"),
//             )
//             .into();
//
//         assert_eq!(
//             r.headers().get(http::header::CONTENT_TYPE),
//             Some(&http::HeaderValue::from_static("text/html"))
//         );
//
//         let r: http::Response<bytes::Bytes> = super::Output::Reload
//             .with_header("content-type", http::HeaderValue::from_static("text/html"))
//             .into();
//
//         assert_eq!(
//             r.headers().get(http::header::CONTENT_TYPE),
//             Some(&http::HeaderValue::from_static("text/html"))
//         );
//     }
//
//     #[test]
//     fn cookie() {
//         let r: http::Response<bytes::Bytes> =
//             super::Output::Reload.with_cookie(("name", "value")).into();
//
//         let cookies = r.headers().get_all(http::header::SET_COOKIE);
//         let mut iter = cookies.iter();
//         assert_eq!(
//             iter.next(),
//             Some(&http::HeaderValue::from_static(
//                 "name=value; Secure; HttpOnly; SameSite=Strict; Max-Age=34560000"
//             ))
//         );
//         assert_eq!(iter.next(), None);
//
//         let r: http::Response<bytes::Bytes> = super::Output::Reload
//             .with_cookie(("name", "value", 200))
//             .into();
//
//         let cookies = r.headers().get_all(http::header::SET_COOKIE);
//         let mut iter = cookies.iter();
//         assert_eq!(
//             iter.next(),
//             Some(&http::HeaderValue::from_static(
//                 "name=value; Secure; HttpOnly; SameSite=Strict; Max-Age=200"
//             ))
//         );
//         assert_eq!(iter.next(), None);
//
//         let r: http::Response<bytes::Bytes> = super::Output::Reload
//             .with_cookie(("name", "value"))
//             .with_cookie(("n2", "v2"))
//             .into();
//
//         let cookies = r.headers().get_all(http::header::SET_COOKIE);
//         let mut iter = cookies.iter();
//         assert_eq!(
//             iter.next(),
//             Some(&http::HeaderValue::from_static(
//                 "name=value; Secure; HttpOnly; SameSite=Strict; Max-Age=34560000"
//             ))
//         );
//         assert_eq!(
//             iter.next(),
//             Some(&http::HeaderValue::from_static(
//                 "n2=v2; Secure; HttpOnly; SameSite=Strict; Max-Age=34560000"
//             ))
//         );
//     }
//
//     #[test]
//     fn raw_cookie() {
//         let r: http::Response<bytes::Bytes> = super::Output::Reload
//             .with_cookie(http::HeaderValue::from_static("hello"))
//             .into();
//
//         let cookies = r.headers().get_all(http::header::SET_COOKIE);
//         let mut iter = cookies.iter();
//         assert_eq!(iter.next(), Some(&http::HeaderValue::from_static("hello")));
//         assert_eq!(iter.next(), None);
//
//         let r: http::Response<bytes::Bytes> = super::Output::Reload
//             .with_cookie(("name", "value"))
//             .with_cookie(http::HeaderValue::from_static("hello"))
//             .into();
//
//         let cookies = r.headers().get_all(http::header::SET_COOKIE);
//         let mut iter = cookies.iter();
//         assert_eq!(
//             iter.next(),
//             Some(&http::HeaderValue::from_static(
//                 "name=value; Secure; HttpOnly; SameSite=Strict; Max-Age=34560000"
//             ))
//         );
//         assert_eq!(iter.next(), Some(&http::HeaderValue::from_static("hello")));
//     }
// }

pub type Result = std::result::Result<Output, ft_sdk::Error>;

#[derive(Debug, thiserror::Error)]
pub enum JsonError {
    #[error("serde_json error {0}")]
    Serde(#[from] serde_json::Error),
    #[error("http error {0}")]
    Http(#[from] http::Error),
}

pub(crate) fn json_<T: serde::Serialize>(
    t: T,
) -> std::result::Result<http::Response<bytes::Bytes>, ft_sdk::Error> {
    let d = match serde_json::to_string(&t) {
        Ok(d) => d,
        Err(e) => return Err(ft_sdk::server_error!("json error: {e:?}")),
    };

    Ok(http::Response::builder()
        .status(200)
        .header("Content-Type", "application/json")
        .body(d.into())?)
}

/// Create a page not found response.
#[macro_export]
macro_rules! not_found {
    ($($t:tt)*) => {{
        let msg = format!($($t)*);
        $crate::http::not_found_(msg)
    }};
}

#[doc(hidden)]
pub fn not_found_(msg: String) -> ft_sdk::Error {
    anyhow::format_err!("not-found: {msg}").context(http::StatusCode::NOT_FOUND)
}

/// Create a server error response.
#[macro_export]
macro_rules! server_error {
    ($($t:tt)*) => {{
        let msg = format!($($t)*);
        ft_sdk::println!("server-error: {msg}");
        anyhow::format_err!("server-error: {msg}").context(http::StatusCode::INTERNAL_SERVER_ERROR)
    }};
}
