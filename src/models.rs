use std::collections::HashMap;

use base64::{Engine, prelude::BASE64_STANDARD};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use url::Url;

use crate::error::Error;

/// Application information
#[derive(Debug, Deserialize, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct ApplicationInformation {
    /// Database path
    pub database: String,
    /// Database size in bytes
    pub database_size: usize,
    /// Latest Mailpit version
    pub latest_version: String,
    /// Total number of messages in the database
    pub messages: usize,
    /// Runtime statistics
    pub runtime_stats: RuntimeStats,
    ///  Tags and message totals per tag
    pub tags: HashMap<String, usize>,
    /// Total number of messages in the database
    pub unread: usize,
    /// Current Mailpit versions
    pub version: String,
}

#[derive(Debug, Deserialize, PartialEq)]
#[serde(rename_all = "PascalCase")]
/// Runtime statistics
pub struct RuntimeStats {
    /// Current memory usage in bytes
    pub memory: usize,
    /// Database runtime messages deleted
    pub messages_deleted: usize,
    /// Accepted runtime SMTP messages
    #[serde(rename = "SMTPAccepted")]
    pub smtp_accepted: usize,
    /// Total runtime accepted messages size in bytes
    #[serde(rename = "SMTPAcceptedSize")]
    pub smtp_accepted_size: usize,
    /// Ignored runtime SMTP messages (when using --ignore-duplicate-ids)
    #[serde(rename = "SMTPIgnored")]
    pub smtp_ignored: usize,
    /// Rejected runtime SMTP messages
    #[serde(rename = "SMTPRejected")]
    pub smtp_rejected: usize,
    /// Mailpit server uptime in seconds
    pub uptime: usize,
}

#[derive(Debug, Deserialize, PartialEq)]
#[serde(rename_all = "PascalCase")]
/// Web UI configuration response
pub struct WebUIConfiguration {
    /// Whether Chaos support is enabled at runtime
    pub chaos_enabled: bool,
    /// Whether messages with duplicate IDs are ignored
    pub duplicates_ignored: bool,
    /// Whether the delete button should be hidden
    pub hide_delete_all_button: bool,
    /// Optional label to identify this Mailpit instance
    pub label: String,
    /// Message Relay information
    pub message_relay: MessageRelay,
    /// Whether SpamAssassin is enabled
    pub spam_assassin: bool,
}

#[derive(Debug, Deserialize, PartialEq)]
#[serde(rename_all = "PascalCase")]
/// Message Relay information
pub struct MessageRelay {
    /// Only allow relaying to these recipients (regex)
    pub allowed_recipients: String,
    /// Block relaying to these recipients (regex)
    pub blocked_recipients: String,
    /// Whether message relaying (release) is enabled
    pub enabled: bool,
    /// Overrides the "From" address for all relayed messages
    pub override_from: String,
    /// Preserve the original Message-IDs when relaying messages
    #[serde(rename = "PreserveMessageIDs")]
    pub preserve_message_ids: bool,
    /// Enforced Return-Path (if set) for relay bounces
    pub return_path: String,
    /// The configured SMTP server address
    #[serde(rename = "SMTPServer")]
    pub smtp_server: String,
}

#[derive(Debug, Deserialize, PartialEq)]
/// MessagesSummary is a summary of a list of messages
pub struct MessagesSummary {
    /// Messages summary in: body
    pub messages: Vec<MessageInfo>,
    /// Total number of messages matching current query
    pub messages_count: usize,
    /// Total number of unread messages matching current query
    pub messages_unread: usize,
    /// Pagination offset
    pub start: usize,
    /// All current tags
    pub tags: Vec<String>,
    /// Total number of messages in mailbox
    pub total: usize,
    /// Total number of unread messages in mailbox
    pub unread: usize,
}

