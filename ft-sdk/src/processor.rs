pub type Result = std::result::Result<ft_sdk::chr::CHR<Output>, ft_sdk::Error>;

#[derive(Debug)]
pub enum Output {
    Redirect(String),
    Json(serde_json::Value),
}

impl Output {
    pub fn map_json<F>(self, f: F) -> Output
    where
        F: FnOnce(serde_json::Value) -> serde_json::Value,
    {
        match self {
            Output::Redirect(url) => Output::Redirect(url),
            Output::Json(j) => Output::Json(f(j)),
        }
    }
}

impl From<ft_sdk::chr::CHR<Output>>
    for std::result::Result<http::Response<bytes::Bytes>, ft_sdk::Error>
{
    fn from(
        ft_sdk::chr::CHR {
            cookies,
            headers,
            response,
        }: ft_sdk::chr::CHR<Output>,
    ) -> Self {
        let response = match response {
            Output::Redirect(url) => crate::json(serde_json::json!({"redirect": url })),
            Output::Json(j) => crate::json(j),
        }?;
        ft_sdk::chr::chr(cookies, headers, response)
    }
}

pub fn json<T: serde::Serialize>(j: T) -> Result {
    Ok(ft_sdk::chr::CHR::new(Output::Json(serde_json::to_value(
        j,
    )?)))
}

pub fn redirect<S: AsRef<str>>(url: S) -> Result {
    Ok(ft_sdk::chr::CHR::new(Output::Redirect(
        url.as_ref().to_string(),
    )))
}
