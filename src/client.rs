use base64::{Engine, prelude::BASE64_STANDARD};
use bytes::Bytes;
use chrono_tz::Tz;
use reqwest::{
    Client, Url,
    header::{self, HeaderMap, HeaderValue},
};

use crate::{
    error::Error,
    models::{
        ApplicationInformation, ChaosTriggersConfiguration, ChaosTriggersResponse,
        DeleteMessagesFilter, HtmlCheckResponse, MessageHeaders, MessageSummary, MessagesSummary,
        ReleaseMessageParams, RenameTagParams, SendMessage, SendMessageResponse,
        SetMessageTagsParams, SetReadStatusParams, SpamAssassinResponse, TagList,
        WebUIConfiguration,
    },
};

pub struct MailpitClient {
    url: Url,
    client: Client,
}

impl MailpitClient {
    /// Create a new [`MailpitClient`] for the given `url`.
    pub fn new(url: &str) -> Result<Self, Error> {
        let url = Url::parse(url)?;
        Ok(Self {
            url,
            client: Client::new(),
        })
    }

    /// Create a new [`MailpitClient`] configured with Basic Authentication
    /// for the given `url`.
    pub fn new_with_auth(url: &str, username: &str, password: &str) -> Result<Self, Error> {
        let url = Url::parse(url)?;

        let encoded = BASE64_STANDARD.encode(&format!("{username}:{password}"));
        let mut headers = HeaderMap::new();
        let mut auth_value = HeaderValue::from_str(&format!("Basic {encoded}")).unwrap();
        auth_value.set_sensitive(true);
        headers.insert(header::AUTHORIZATION, auth_value);

        let client = Client::builder().default_headers(headers).build()?;
        Ok(Self { url, client })
    }

    /// #### Get application information
    /// __GET__ `/api/v1/info`
    ///
    /// Returns basic runtime information, message totals and latest release version.
    ///
    /// #### Errors:
    /// - __`400`__ - Server error will return with a 400 status code with the error message in the body
    pub async fn get_application_information(&self) -> Result<ApplicationInformation, Error> {
        let response = self
            .client
            .get(format!("{}api/v1/info", self.url))
            .send()
            .await?;
        Error::check_response(response)
            .await?
            .json()
            .await
            .map_err(Into::into)
    }

    /// #### Get web UI configuration
    /// __GET__ `/api/v1/webui`
    ///
    /// Returns configuration settings for the web UI. Intended for web UI only!
    ///
    /// #### Errors:
    /// - __`400`__ - Server error will return with a 400 status code with the error message in the body
    pub async fn get_webui_configuration(&self) -> Result<WebUIConfiguration, Error> {
        let response = self
            .client
            .get(format!("{}api/v1/webui", self.url))
            .send()
            .await?;
        Error::check_response(response)
            .await?
            .json()
            .await
            .map_err(Into::into)
    }

    /// #### Get message summary
    /// __GET__ `/api/v1/message/{ID}`
    ///
    /// Returns the summary of a message, marking the message as read.
    ///
    /// The ID can be set to `latest` to return the latest message.
    ///
    /// #### Errors:
    /// - __`400`__ - Server error will return with a 400 status code with the error message in the body
    /// - __`404`__ - Not found error will return a 404 status code
    pub async fn get_message(&self, id: &str) -> Result<MessageSummary, Error> {
        let response = self
            .client
            .get(format!("{}api/v1/message/{id}", self.url))
            .send()
            .await?;
        Error::check_response(response)
            .await?
            .json()
            .await
            .map_err(Into::into)
    }

    /// #### Get message headers
    /// __GET__ `/api/v1/message/{ID}/headers`
    ///
    /// Returns the message headers as an array. Note that header keys are returned alphabetically.
    ///
    /// The ID can be set to `latest` to return the latest message.
    ///
    /// #### Errors:
    /// - __`400`__ - Server error will return with a 400 status code with the error message in the body
    /// - __`404`__ - Not found error will return a 404 status code
    pub async fn get_message_headers(&self, id: &str) -> Result<MessageHeaders, Error> {
        let response = self
            .client
            .get(format!("{}api/v1/message/{id}/headers", self.url))
            .send()
            .await?;
        Error::check_response(response)
            .await?
            .json()
            .await
            .map_err(Into::into)
    }