#[derive(Debug, Deserialize, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct MessageBase<T> {
    /// Message attachments
    pub attachments: T,
    /// Bcc addresses
    pub bcc: Option<Vec<AddressObject>>,
    /// Cc addresses
    pub cc: Option<Vec<AddressObject>>,
    /// __Address represents a single mail address.:__ An address such as
    /// "Barry Gibbs bg@example.com" is represented as Address{Name:
    /// "Barry Gibbs", Address: "bg@example.com"}.
    pub from: AddressObject,
    /// Database ID
    #[serde(rename = "ID")]
    pub id: String,
    /// Message ID
    #[serde(rename = "MessageID")]
    pub message_id: String,
    /// ReplyTo addresses
    pub reply_to: Vec<AddressObject>,
    /// Message size in bytes
    pub size: usize,
    /// Message subject
    pub subject: String,
    /// Message tags
    pub tags: Vec<String>,
    /// To addresses
    pub to: Vec<AddressObject>,
    /// Username used for authentication (if provided) with the SMTP or
    /// Send API
    pub username: String,
}

#[derive(Debug, Deserialize, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct MessageInfo {
    #[serde(flatten)]
    pub base: MessageBase<usize>,
    /// Received RFC3339Nano date & time
    /// ([extended RFC3339](https://tools.ietf.org/html/rfc3339#section-5.6)
    /// format with optional nano seconds)
    pub created: DateTime<Utc>,
    /// Read status
    pub read: bool,
    /// Message snippet includes up to 250 characters
    pub snippet: String,
}

impl MessageInfo {
    /// Message attachments
    pub fn attachments(&self) -> usize {
        self.base.attachments
    }

    /// Bcc addresses
    pub fn bcc(&self) -> Option<&Vec<AddressObject>> {
        self.base.bcc.as_ref()
    }

    /// Cc addresses
    pub fn cc(&self) -> Option<&Vec<AddressObject>> {
        self.base.cc.as_ref()
    }

    /// __Address represents a single mail address.:__ An address such as
    /// "Barry Gibbs bg@example.com" is represented as Address{Name:
    /// "Barry Gibbs", Address: "bg@example.com"}.
    pub fn from(&self) -> &AddressObject {
        &self.base.from
    }

    /// Database ID
    pub fn id(&self) -> &str {
        &self.base.id
    }

    /// Message ID
    pub fn message_id(&self) -> &str {
        &self.base.message_id
    }

    /// ReplyTo addresses
    pub fn reply_to(&self) -> &Vec<AddressObject> {
        &self.base.reply_to
    }

    /// Message size in bytes
    pub fn size(&self) -> usize {
        self.base.size
    }

    /// Message subject
    pub fn subject(&self) -> &str {
        &self.base.subject
    }

    /// Message tags
    pub fn tags(&self) -> &Vec<String> {
        &self.base.tags
    }

    /// To addresses
    pub fn to(&self) -> &Vec<AddressObject> {
        &self.base.to
    }

    /// Username used for authentication (if provided) with the SMTP or
    /// Send API
    pub fn username(&self) -> &str {
        &self.base.username
    }
}

#[derive(Debug, Deserialize, PartialEq)]
#[serde(rename_all = "PascalCase")]
/// Message data excluding physical attachments
pub struct MessageSummary {
    #[serde(flatten)]
    pub base: MessageBase<Vec<AttachmentInfo>>,
    /// Message RFC3339Nano date & time (if set), else date & time received
    /// ([extended RFC3339](https://tools.ietf.org/html/rfc3339#section-5.6)
    /// format with optional nano seconds)
    pub date: DateTime<Utc>,
    #[serde(rename = "HTML")]
    /// Message body HTML
    pub html: String,
    /// Inline message attachments
    pub inline: Vec<AttachmentInfo>,
    /// ListUnsubscribe contains a summary of List-Unsubscribe &
    /// List-Unsubscribe-Post headers including validation of the link
    /// structure
    pub list_unsubscribe: ListUnsubscribe,
    /// Return-Path
    pub return_path: String,
    /// Message body text
    pub text: String,
}

impl MessageSummary {
    /// Message attachments
    pub fn attachments(&self) -> &Vec<AttachmentInfo> {
        &self.base.attachments
    }

    /// Bcc addresses
    pub fn bcc(&self) -> Option<&Vec<AddressObject>> {
        self.base.bcc.as_ref()
    }

    /// Cc addresses
    pub fn cc(&self) -> Option<&Vec<AddressObject>> {
        self.base.cc.as_ref()
    }

