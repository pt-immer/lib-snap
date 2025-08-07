#![allow(clippy::doc_lazy_continuation)]
#![allow(clippy::zero_prefixed_literal)]

pub mod category;
pub mod error;

mod macros;

pub use category::Category as ResponseCategory;
pub use error::Error as ResponseError;

pub type Result<T> = core::result::Result<T, ResponseError>;

#[derive(Debug, Clone)]
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SNAPResponseCommon {
    response_code: String,
    response_message: String,
    #[serde(skip)]
    http_code: Option<actix_web::http::StatusCode>,
    #[serde(skip)]
    service_code: Option<u8>,
}

impl SNAPResponseCommon {
    pub fn success(service_code: u8, sub_code: u8) -> Self {
        let response_code = ((service_code as u32) * 100 + (sub_code as u32) + 2_000_000).to_string();
        let response_message = "Successful".to_string();
        let http_code = actix_web::http::StatusCode::OK;

        Self {
            response_code,
            response_message,
            http_code: Some(http_code),
            service_code: Some(service_code),
        }
    }

    pub fn from_error(error: crate::ResponseError, service_code: u8) -> Self {
        let response_code = (error.get_code(service_code)).to_string();
        let response_message = error.to_string();
        let http_code = Some(error.get_http_status_code());
        let service_code = Some(service_code);

        Self {
            response_code,
            response_message,
            http_code,
            service_code,
        }
    }

    pub fn http_code(&self) -> Option<actix_web::http::StatusCode> {
        self.http_code
    }

    pub fn service_code(&self) -> Option<u8> {
        self.service_code
    }
}

#[derive(Debug, Clone)]
#[derive(serde::Serialize)]
pub struct SNAPResponse<T>
where
    T: serde::Serialize + serde::de::DeserializeOwned,
{
    #[serde(flatten)]
    common: Option<SNAPResponseCommon>,
    #[serde(flatten)]
    payload: Option<T>,
}

impl<T> SNAPResponse<T>
where
    T: serde::Serialize + serde::de::DeserializeOwned,
{
    pub fn get_payload(&self) -> Option<&T> {
        self.payload.as_ref()
    }

    pub fn from_payload(payload: T, service_code: u8) -> Self {
        let common = SNAPResponseCommon::success(service_code, 0);

        SNAPResponse {
            common: Some(common),
            payload: Some(payload),
        }
    }

    pub fn from_error(error: crate::error::Error, service_code: u8) -> Self {
        let common = SNAPResponseCommon::from_error(error, service_code);

        SNAPResponse {
            common: Some(common),
            payload: None,
        }
    }
}

impl<'de, T> serde::Deserialize<'de> for SNAPResponse<T>
where
    T: serde::Serialize + serde::de::DeserializeOwned,
{
    fn deserialize<D>(deserializer: D) -> core::result::Result<Self, D::Error>
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

            fn visit_map<M>(self, mut map: M) -> core::result::Result<Self::Value, M::Error>
            where
                M: serde::de::MapAccess<'de>,
            {
                let mut deserialized_response = SNAPResponse::<T> {
                    common: None,
                    payload: None,
                };

                // Collect all key-value pairs into a serde_json::Map
                let mut value = serde_json::Map::new();

                while let Some((key, val)) = map.next_entry::<String, serde_json::Value>()? {
                    value.insert(key, val);
                }

                // SNAPResponseCommon is expected to be present in the map
                match serde_json::from_value::<crate::SNAPResponseCommon>(serde_json::Value::Object(
                    value.clone(),
                )) {
                    Ok(mut common_response) => {
                        let response_code = common_response.response_code.clone();
                        let response_code = response_code
                            .parse::<u32>()
                            .map_err(|_| serde::de::Error::custom("Invalid responseCode format"))?;
                        let http_code =
                            actix_web::http::StatusCode::from_u16((response_code / 10_000) as u16)
                                .map_err(|x| serde::de::Error::custom(x.to_string()))?;
                        let service_code =
                            (response_code - ((http_code.as_u16() as u32) * 10_000) / 100) as u8;
                        common_response.http_code = Some(http_code);
                        common_response.service_code = Some(service_code);
                        deserialized_response.common = Some(common_response);
                    }
                    Err(_) => {
                        return Err(serde::de::Error::custom(
                            "Invalid SNAPResponse: neither error nor success could be deserialized",
                        ));
                    }
                }

                // Attempt to deserialize the payload if it exists
                match serde_json::from_value::<T>(serde_json::Value::Object(value)) {
                    Ok(payload) => deserialized_response.payload = Some(payload),
                    Err(_) => deserialized_response.payload = None,
                }

                Ok(deserialized_response)
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
        let http_code = self.common.as_ref().unwrap().http_code().unwrap();

        actix_web::HttpResponseBuilder::new(http_code).json(self)
    }
}

impl<T> From<Result<T>> for SNAPResponse<T>
where
    T: serde::Serialize + serde::de::DeserializeOwned,
{
    fn from(result: Result<T>) -> Self {
        match result {
            Ok(payload) => SNAPResponse::from_payload(payload, 0),
            Err(error) => SNAPResponse::from_error(error, 0),
        }
    }
}
