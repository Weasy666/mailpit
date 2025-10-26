use httpmock::{Method::GET, MockServer};
use mailpit_client::{
    MailpitClient,
    models::{ApplicationInformation, WebUIConfiguration},
};
use pretty_assertions::assert_eq;

#[tokio::test]
async fn get_application_information_success() {
    let expected_response = r#"{
      "Database": "string",
      "DatabaseSize": 0,
      "LatestVersion": "string",
      "Messages": 0,
      "RuntimeStats": {
        "Memory": 0,
        "MessagesDeleted": 0,
        "SMTPAccepted": 0,
        "SMTPAcceptedSize": 0,
        "SMTPIgnored": 0,
        "SMTPRejected": 0,
        "Uptime": 0
      },
      "Tags": {
        "property1": 0,
        "property2": 0
      },
      "Unread": 0,
      "Version": "string"
    }"#;

    let server = MockServer::start_async().await;
    let mock = server
        .mock_async(|when, then| {
            when.method(GET).path("/api/v1/info");
            then.status(200)
                .header("content-type", "application/json")
                .body(expected_response);
        })
        .await;

    let client = MailpitClient::new(&server.base_url()).unwrap();
    let response = client.get_application_information().await.unwrap();

    let expected_response: ApplicationInformation =
        serde_json::from_str(expected_response).unwrap();
    assert_eq!(&expected_response, &response);

    mock.assert();
}

#[tokio::test]
async fn get_web_ui_configuration_success() {
    let expected_response = r#"{
      "ChaosEnabled": false,
      "DuplicatesIgnored": false,
      "HideDeleteAllButton": false,
      "Label": "string",
      "MessageRelay": {
        "AllowedRecipients": "string",
        "BlockedRecipients": "string",
        "Enabled": false,
        "OverrideFrom": "string",
        "PreserveMessageIDs": false,
        "ReturnPath": "string",
        "SMTPServer": "string"
      },
      "SpamAssassin": false
    }"#;

    let server = MockServer::start_async().await;
    let mock = server
        .mock_async(|when, then| {
            when.method(GET).path("/api/v1/webui");
            then.status(200)
                .header("content-type", "application/json")
                .body(expected_response);
        })
        .await;

    let client = MailpitClient::new(&server.base_url()).unwrap();
    let response = client.get_webui_configuration().await.unwrap();

    let expected_response: WebUIConfiguration = serde_json::from_str(expected_response).unwrap();
    assert_eq!(&expected_response, &response);

    mock.assert();
}
