#[cfg(feature = "reqwest")]
use crate::Error;

#[cfg(feature = "bytes")]
use bytes::Bytes;
#[cfg(feature = "reqwest")]
use reqwest::Client;
#[cfg(feature = "url")]
use url::Url;

#[cfg(feature = "reqwest")]
pub async fn request(client: &Client, url: Url) -> Result<String, Error> {
    let response = client
        .get(url.clone())
        .send()
        .await
        .map_err(|e| Error::Reqwest(e, url.clone()))?;

    if !response.status().is_success() {
        return Err(Error::Response(url, response.status()));
    }

    match response.text().await {
        Ok(text) => Ok(text),
        Err(err) => Err(Error::from_reqwest(err, url)),
    }
}

#[cfg(all(feature = "reqwest", feature = "bytes"))]
pub async fn request_bytes(client: &Client, url: Url) -> Result<Bytes, Error> {
    let response = client
        .get(url.clone())
        .send()
        .await
        .map_err(|e| Error::Reqwest(e, url.clone()))?;

    if !response.status().is_success() {
        return Err(Error::Response(url, response.status()));
    }

    match response.bytes().await {
        Ok(text) => Ok(text),
        Err(err) => Err(Error::from_reqwest(err, url)),
    }
}
