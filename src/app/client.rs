use std::io::Cursor;

use anyhow::Result;
use encoding_rs::WINDOWS_1252;
use quick_xml::de::Deserializer;
use reqwest::{Client, Response};
use serde::de::DeserializeOwned;
use tokio::io::AsyncReadExt;
use tracing::{debug, instrument};

// Our client uses reqwest under the hood.
#[derive(Debug)]
pub struct AddInfoClient {
    pub ics_base_url: String,
    pub efa_base_url: String,
    pub client: Client,
}

impl AddInfoClient {
    // Construct a client from the environment variables provided.
    #[instrument(level = "debug")]
    pub fn new(ics_base_url: &str, efa_base_url: &str) -> Self {
        Self {
            ics_base_url: ics_base_url.into(),
            efa_base_url: efa_base_url.into(),
            client: Client::new(),
        }
    }

    // We read response bodies in chunks such that
    // we can make use of the advantages of async code.
    #[instrument(level = "debug", skip_all)]
    pub async fn as_bytes(response: &mut Response) -> Result<Vec<u8>> {
        let mut bytes = vec![];

        while let Some(chunk) = response.chunk().await? {
            bytes.extend_from_slice(&chunk);
        }

        Ok(bytes)
    }

    // Deserialize an XML byte slice into a given type.
    #[instrument(level = "debug", skip_all)]
    pub async fn deserialize<T>(bytes: &[u8]) -> Result<T>
    where
        T: DeserializeOwned,
    {
        debug!("read bytes into buffer");

        let mut reader = Cursor::new(bytes);
        let mut buffer = vec![];
        reader.read_to_end(&mut buffer).await?;

        debug!("decode buffer from windows-1252 into utf-8");

        // The requirement of decoding from Windows-1252 to UTF-8 may depend on
        // the message content and usually isn't necessary. However, this has
        // been implemented for safety and compatibility reasons. You may want
        // to remove this part if you know that your server side only produces
        // valid UTF-8 content. This mainly applies to the XML_ADDINFO_REQUEST
        // as its response may contain arbitrary user input under Windows.
        let (cow, _, _) = WINDOWS_1252.decode(&buffer);
        let utf8 = cow.into_owned();

        debug!("deserialize xml");

        // We require that T implement DeserializeOwned for the below to work.
        let cursor = Cursor::new(utf8);
        let mut deserializer = Deserializer::from_reader(cursor);
        let deserialized = T::deserialize(&mut deserializer)?;

        debug!("ok");

        Ok(deserialized)
    }
}