    /// #### Get message attachment
    /// __GET__ `/api/v1/message/{ID}/part/{PartID}`
    ///
    /// This will return the attachment part using the appropriate Content-Type.
    ///
    /// The ID can be set to `latest` to return the latest message.
    ///
    /// #### Errors:
    /// - __`400`__ - Server error will return with a 400 status code with the error message in the body
    /// - __`404`__ - Not found error will return a 404 status code
    pub async fn get_message_attachment(&self, id: &str, part_id: &str) -> Result<Bytes, Error> {
        let response = self
            .client
            .get(format!("{}api/v1/message/{id}/part/{part_id}", self.url))
            .send()
            .await?;
        Error::check_response(response)
            .await?
            .bytes()
            .await
            .map_err(Into::into)
    }

    /// #### Get an attachment image thumbnail
    /// __GET__ `/api/v1/message/{ID}/part/{PartID}/thumb`
    ///
    /// This will return a cropped 180x120 JPEG thumbnail of an image
    /// attachment. If the image is smaller than 180x120 then the image
    /// is padded. If the attachment is not an image then a blank image
    /// is returned.
    ///
    /// The ID can be set to `latest` to return the latest message.
    ///
    /// #### Errors:
    /// - __`400`__ - Server error will return with a 400 status code with the error message in the body
    pub async fn get_message_attachment_image_thumbnail(
        &self,
        id: &str,
        part_id: &str,
    ) -> Result<Bytes, Error> {
        let response = self
            .client
            .get(format!(
                "{}api/v1/message/{id}/part/{part_id}/thumb",
                self.url
            ))
            .send()
            .await?;
        Error::check_response(response)
            .await?
            .bytes()
            .await
            .map_err(Into::into)
    }

    /// #### Get message source
    /// __GET__ `/api/v1/message/{ID}/raw`
    ///
    /// Returns the full email source as plain text.
    ///
    /// The ID can be set to `latest` to return the latest message.
    ///
    /// #### Errors:
    /// - __`400`__ - Server error will return with a 400 status code with the error message in the body
    /// - __`404`__ - Not found error will return a 404 status code
    pub async fn get_message_source(&self, id: &str) -> Result<String, Error> {
        let response = self
            .client
            .get(format!("{}api/v1/message/{id}/raw", self.url))
            .send()
            .await?;
        Error::check_response(response)
            .await?
            .text()
            .await
            .map_err(Into::into)
    }

    /// #### Release message
    /// __POST__ `/api/v1/message/{ID}/release`
    ///
    /// Release a message via a pre-configured external SMTP server.
    /// This is only enabled if message relaying has been configured.
    ///
    /// The ID can be set to `latest` to return the latest message.
    ///
    /// `To` is a list of addresses.
    ///
    /// #### Errors:
    /// - __`400`__ - Server error will return with a 400 status code with the error message in the body
    /// - __`404`__ - Not found error will return a 404 status code
    pub async fn post_release_message(&self, id: &str, to: &[&str]) -> Result<bool, Error> {
        let response = self
            .client
            .post(format!("{}api/v1/message/{id}/release", self.url))
            .json(&ReleaseMessageParams { to })
            .send()
            .await?;
        Error::check_response(response)
            .await?
            .text()
            .await
            .map(|t| t == "ok")
            .map_err(Into::into)
    }

    /// #### Send a message
    /// __POST__ `/api/v1/send`
    ///
    /// Release a message via a pre-configured external SMTP server.
    /// This is only enabled if message relaying has been configured.
    ///
    /// The ID can be set to `latest` to return the latest message.
    ///
    /// `To` is a list of addresses.
    ///
    /// #### Errors:
    /// - __`400`__ - Server error will return with a 400 status code with a JSON error response in the body
    pub async fn post_send_a_message(
        &self,
        message: SendMessage,
    ) -> Result<SendMessageResponse, Error> {
        let response = self
            .client
            .post(format!("{}api/v1/send", self.url))
            .json(&message)
            .send()
            .await?;
        Error::check_response(response)
            .await?
            .json()
            .await
            .map_err(Into::into)
    }

