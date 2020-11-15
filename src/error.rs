use std::fmt;

#[cfg(feature = "reqwest")]
use reqwest::StatusCode;
#[cfg(feature = "url")]
use url::Url;

/// The error type for libaur.
#[derive(Debug)]
pub enum Error {
    ///Reqwest returned an error.
    #[cfg(feature = "reqwest")]
    Reqwest(reqwest::Error, Url),
    #[cfg(feature = "url")]
    /// There was an error parsing a URL.
    Url(url::ParseError),
    #[cfg(feature = "reqwest")]
    /// The response code was not success.
    Response(Url, StatusCode),
    #[cfg(feature = "rss")]
    /// Rss returned an error.
    Rss(rss::Error),
}

impl Error {
    #[cfg(feature = "reqwest")]
    pub(crate) fn from_reqwest(err: reqwest::Error, url: Url) -> Self {
        Error::Reqwest(err, url)
    }
}

#[cfg(feature = "url")]
impl From<url::ParseError> for Error {
    fn from(err: url::ParseError) -> Self {
        Error::Url(err)
    }
}

#[cfg(feature = "rss")]
impl From<rss::Error> for Error {
    fn from(err: rss::Error) -> Self {
        Error::Rss(err)
    }
}

impl fmt::Display for Error {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            #[cfg(feature = "reqwest")]
            Error::Reqwest(e, u) => write!(f, "{}: {}", u, e)?,
            #[cfg(feature = "url")]
            Error::Url(e) => e.fmt(f)?,
            #[cfg(feature = "reqwest")]
            Error::Response(u, s) => write!(f, "{}: {}", u, s)?,
            #[cfg(feature = "rss")]
            Error::Rss(e) => e.fmt(f)?,
            #[allow(unreachable_patterns)]
            _ => (),
        }

        Ok(())
    }
}

impl std::error::Error for Error {}