    /// __Address represents a single mail address.:__ An address such as
    /// "Barry Gibbs bg@example.com" is represented as Address{Name:
    /// "Barry Gibbs", Address: "bg@example.com"}.
    pub fn from(&self) -> &AddressObject {
        &self.base.from
    }

    /// Database ID
    pub fn id(&self) -> &str {
        &self.base.id
    }

    /// Message ID
    pub fn message_id(&self) -> &str {
        &self.base.message_id
    }

    /// ReplyTo addresses
    pub fn reply_to(&self) -> &Vec<AddressObject> {
        &self.base.reply_to
    }

    /// Message size in bytes
    pub fn size(&self) -> usize {
        self.base.size
    }

    /// Message subject
    pub fn subject(&self) -> &str {
        &self.base.subject
    }

    /// Message tags
    pub fn tags(&self) -> &Vec<String> {
        &self.base.tags
    }

    /// To addresses
    pub fn to(&self) -> &Vec<AddressObject> {
        &self.base.to
    }

    /// Username used for authentication (if provided) with the SMTP or
    /// Send API
    pub fn username(&self) -> &str {
        &self.base.username
    }
}

#[derive(Debug, Deserialize, Serialize, PartialEq)]
#[serde(rename_all = "PascalCase")]
/// Email address object
pub struct AddressObject {
    /// Address
    #[serde(rename(serialize = "Email"))]
    pub address: String,
    /// Name
    pub name: Option<String>,
}

#[derive(Debug, Deserialize, PartialEq)]
#[serde(rename_all = "PascalCase")]
/// Message attachment info
pub struct AttachmentInfo {
    /// Content ID
    #[serde(rename = "ContentID")]
    pub content_id: String,
    /// Content type
    pub content_type: String,
    /// File name
    pub file_name: String,
    /// Attachment part ID
    #[serde(rename = "PartID")]
    pub part_id: String,
    /// Size in bytes
    pub size: usize,
}

#[derive(Debug, Deserialize, PartialEq)]
#[serde(rename_all = "PascalCase")]
/// ListUnsubscribe contains a summary of List-Unsubscribe &
/// List-Unsubscribe-Post headers including validation of the link
/// structure
pub struct ListUnsubscribe {
    /// Validation errors (if any)
    pub errors: String,
    /// List-Unsubscribe header value
    pub header: String,
    /// List-Unsubscribe-Post value (if set)
    pub header_post: String,
    /// Detected links, maximum one email and one HTTP(S) link
    pub links: Vec<String>,
}

/// Message headers
pub type MessageHeaders = HashMap<String, Vec<String>>;

#[derive(Debug, Serialize, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub(crate) struct ReleaseMessageParams<'a> {
    pub(crate) to: &'a [&'a str],
}

#[derive(Debug, Serialize, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct SendMessage {
    /// Attachments
    pub attachments: Option<Vec<Attachment>>,
    /// Bcc recipients email addresses only
    pub bcc: Option<Vec<String>>,
    /// Cc recipients
    pub cc: Option<Vec<AddressObject>>,
    /// "From" recipient
    pub from: AddressObject,
    /// Message body (HTML)
    #[serde(rename = "HTML")]
    pub html: String,
    /// Optional headers in {"key":"value"} format
    pub headers: Option<HashMap<String, String>>,
    /// Optional Reply-To recipients
    pub reply_to: Option<Vec<AddressObject>>,
    /// Subject
    pub subject: String,
    /// Mailpit tags
    pub tags: Vec<String>,
    /// Message body (text)
    pub text: String,
    /// "To" recipients
    pub to: Vec<AddressObject>,
}

#[derive(Debug, Serialize, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct Attachment {
    /// Base64-encoded string of the file content
    content: String,
    /// Optional Content-ID (cid) for attachment. If this field is set
    /// then the file is attached inline.
    #[serde(rename = "ContentID")]
    content_id: Option<String>,
    /// Optional Content Type for the the attachment. If this field is
    /// not set (or empty) then the content type is automatically
    /// detected.
    content_type: Option<String>,
    /// Filename
    filename: String,
}

impl Attachment {
    /// Returns [`AttachmentBuilder`] to create an [`Attachment`].
    pub fn builder<'a>() -> AttachmentBuilder<'a> {
        AttachmentBuilder::new()
    }
}

