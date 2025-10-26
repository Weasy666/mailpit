use reqwest::Error as ReqwestError;
use serde::Deserialize;
use thiserror::Error;
use url::ParseError;

#[derive(Debug, Error)]
pub enum Error {
    #[error("Invalid Url: {0}")]
    InvalidUrl(#[from] ParseError),
    #[error("Mailpit network error: {0}")]
    ReqwestFailure(#[from] ReqwestError),
    #[error("Mailpit network error: {status}")]
    HttpFailure {
        status: u16,
        body: Option<MailpitError>,
        text: String,
    },
    #[error(
        "Trying to build an attachment without a `filename`. Make sure you set one on the builder."
    )]
    AttachmentFilenameMissing,
    #[error(
        "Trying to build an attachment without `content`. Make sure you set content on the builder."
    )]
    AttachmentContentMissing,
}

impl Error {
    pub(crate) async fn check_response(
        response: reqwest::Response,
    ) -> Result<reqwest::Response, Error> {
        if !response.status().is_success() {
            let status = response.status().into();
            let text = response.text().await?;
            return Err(Error::HttpFailure {
                status,
                body: serde_json::from_str(&text).ok(),
                text,
            });
        }

        Ok(response)
    }
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct MailpitError {
    pub error: String,
}