    /// ####  List messages
    /// __GET__ `/api/v1/messages`
    ///
    /// Returns messages from the mailbox ordered from newest to oldest.
    ///
    /// #### Errors:
    /// - __`400`__ - Server error will return with a 400 status code with the error message in the body
    pub async fn get_list_messages(
        &self,
        start: Option<usize>,
        limit: Option<usize>,
    ) -> Result<MessagesSummary, Error> {
        let mut builder = self.client.get(format!("{}api/v1/messages", self.url));

        if let Some(v) = start {
            builder = builder.query(&[("start", v)]);
        }

        if let Some(v) = limit {
            builder = builder.query(&[("limit", v)]);
        }

        let response = builder.send().await?;
        Error::check_response(response)
            .await?
            .json()
            .await
            .map_err(Into::into)
    }

    /// #### Set read status
    /// __PUT__ `/api/v1/messages`
    ///
    /// You can optionally provide an array of IDs or a search string.
    /// If neither IDs nor search is provided then all mailbox messages
    /// are updated.
    ///
    /// `To` is a list of addresses.
    ///
    /// #### Errors:
    /// - __`400`__ - Server error will return with a 400 status code with the error message in the body
    pub async fn put_set_read_status(
        &self,
        read: Option<bool>,
        ids: Option<&[&str]>,
        search: Option<&str>,
        tz: Option<Tz>,
    ) -> Result<bool, Error> {
        let mut builder = self.client.put(format!("{}api/v1/messages", self.url));

        if let Some(tz) = tz {
            builder = builder.query(&[("tz", tz)]);
        }

        let response = builder
            .json(&SetReadStatusParams {
                ids,
                read: read.unwrap_or_default(),
                search,
            })
            .send()
            .await?;
        Error::check_response(response)
            .await?
            .text()
            .await
            .map(|t| t == "ok")
            .map_err(Into::into)
    }

    /// #### Delete all messages
    /// __DELETE__ `/api/v1/messages`
    ///
    /// Delete all messages. This is only a conveniency wrapper around
    /// [`delete_messages`].
    ///
    /// #### Errors:
    /// - __`400`__ - Server error will return with a 400 status code with the error message in the body
    ///
    /// [`delete_messages`]: crate::client::MailpitClient::delete_messages
    pub async fn delete_all_messages(&self) -> Result<bool, Error> {
        self.delete_messages(&[]).await
    }

    /// #### Delete messages
    /// __DELETE__ `/api/v1/messages`
    ///
    /// Delete individual or all messages. If no IDs are provided then
    /// all messages are deleted.
    ///
    /// #### Errors:
    /// - __`400`__ - Server error will return with a 400 status code with the error message in the body
    pub async fn delete_messages(&self, message_ids: &[&str]) -> Result<bool, Error> {
        let response = self
            .client
            .delete(format!("{}api/v1/messages", self.url))
            .json(&DeleteMessagesFilter { ids: message_ids })
            .send()
            .await?;
        Error::check_response(response)
            .await?
            .text()
            .await
            .map(|t| t == "ok")
            .map_err(Into::into)
    }

    /// #### Search messages
    /// __GET__ `/api/v1/search`
    ///
    /// Returns messages matching a search, sorted by received date
    /// (descending).
    ///
    /// `To` is a list of addresses.
    ///
    /// #### Errors:
    /// - __`400`__ - Server error will return with a 400 status code with the error message in the body
    pub async fn get_search_messages(
        &self,
        query: &str,
        start: Option<&[&str]>,
        limit: Option<String>,
        tz: Option<Tz>,
    ) -> Result<MessagesSummary, Error> {
        let mut builder = self
            .client
            .get(format!("{}api/v1/search", self.url))
            .query(&[("query", query)]);

        if let Some(start) = start {
            builder = builder.query(&[("start", start)]);
        }

        if let Some(limit) = limit {
            builder = builder.query(&[("limit", limit)]);
        }

        if let Some(tz) = tz {
            builder = builder.query(&[("tz", tz)]);
        }

        let response = builder.send().await?;
        Error::check_response(response)
            .await?
            .json()
            .await
            .map_err(Into::into)
    }