/// Builder to create an [`Attachment`].
#[derive(Default)]
pub struct AttachmentBuilder<'a> {
    content: Option<&'a [u8]>,
    content_id: Option<&'a str>,
    content_type: Option<&'a str>,
    filename: Option<&'a str>,
}

impl<'a> AttachmentBuilder<'a> {
    /// Returns [`AttachmentBuilder`] to create an [`Attachment`].
    pub fn new() -> Self {
        AttachmentBuilder::default()
    }

    /// String of the file content. Will be Base64-encoded on build.
    pub fn content(mut self, content: &'a [u8]) -> Self {
        self.content = Some(content);
        self
    }

    ///  Optional Content-ID (cid) for attachment. If this field is set
    /// then the file is attached inline.
    pub fn content_id(mut self, id: &'a str) -> Self {
        self.content_id = Some(id);
        self
    }

    /// Optional Content Type for the the attachment. If this field is
    /// not set (or empty) then the content type is automatically
    /// detected.
    pub fn content_type(mut self, content_type: &'a str) -> Self {
        self.content_type = Some(content_type);
        self
    }

    /// Filename
    pub fn filename(mut self, name: &'a str) -> Self {
        self.filename = Some(name);
        self
    }

    /// Try building an [`Attachment`] from the set values.
    pub fn build(self) -> Result<Attachment, Error> {
        let Some(filename) = self.filename else {
            return Err(Error::AttachmentFilenameMissing);
        };
        let Some(content) = self.content else {
            return Err(Error::AttachmentContentMissing);
        };

        let encoded_content = BASE64_STANDARD.encode(content);
        Ok(Attachment {
            content: encoded_content,
            content_id: self.content_id.map(Into::into),
            content_type: self.content_type.map(Into::into),
            filename: filename.to_string(),
        })
    }
}

#[derive(Debug, Deserialize, PartialEq)]
/// Confirmation message for HTTP send API
pub struct SendMessageResponse {
    /// Database ID
    #[serde(rename = "ID")]
    pub id: String,
}

#[derive(Debug, Serialize, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub(crate) struct SetReadStatusParams<'a> {
    /// Optional array of message database IDs
    #[serde(rename = "IDs")]
    pub(crate) ids: Option<&'a [&'a str]>,
    /// Read status (Default: false)
    pub(crate) read: bool,
    /// Optional messages matching a search
    pub(crate) search: Option<&'a str>,
}

#[derive(Debug, Serialize, PartialEq)]
pub(crate) struct DeleteMessagesFilter<'a> {
    #[serde(rename = "IDs")]
    pub(crate) ids: &'a [&'a str],
}

#[derive(Debug, Deserialize, PartialEq)]
#[serde(rename_all = "PascalCase")]
/// Response represents the HTML check response struct
pub struct HtmlCheckResponse {
    /// All platforms tested, mainly for the web UI
    pub platforms: HashMap<String, Vec<String>>,
    /// Total weighted result for all scores
    pub total: HtmlTotalScores,
    /// List of warnings from tests
    pub warnings: Vec<HtmlWarning>,
}

#[derive(Debug, Deserialize, PartialEq)]
#[serde(rename_all = "PascalCase")]
/// Response represents the Link check response
pub struct LinkCheckResponse {
    /// Total number of errors
    pub errors: usize,
    /// Tested links
    pub links: Vec<TestedLink>,
}

#[derive(Debug, Deserialize, PartialEq)]
#[serde(rename_all = "PascalCase")]
/// Tested link
pub struct TestedLink {
    /// HTTP status definition
    pub status: String,
    /// HTTP status code
    pub status_code: usize,
    /// Link URL
    #[serde(rename = "URL")]
    pub url: Url,
}

#[derive(Debug, Deserialize, PartialEq)]
#[serde(rename_all = "PascalCase")]
/// Total weighted result for all scores
pub struct HtmlTotalScores {
    /// Total number of HTML nodes detected in message
    pub nodes: usize,
    /// Overall percentage partially supported
    pub partial: f32,
    /// Overall percentage supported
    pub supported: f32,
    /// Total number of tests done
    pub tests: usize,
    /// Overall percentage unsupported
    pub unsupported: f32,
}

