pub struct JsonBody {
    pub body: serde_json::Map<String, serde_json::Value>,
}

impl JsonBody {
    pub fn field<T: serde::de::DeserializeOwned>(
        &self,
        field: &str,
    ) -> serde_json::Result<Option<T>> {
        match self.body.get(field) {
            Some(v) => Ok(serde_json::from_value(v.clone())?),
            None => Ok(None),
        }
    }

    fn field_<T: serde::de::DeserializeOwned>(
        &self,
        field: &str,
        errors: &mut std::collections::HashMap<String, String>,
    ) -> Result<T, ()> {
        match self.field(field) {
            Ok(Some(v)) => Ok(v),
            Ok(None) => {
                errors.insert(field.to_string(), "required".to_string());
                Err(())
            }
            Err(e) => {
                errors.insert(field.to_string(), e.to_string());
                Err(())
            }
        }
    }

    pub fn json<T: serde::de::DeserializeOwned>(&self) -> serde_json::Result<T> {
        serde_json::from_value(serde_json::Value::Object(self.body.clone()))
    }
}

pub trait JsonBodyExt {
    fn json_body(&self) -> serde_json::Result<JsonBody>;
    fn json_body_(&self) -> Result<JsonBody, std::collections::HashMap<String, String>>;
    fn required<T: serde::de::DeserializeOwned>(
        &self,
        field: &str,
    ) -> Result<T, std::collections::HashMap<String, String>>;
    fn required2<T1: serde::de::DeserializeOwned, T2: serde::de::DeserializeOwned>(
        &self,
        field1: &str,
        field2: &str,
    ) -> Result<(T1, T2), std::collections::HashMap<String, String>>;
}

impl JsonBodyExt for http::Request<bytes::Bytes> {
    fn json_body_(&self) -> Result<JsonBody, std::collections::HashMap<String, String>> {
        // TODO: check if content type is application/json
        Ok(JsonBody {
            body: match serde_json::from_slice(self.body()) {
                Ok(v) => v,
                Err(e) => {
                    let mut errors = std::collections::HashMap::new();
                    errors.insert("all".to_string(), e.to_string());
                    return Err(errors);
                }
            },
        })
    }

    fn json_body(&self) -> serde_json::Result<JsonBody> {
        // TODO: check if content type is application/json
        Ok(JsonBody {
            body: serde_json::from_slice(self.body())?,
        })
    }

    fn required<T: serde::de::DeserializeOwned>(
        &self,
        field: &str,
    ) -> Result<T, std::collections::HashMap<String, String>> {
        let mut errors = std::collections::HashMap::new();
        let j = self.json_body_()?;
        j.field_(field, &mut errors).map_err(|_| errors)
    }

    fn required2<T1: serde::de::DeserializeOwned, T2: serde::de::DeserializeOwned>(
        &self,
        field1: &str,
        field2: &str,
    ) -> Result<(T1, T2), std::collections::HashMap<String, String>> {
        let j = self.json_body_()?;

        let mut errors = std::collections::HashMap::new();
        match (j.field_(field1, &mut errors), j.field_(field2, &mut errors)) {
            (Ok(v1), Ok(v2)) => Ok((v1, v2)),
            _ => Err(errors),
        }
    }
}
