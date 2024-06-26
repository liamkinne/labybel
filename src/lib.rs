use serde::Deserialize;
use serde_xml_rs::from_str;

#[derive(Debug)]
pub struct Client {
    host: String,
    port: u16,
}

#[derive(Debug)]
pub enum Error {
    Request(reqwest::Error),
    Deserialization(serde_xml_rs::Error),
}

impl Client {
    const BASE_PATH: &'static str = "DYMO/DLS/Printing";

    /// Creates a new client instance.
    ///
    /// # Example
    ///
    /// ```rust
    /// use labybel::Client;
    /// let client = Client::new("http://127.0.0.1", None);
    /// ```
    pub fn new<S>(host: S, port: Option<u16>) -> Self
    where
        S: Into<String>,
    {
        Self {
            host: host.into(),
            port: port.unwrap_or(41951),
        }
    }

    fn request(&self) -> reqwest::Client {
        reqwest::Client::new()
    }

    fn path(&self) -> String {
        format!("{}:{}/{}/", self.host, self.port, Self::BASE_PATH)
    }

    /// Get connection status.
    pub async fn connected(&self) -> Result<bool, Error> {
        let res: bool = self
            .request()
            .get(self.path() + "StatusConnected")
            .send()
            .await
            .map_err(Error::Request)?
            .json()
            .await
            .map_err(Error::Request)?;

        Ok(res)
    }

    /// Get connection status.
    pub async fn printers(&self) -> Result<Vec<PrinterResponse>, Error> {
        let res = self
            .request()
            .get(self.path() + "GetPrinters")
            .send()
            .await
            .map_err(Error::Request)?
            .text()
            .await
            .map_err(Error::Request)?;

        #[derive(Deserialize)]
        #[serde(rename_all = "PascalCase")]
        struct Response {
            #[serde(rename = "$value")]
            printers: Vec<PrinterResponse>,
        }

        let res: Response = from_str(&res).map_err(Error::Deserialization)?;

        Ok(res.printers)
    }
}

#[derive(Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct PrinterResponse {
    name: String,
    model_name: String,
    #[serde(deserialize_with = "deserialize_bool_from_str")]
    is_connected: bool,
    #[serde(deserialize_with = "deserialize_bool_from_str")]
    is_local: bool,
    #[serde(deserialize_with = "deserialize_bool_from_str")]
    is_twin_turbo: bool,
}

/// Deal with capitalised `True` and `False` in response.
fn deserialize_bool_from_str<'de, D>(deserializer: D) -> Result<bool, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let s: String = Deserialize::deserialize(deserializer)?;
    Ok(s == "True")
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde::de::value::StrDeserializer;
    use serde::de::IntoDeserializer;

    #[test]
    fn test_deserialize_bool_from_str() {
        let true_str: StrDeserializer<serde::de::value::Error> = "True".into_deserializer();
        let false_str: StrDeserializer<serde::de::value::Error> = "False".into_deserializer();

        assert_eq!(deserialize_bool_from_str(true_str).unwrap(), true);
        assert_eq!(deserialize_bool_from_str(false_str).unwrap(), false);
    }
}