#[derive(Debug, Deserialize, PartialEq)]
#[serde(rename_all = "PascalCase")]
/// List of warnings from tests
pub struct HtmlWarning {
    /// Category [css, html]
    pub category: String,
    /// Description
    pub description: String,
    ///Keywords
    pub keywords: String,
    /// Notes based on results
    pub notes_by_number: HashMap<String, String>,
    /// Test results
    pub results: Vec<WarningResult>,
    /// Score struct
    pub score: WarningScore,
    /// Slug identifier
    pub slug: String,
    ///Tags
    pub tags: Vec<String>,
    /// Friendly title
    pub title: String,
    /// URL to caniemail.com
    #[serde(rename = "URL")]
    pub url: String,
}

#[derive(Debug, Deserialize, PartialEq)]
#[serde(rename_all = "PascalCase")]
/// Test results
pub struct WarningResult {
    /// Family eg: Outlook, Mozilla Thunderbird
    pub family: String,
    /// Friendly name of result, combining family, platform & version
    pub name: String,
    /// Note number for partially supported if applicable
    pub note_number: String,
    /// Platform eg: ios, android, windows
    pub platform: String,
    /// Support [yes, no, partial]
    pub support: String,
    /// Family version eg: 4.7.1, 2019-10, 10.3
    pub version: String,
}

#[derive(Debug, Deserialize, PartialEq)]
#[serde(rename_all = "PascalCase")]
/// Score struct
pub struct WarningScore {
    /// Number of matches in the document
    pub found: usize,
    /// Total percentage partially supported
    pub partial: f32,
    /// Total percentage supported
    pub supported: f32,
    /// Total percentage unsupported
    pub unsupported: f32,
}

#[derive(Debug, Deserialize, PartialEq)]
#[serde(rename_all = "PascalCase")]
/// Result is a SpamAssassin result
pub struct SpamAssassinResponse {
    /// If populated will return an error string
    pub error: String,
    /// Whether the message is spam or not
    pub is_spam: bool,
    /// Spam rules triggered
    pub rules: Vec<SpamRule>,
    /// Total spam score based on triggered rules
    pub score: f32,
}

#[derive(Debug, Deserialize, PartialEq)]
#[serde(rename_all = "PascalCase")]
/// Spam rule
pub struct SpamRule {
    /// SpamAssassin rule description
    pub description: String,
    /// SpamAssassin rule name
    pub name: String,
    /// Spam rule score
    pub score: f32,
}

/// Tag array
pub type TagList = Vec<String>;

#[derive(Debug, Serialize, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub(crate) struct SetMessageTagsParams<'a> {
    /// Array of message database IDs
    #[serde(rename = "IDs")]
    pub(crate) ids: &'a [&'a str],
    /// Array of tag names to set
    pub(crate) tags: &'a [&'a str],
}

#[derive(Debug, Serialize, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub(crate) struct RenameTagParams<'a> {
    /// New name
    pub(crate) name: &'a str,
}

#[derive(Debug, Deserialize, PartialEq)]
#[serde(rename_all = "PascalCase")]
/// Triggers for the Chaos configuration
pub struct ChaosTriggersResponse {
    /// Trigger for Chaos
    pub authentication: ChaosTrigger,
    /// Trigger for Chaos
    pub recipient: ChaosTrigger,
    /// Trigger for Chaos
    pub sender: ChaosTrigger,
}

#[derive(Debug, Deserialize, Serialize, PartialEq)]
#[serde(rename_all = "PascalCase")]
/// Trigger for Chaos
pub struct ChaosTrigger {
    /// SMTP error code to return. The value must range from 400 to 599.
    pub error_code: i32,
    /// Probability (chance) of triggering the error. The value must
    /// range from 0 to 100.
    pub probability: i32,
}

#[derive(Debug, Serialize, PartialEq)]
#[serde(rename_all = "PascalCase")]
/// Triggers for the Chaos configuration
pub struct ChaosTriggersConfiguration {
    /// Trigger for Chaos
    pub authentication: ChaosTrigger,
    /// Trigger for Chaos
    pub recipient: ChaosTrigger,
    /// Trigger for Chaos
    pub sender: ChaosTrigger,
}