    /// #### Delete messages by search
    /// __DELETE__ `/api/v1/search`
    ///
    /// Delete all messages matching [a search](https://mailpit.axllent.org/docs/usage/search-filters/).
    ///
    /// #### Errors:
    /// - __`400`__ - Server error will return with a 400 status code with the error message in the body
    pub async fn delete_messages_by_search(
        &self,
        query: &str,
        tz: Option<Tz>,
    ) -> Result<bool, Error> {
        let mut builder = self
            .client
            .delete(format!("{}api/v1/search", self.url))
            .query(&[("query", query)]);

        if let Some(tz) = tz {
            builder = builder.query(&[("tz", tz)]);
        }

        let response = builder.send().await?;
        Error::check_response(response)
            .await?
            .text()
            .await
            .map(|t| t == "ok")
            .map_err(Into::into)
    }

    /// #### HTML check
    /// __GET__ `/api/v1/message/{ID}/html-check`
    ///
    /// Returns the summary of the message HTML checker.
    ///
    /// The ID can be set to `latest` to return the latest message.
    ///
    /// #### Errors:
    /// - __`400`__ - Server error will return with a 400 status code with the error message in the body
    pub async fn get_html_check(&self, id: &str) -> Result<HtmlCheckResponse, Error> {
        let response = self
            .client
            .get(format!("{}api/v1/message/{id}/html-check", self.url))
            .send()
            .await?;
        Error::check_response(response)
            .await?
            .json()
            .await
            .map_err(Into::into)
    }

    /// #### SpamAssassin check
    /// __GET__ `/api/v1/message/{ID}/sa-check`
    ///
    /// Returns the SpamAssassin summary (if enabled) of the message.
    ///
    /// The ID can be set to `latest` to return the latest message.
    ///
    /// #### Errors:
    /// - __`400`__ - Server error will return with a 400 status code with the error message in the body
    /// - __`404`__ - Not found error will return a 404 status code
    pub async fn get_spam_assassin_check(&self, id: &str) -> Result<SpamAssassinResponse, Error> {
        let response = self
            .client
            .get(format!("{}api/v1/message/{id}/sa-check", self.url))
            .send()
            .await?;
        Error::check_response(response)
            .await?
            .json()
            .await
            .map_err(Into::into)
    }

    /// #### Get all current tags
    /// __GET__ `/api/v1/tags`
    ///
    /// Returns a JSON array of all unique message tags.
    ///
    /// #### Errors:
    /// - __`400`__ - Server error will return with a 400 status code with the error message in the body
    pub async fn get_all_current_tags(&self) -> Result<TagList, Error> {
        let response = self
            .client
            .get(format!("{}api/v1/tags", self.url))
            .send()
            .await?;
        Error::check_response(response)
            .await?
            .json()
            .await
            .map_err(Into::into)
    }

    /// #### Set message tags
    /// __PUT__ `/api/v1/tags`
    ///
    /// This will overwrite any existing tags for selected message
    /// database IDs. To remove all tags from a message, pass an empty
    /// tags array.
    ///
    /// #### Errors:
    /// - __`400`__ - Server error will return with a 400 status code with the error message in the body
    pub async fn put_set_message_tags(&self, ids: &[&str], tags: &[&str]) -> Result<bool, Error> {
        let response = self
            .client
            .put(format!("{}api/v1/tags", self.url))
            .json(&SetMessageTagsParams { ids, tags })
            .send()
            .await?;
        Error::check_response(response)
            .await?
            .text()
            .await
            .map(|t| t == "ok")
            .map_err(Into::into)
    }

    /// #### Rename a tag
    /// __PUT__ `/api/v1/tags/{Tag}`
    ///
    /// Renames an existing tag.
    ///
    /// #### Errors:
    /// - __`400`__ - Server error will return with a 400 status code with the error message in the body
    pub async fn put_rename_a_tag(&self, tag: &str, name: &str) -> Result<bool, Error> {
        let tag = urlencoding::encode(tag);
        let response = self
            .client
            .put(format!("{}api/v1/tags/{tag}", self.url))
            .json(&RenameTagParams { name })
            .send()
            .await?;
        Error::check_response(response)
            .await?
            .text()
            .await
            .map(|t| t == "ok")
            .map_err(Into::into)
    }

