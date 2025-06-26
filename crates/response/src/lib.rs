pub mod category;
pub mod error;

pub use category::Category as ResponseCategory;

#[derive(Debug, Clone)]
#[derive(serde::Serialize)]
pub struct SNAPResponse<T>
where
    T: serde::Serialize + serde::de::DeserializeOwned,
{
    #[serde(flatten)]
    response_error: Option<crate::error::ErrorResponse>,
    #[serde(flatten)]
    response_success: Option<T>,
}

impl<const SERVICE_CODE: u8> From<crate::error::Error<SERVICE_CODE>> for SNAPResponse<()> {
    fn from(error: crate::error::Error<SERVICE_CODE>) -> Self {
        SNAPResponse {
            response_error: Some(error.into()),
            response_success: None,
        }
    }
}

impl<'de, T> serde::Deserialize<'de> for SNAPResponse<T>
where
    T: serde::Serialize + serde::de::DeserializeOwned,
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct SNAPResponseVisitor<T>
        where
            T: serde::Serialize + serde::de::DeserializeOwned,
        {
            marker: std::marker::PhantomData<T>,
        }

        impl<'de, T> serde::de::Visitor<'de> for SNAPResponseVisitor<T>
        where
            T: serde::Serialize + serde::de::DeserializeOwned,
        {
            type Value = SNAPResponse<T>;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("a SNAPResponse struct")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: serde::de::MapAccess<'de>,
            {
                // We'll try to deserialize both error and success, but only one should be
                // present
                let mut error: Option<crate::error::ErrorResponse> = None;
                let mut success: Option<T> = None;

                // Collect all key-value pairs into a serde_json::Map
                let mut value = serde_json::Map::new();

                while let Some((key, val)) = map.next_entry::<String, serde_json::Value>()? {
                    value.insert(key, val);
                }

                // Try "error" first
                match serde_json::from_value::<crate::error::ErrorResponse>(serde_json::Value::Object(
                    value.clone(),
                )) {
                    Ok(e) => {
                        error = Some(e);
                    }
                    Err(_) => {
                        // Try "success"
                        match serde_json::from_value::<T>(serde_json::Value::Object(value)) {
                            Ok(s) => {
                                success = Some(s);
                            }
                            Err(_) => {
                                // Neither "error" nor "success" matched
                                return Err(serde::de::Error::custom(
                                    "Invalid SNAPResponse: neither error nor success could be deserialized",
                                ));
                            }
                        }
                    }
                }

                Ok(SNAPResponse {
                    response_error: error,
                    response_success: success,
                })
            }
        }

        deserializer.deserialize_map(SNAPResponseVisitor {
            marker: std::marker::PhantomData,
        })
    }
}

impl<T> actix_web::Responder for SNAPResponse<T>
where
    T: serde::Serialize + serde::de::DeserializeOwned,
{
    type Body = actix_web::body::BoxBody;

    fn respond_to(self, _: &actix_web::HttpRequest) -> actix_web::HttpResponse<Self::Body> {
        let status_code = if let Some(error_ref) = &self.response_error {
            error_ref.http_code
        } else {
            http::StatusCode::OK
        };

        actix_web::HttpResponseBuilder::new(status_code).json(self)
    }
}
