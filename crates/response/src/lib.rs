pub mod category;
pub mod error;

pub use category::Category as ResponseCategory;

#[derive(Debug, Clone)]
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SNAPResponseCommon {
    response_code: u32,
    response_message: String,
    #[serde(skip)]
    http_code: Option<http::StatusCode>,
    #[serde(skip)]
    service_code: Option<u8>,
}

impl SNAPResponseCommon {
    pub fn actual_http_code(&self) -> Option<http::StatusCode> {
        self.http_code
    }

    pub fn actual_service_code(&self) -> Option<u8> {
        self.service_code
    }
}

impl<const SERVICE_CODE: u8> From<crate::error::Error<SERVICE_CODE>> for SNAPResponseCommon {
    fn from(error: crate::error::Error<SERVICE_CODE>) -> Self {
        Self {
            response_code: error.get_code(),
            response_message: error.to_string(),
            http_code: Some(error.get_http_status_code()),
            service_code: Some(SERVICE_CODE),
        }
    }
}

#[derive(Debug, Clone)]
#[derive(serde::Serialize)]
pub struct SNAPResponse<T>
where
    T: serde::Serialize + serde::de::DeserializeOwned,
{
    #[serde(flatten)]
    common: SNAPResponseCommon,
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
                let mut deserialized_response = SNAPResponse::<T>::from(crate::error::Error::<00>::default());

                // Collect all key-value pairs into a serde_json::Map
                let mut value = serde_json::Map::new();

                while let Some((key, val)) = map.next_entry::<String, serde_json::Value>()? {
                    value.insert(key, val);
                }

                // SNAPResponseCommon is expected to be present in the map
                match serde_json::from_value::<crate::SNAPResponseCommon>(serde_json::Value::Object(
                    value.clone(),
                )) {
                    Ok(common_response) => {
                        let response_code = common_response.response_code;
                        let http_code = http::StatusCode::from_u16((response_code / 10_000) as u16)
                            .map_err(|x| serde::de::Error::custom(x.to_string()))?;
                        let service_code =
                            (response_code - ((http_code.as_u16() as u32) * 10_000) / 100) as u8;
                        deserialized_response.common = common_response;
                        deserialized_response.common.http_code = Some(http_code);
                        deserialized_response.common.service_code = Some(service_code);
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

impl<const SERVICE_CODE: u8, T> From<crate::error::Error<SERVICE_CODE>> for SNAPResponse<T>
where
    T: serde::Serialize + serde::de::DeserializeOwned,
{
    fn from(error: crate::error::Error<SERVICE_CODE>) -> Self {
        SNAPResponse {
            common: SNAPResponseCommon::from(error),
            payload: None,
        }
    }
}

impl<T> actix_web::Responder for SNAPResponse<T>
where
    T: serde::Serialize + serde::de::DeserializeOwned,
{
    type Body = actix_web::body::BoxBody;

    fn respond_to(self, _: &actix_web::HttpRequest) -> actix_web::HttpResponse<Self::Body> {
        actix_web::HttpResponseBuilder::new(self.common.http_code.unwrap()).json(self)
    }
}