    /// #### Delete a tag
    /// __DELETE__ `/api/v1/tags/{Tag}`
    ///
    /// Deletes a tag. This will not delete any messages with the tag,
    /// but will remove the tag from any messages containing the tag.
    ///
    /// #### Errors:
    /// - __`400`__ - Server error will return with a 400 status code with the error message in the body
    pub async fn delete_a_tag(&self, tag: &str) -> Result<bool, Error> {
        let tag = urlencoding::encode(tag);
        let response = self
            .client
            .delete(format!("{}api/v1/tags/{tag}", self.url))
            .send()
            .await?;
        Error::check_response(response)
            .await?
            .text()
            .await
            .map(|t| t == "ok")
            .map_err(Into::into)
    }

    /// #### Get Chaos triggers
    /// __Get__ `/api/v1/chaos`
    ///
    /// Returns the current Chaos triggers configuration. This API
    /// route will return an error if Chaos is not enabled at runtime.
    ///
    /// #### Errors:
    /// - __`400`__ - Server error will return with a 400 status code with the error message in the body
    pub async fn get_chaos_triggers(&self) -> Result<ChaosTriggersResponse, Error> {
        let response = self
            .client
            .get(format!("{}api/v1/chaos", self.url))
            .send()
            .await?;
        Error::check_response(response)
            .await?
            .json()
            .await
            .map_err(Into::into)
    }

    /// #### Set Chaos triggers
    /// __PUT__ `/api/v1/chaos`
    ///
    /// Set the Chaos triggers configuration and return the updated
    /// values. This API route will return an error if Chaos is not
    /// enabled at runtime.
    ///
    /// If any triggers are omitted from the request, then those are
    /// reset to their default values with a 0% probability (ie:
    /// disabled). Setting a blank `{}` will reset all triggers to their
    /// default values.
    ///
    /// #### Errors:
    /// - __`400`__ - Server error will return with a 400 status code with the error message in the body
    pub async fn put_set_chaos_triggers(
        &self,
        config: Option<ChaosTriggersConfiguration>,
    ) -> Result<ChaosTriggersResponse, Error> {
        let response = self
            .client
            .put(format!("{}api/v1/chaos", self.url))
            .json(&config)
            .send()
            .await?;
        Error::check_response(response)
            .await?
            .json()
            .await
            .map_err(Into::into)
    }

    /// #### Render message HTML part
    /// __GET__ `/view/{ID}.html`
    ///
    /// Renders just the message's HTML part which can be used for UI
    /// integration testing. Attached inline images are modified to
    /// link to the API provided they exist. Note that is the message
    /// does not contain a HTML part then an 404 error is returned.
    ///
    /// The ID can be set to `latest` to return the latest message.
    ///
    /// #### Errors:
    /// - __`400`__ - Server error will return with a 400 status code with the error message in the body
    /// - __`404`__ - Not found error will return a 404 status code
    pub async fn get_render_message_html_part(
        &self,
        id: &str,
        embed: Option<bool>,
    ) -> Result<String, Error> {
        let mut builder = self.client.get(format!("{}view/{id}.html", self.url));

        if let Some(embed) = embed {
            builder = builder.query(&[("embed", embed as u8)]);
        }

        let response = builder.send().await?;
        Error::check_response(response)
            .await?
            .text()
            .await
            .map_err(Into::into)
    }

    /// #### Render message text part
    /// __GET__ `/view/{ID}.txt`
    ///
    /// Renders just the message's text part which can be used for UI
    /// integration testing.
    ///
    /// The ID can be set to `latest` to return the latest message.
    ///
    /// #### Errors:
    /// - __`400`__ - Server error will return with a 400 status code with the error message in the body
    /// - __`404`__ - Not found error will return a 404 status code
    pub async fn get_render_message_test_part(&self, id: &str) -> Result<String, Error> {
        let response = self
            .client
            .get(format!("{}view/{id}.txt", self.url))
            .send()
            .await?;
        Error::check_response(response)
            .await?
            .text()
            .await
            .map_err(Into::into)
    }
}
