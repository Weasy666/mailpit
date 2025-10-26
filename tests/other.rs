use httpmock::{Method::GET, MockServer};
use mailpit_client::{
    MailpitClient,
    models::{HtmlCheckResponse, LinkCheckResponse, SpamAssassinResponse},
};
use pretty_assertions::assert_eq;

#[tokio::test]
async fn get_html_check_success() {
    let expected_response = r#"{
      "Platforms": {
        "property1": [
          "string"
        ],
        "property2": [
          "string"
        ]
      },
      "Total": {
        "Nodes": 0,
        "Partial": 0,
        "Supported": 0,
        "Tests": 0,
        "Unsupported": 0
      },
      "Warnings": [
        {
          "Category": "string",
          "Description": "string",
          "Keywords": "string",
          "NotesByNumber": {
            "property1": "string",
            "property2": "string"
          },
          "Results": [
            {
              "Family": "string",
              "Name": "string",
              "NoteNumber": "string",
              "Platform": "string",
              "Support": "string",
              "Version": "string"
            }
          ],
          "Score": {
            "Found": 0,
            "Partial": 0,
            "Supported": 0,
            "Unsupported": 0
          },
          "Slug": "string",
          "Tags": [
            "string"
          ],
          "Title": "string",
          "URL": "string"
        }
      ]
    }"#;

    let server = MockServer::start_async().await;
    let mock = server
        .mock_async(|when, then| {
            when.method(GET)
                .path("/api/v1/message/database-id/html-check");
            then.status(200)
                .header("content-type", "application/json")
                .body(expected_response);
        })
        .await;

    let client = MailpitClient::new(&server.base_url()).unwrap();
    let response = client.get_html_check("database-id").await.unwrap();

    let expected_response: HtmlCheckResponse = serde_json::from_str(expected_response).unwrap();
    assert_eq!(&expected_response, &response);

    mock.assert();
}

#[tokio::test]
async fn get_link_check_success() {
    let expected_response = r#"{
      "Errors": 0,
      "Links": [
        {
          "Status": "string",
          "StatusCode": 0,
          "URL": "http://localhost:8025/api"
        }
      ]
    }"#;

    let server = MockServer::start_async().await;
    let mock = server
        .mock_async(|when, then| {
            when.method(GET)
                .path("/api/v1/message/database-id/link-check")
                .query_param("follow", "true");
            then.status(200)
                .header("content-type", "application/json")
                .body(expected_response);
        })
        .await;

    let client = MailpitClient::new(&server.base_url()).unwrap();
    let response = client
        .get_link_check("database-id", Some(true))
        .await
        .unwrap();

    let expected_response: LinkCheckResponse = serde_json::from_str(expected_response).unwrap();
    assert_eq!(&expected_response, &response);

    mock.assert();
}

#[tokio::test]
async fn get_spam_assassin_check_success() {
    let expected_response = r#"{
      "Error": "string",
      "IsSpam": false,
      "Rules": [
        {
          "Description": "string",
          "Name": "string",
          "Score": 0
        }
      ],
      "Score": 0
    }"#;

    let server = MockServer::start_async().await;
    let mock = server
        .mock_async(|when, then| {
            when.method(GET)
                .path("/api/v1/message/database-id/sa-check");
            then.status(200)
                .header("content-type", "application/json")
                .body(expected_response);
        })
        .await;

    let client = MailpitClient::new(&server.base_url()).unwrap();
    let response = client.get_spam_assassin_check("database-id").await.unwrap();

    let expected_response: SpamAssassinResponse = serde_json::from_str(expected_response).unwrap();
    assert_eq!(&expected_response, &response);

    mock.assert();
}
