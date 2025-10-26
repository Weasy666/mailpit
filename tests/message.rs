use base64::{Engine, prelude::BASE64_STANDARD};
use bytes::Bytes;
use httpmock::{
    Method::{GET, POST},
    MockServer,
};
use mailpit_client::{
    MailpitClient,
    models::{
        AddressObject, Attachment, MessageHeaders, MessageSummary, SendMessage, SendMessageResponse,
    },
};
use pretty_assertions::{assert_eq, assert_str_eq};

#[tokio::test]
async fn get_message_summary_success() {
    let expected_response = r#"{
      "Attachments": [
        {
          "ContentID": "string",
          "ContentType": "string",
          "FileName": "string",
          "PartID": "string",
          "Size": 0
        }
      ],
      "Bcc": [
        {
          "Address": "string",
          "Name": "string"
        }
      ],
      "Cc": [
        {
          "Address": "string",
          "Name": "string"
        }
      ],
      "Date": "1970-01-01T00:00:00.000Z",
      "From": {
        "Address": "string",
        "Name": "string"
      },
      "HTML": "string",
      "ID": "database-id",
      "Inline": [
        {
          "ContentID": "string",
          "ContentType": "string",
          "FileName": "string",
          "PartID": "string",
          "Size": 0
        }
      ],
      "ListUnsubscribe": {
        "Errors": "string",
        "Header": "string",
        "HeaderPost": "string",
        "Links": [
          "string"
        ]
      },
      "MessageID": "string",
      "ReplyTo": [
        {
          "Address": "string",
          "Name": "string"
        }
      ],
      "ReturnPath": "string",
      "Size": 0,
      "Subject": "string",
      "Tags": [
        "string"
      ],
      "Text": "string",
      "To": [
        {
          "Address": "string",
          "Name": "string"
        }
      ],
      "Username": "string"
    }"#;

    let server = MockServer::start_async().await;
    let mock = server
        .mock_async(|when, then| {
            when.method(GET).path("/api/v1/message/database-id");
            then.status(200)
                .header("content-type", "application/json")
                .body(expected_response);
        })
        .await;

    let client = MailpitClient::new(&server.base_url()).unwrap();
    let response = client.get_message_summary("database-id").await.unwrap();

    let expected_response: MessageSummary = serde_json::from_str(expected_response).unwrap();
    assert_eq!(&expected_response, &response);

    mock.assert();
}

#[tokio::test]
async fn get_message_headers_success() {
    let expected_response = r#"{
      "property1": [
        "string"
      ],
      "property2": [
        "string"
      ]
    }"#;

    let server = MockServer::start_async().await;
    let mock = server
        .mock_async(|when, then| {
            when.method(GET).path("/api/v1/message/database-id/headers");
            then.status(200)
                .header("content-type", "application/json")
                .body(expected_response);
        })
        .await;

    let client = MailpitClient::new(&server.base_url()).unwrap();
    let response = client.get_message_headers("database-id").await.unwrap();

    let expected_response: MessageHeaders = serde_json::from_str(expected_response).unwrap();
    assert_eq!(&expected_response, &response);

    mock.assert();
}

#[tokio::test]
async fn get_message_attachment_success() {
    let expected_response = Bytes::from("Hello!");

    let server = MockServer::start_async().await;
    let mock = server
        .mock_async(|when, then| {
            when.method(GET)
                .path("/api/v1/message/database-id/part/part-id");
            then.status(200)
                .header("content-type", "application/json")
                .body(&expected_response);
        })
        .await;

    let client = MailpitClient::new(&server.base_url()).unwrap();
    let response = client
        .get_message_attachment("database-id", "part-id")
        .await
        .unwrap();

    assert_eq!(&expected_response, &response);

    mock.assert();
}

#[tokio::test]
async fn get_message_attachment_image_thumbnail_success() {
    let expected_response = Bytes::from("Hello!");

    let server = MockServer::start_async().await;
    let mock = server
        .mock_async(|when, then| {
            when.method(GET)
                .path("/api/v1/message/database-id/part/part-id/thumb");
            then.status(200)
                .header("content-type", "application/json")
                .body(&expected_response);
        })
        .await;

    let client = MailpitClient::new(&server.base_url()).unwrap();
    let response = client
        .get_message_attachment_image_thumbnail("database-id", "part-id")
        .await
        .unwrap();

    assert_eq!(&expected_response, &response);

    mock.assert();
}

#[tokio::test]
async fn get_message_source_success() {
    let expected_response = r#"Some plain text"#;

    let server = MockServer::start_async().await;
    let mock = server
        .mock_async(|when, then| {
            when.method(GET).path("/api/v1/message/database-id/raw");
            then.status(200)
                .header("content-type", "application/json")
                .body(&expected_response);
        })
        .await;

    let client = MailpitClient::new(&server.base_url()).unwrap();
    let response = client.get_message_source("database-id").await.unwrap();

    assert_str_eq!(expected_response, &response);

    mock.assert();
}

#[tokio::test]
async fn post_release_message_success() {
    let expected_request = r#"{"To":["user1@example.com","user2@example.com"]}"#;

    let server = MockServer::start_async().await;
    let mock = server
        .mock_async(|when, then| {
            when.method(POST)
                .path("/api/v1/message/database-id/release")
                .body(expected_request);
            then.status(200)
                .header("content-type", "application/json")
                .body("ok");
        })
        .await;

    let client = MailpitClient::new(&server.base_url()).unwrap();
    let response = client
        .post_release_message("database-id", &["user1@example.com", "user2@example.com"])
        .await
        .unwrap();

    assert!(response);

    mock.assert();
}

#[tokio::test]
async fn post_send_message_success() {
    let expected_request = "{\"Attachments\":[{\"Content\":\"iVBORw0KGgoAAAANSUhEUgAAAEEAAAA8CAMAAAAOlSdoAAAACXBIWXMAAAHrAAAB6wGM2bZBAAAAS1BMVEVHcEwRfnUkZ2gAt4UsSF8At4UtSV4At4YsSV4At4YsSV8At4YsSV4At4YsSV4sSV4At4YsSV4At4YtSV4At4YsSV4At4YtSV8At4YsUWYNAAAAGHRSTlMAAwoXGiktRE5dbnd7kpOlr7zJ0d3h8PD8PCSRAAACWUlEQVR42pXT4ZaqIBSG4W9rhqQYocG+/ys9Y0Z0Br+x3j8zaxUPewFh65K+7yrIMeIY4MT3wPfEJCidKXEMnLaVkxDiELiMz4WEOAZSFghxBIypCOlKiAMgXfIqTnBgSm8CIQ6BImxEUxEckClVQiHGj4Ba4AQHikAIClwTE9KtIghAhUJwoLkmLnCiAHJLRKgIMsEtVUKbBUIwoAg2C4QgQBE6l4VCnApBgSKYLLApCnCa0+96AEMW2BQcmC+Pr3nfp7o5Exy49gIADcIqUELGfeA+bp93LmAJp8QJoEcN3C7NY3sbVANixMyI0nku20/n5/ZRf3KI2k6JEDWQtxcbdGuAqu3TAXG+/799Oyyas1B1MnMiA+XyxHp9q0PUKGPiRAau1fZbLRZV09wZcT8/gHk8QQAxXn8VgaDqcUmU6O/r28nbVwXAqca2mRNtPAF5+zoP2MeN9Fy4NgC6RfcbgE7XITBRYTtOE3U3C2DVff7pk+PkUxgAbvtnPXJaD6DxulMLwOhPS/M3MQkgg1ZFrIXnmfaZoOfpKiFgzeZD/WuKqQEGrfJYkyWf6vlG3xUgTuscnkNkQsb599q124kdpMUjCa/XARHs1gZymVtGt3wLkiFv8rUgTxitYCex5EVGec0Y9VmoDTFBSQte2TfXGXlf7hbdaUM9Sk7fisEN9qfBBTK+FZcvM9fQSdkl2vj4W2oX/bRogO3XasiNH7R0eW7fgRM834ImTg+Lg6BEnx4vz81rhr+MYPBBQg1v8GndEOrthxaCTxNAOut8WKLGZQl+MPz88Q9tAO/hVuSeqQAAAABJRU5ErkJggg==\",\"ContentID\":\"mailpit-logo\",\"ContentType\":\"image/png\",\"Filename\":\"mailpit.png\"}],\"Bcc\":[\"jack@example.com\"],\"Cc\":[{\"Email\":\"manager@example.com\",\"Name\":\"Manager\"}],\"From\":{\"Email\":\"john@example.com\",\"Name\":\"John Doe\"},\"HTML\":\"<div style=\\\"text-align:center\\\"><p style=\\\"font-family: arial; font-size: 24px;\\\">Mailpit is <b>awesome</b>!</p><p><img src=\\\"cid:mailpit-logo\\\" /></p></div>\",\"Headers\":{\"X-IP\":\"1.2.3.4\"},\"ReplyTo\":[{\"Email\":\"secretary@example.com\",\"Name\":\"Secretary\"}],\"Subject\":\"Mailpit message via the HTTP API\",\"Tags\":[\"Tag 1\",\"Tag 2\"],\"Text\":\"Mailpit is awesome!\",\"To\":[{\"Email\":\"jane@example.com\",\"Name\":\"Jane Doe\"}]}";
    let expected_response = r#"{
      "ID": "iAfZVVe2UQfNSG5BAjgYwa"
    }"#;

    let server = MockServer::start_async().await;
    let mock = server
        .mock_async(|when, then| {
            when.method(POST)
                .path("/api/v1/send")
                .body(expected_request);
            then.status(200)
                .header("content-type", "application/json")
                .body(expected_response);
        })
        .await;

    let client = MailpitClient::new(&server.base_url()).unwrap();
    let content = BASE64_STANDARD.decode("iVBORw0KGgoAAAANSUhEUgAAAEEAAAA8CAMAAAAOlSdoAAAACXBIWXMAAAHrAAAB6wGM2bZBAAAAS1BMVEVHcEwRfnUkZ2gAt4UsSF8At4UtSV4At4YsSV4At4YsSV8At4YsSV4At4YsSV4sSV4At4YsSV4At4YtSV4At4YsSV4At4YtSV8At4YsUWYNAAAAGHRSTlMAAwoXGiktRE5dbnd7kpOlr7zJ0d3h8PD8PCSRAAACWUlEQVR42pXT4ZaqIBSG4W9rhqQYocG+/ys9Y0Z0Br+x3j8zaxUPewFh65K+7yrIMeIY4MT3wPfEJCidKXEMnLaVkxDiELiMz4WEOAZSFghxBIypCOlKiAMgXfIqTnBgSm8CIQ6BImxEUxEckClVQiHGj4Ba4AQHikAIClwTE9KtIghAhUJwoLkmLnCiAHJLRKgIMsEtVUKbBUIwoAg2C4QgQBE6l4VCnApBgSKYLLApCnCa0+96AEMW2BQcmC+Pr3nfp7o5Exy49gIADcIqUELGfeA+bp93LmAJp8QJoEcN3C7NY3sbVANixMyI0nku20/n5/ZRf3KI2k6JEDWQtxcbdGuAqu3TAXG+/799Oyyas1B1MnMiA+XyxHp9q0PUKGPiRAau1fZbLRZV09wZcT8/gHk8QQAxXn8VgaDqcUmU6O/r28nbVwXAqca2mRNtPAF5+zoP2MeN9Fy4NgC6RfcbgE7XITBRYTtOE3U3C2DVff7pk+PkUxgAbvtnPXJaD6DxulMLwOhPS/M3MQkgg1ZFrIXnmfaZoOfpKiFgzeZD/WuKqQEGrfJYkyWf6vlG3xUgTuscnkNkQsb599q124kdpMUjCa/XARHs1gZymVtGt3wLkiFv8rUgTxitYCex5EVGec0Y9VmoDTFBSQte2TfXGXlf7hbdaUM9Sk7fisEN9qfBBTK+FZcvM9fQSdkl2vj4W2oX/bRogO3XasiNH7R0eW7fgRM834ImTg+Lg6BEnx4vz81rhr+MYPBBQg1v8GndEOrthxaCTxNAOut8WKLGZQl+MPz88Q9tAO/hVuSeqQAAAABJRU5ErkJggg==").unwrap();
    let attachment = Attachment::builder()
        .content(&content)
        .content_id("mailpit-logo")
        .content_type("image/png")
        .filename("mailpit.png")
        .build()
        .unwrap();
    let request = SendMessage {
        attachments: Some(vec![attachment]),
        bcc: Some(vec!["jack@example.com".to_string()]),
        cc: Some(vec![AddressObject {
            address: "manager@example.com".to_string(),
            name: Some("Manager".to_string()),
        }]),
        from: AddressObject {
            address: "john@example.com".to_string(),
            name: Some("John Doe".to_string()),
        },
        html: "<div style=\"text-align:center\"><p style=\"font-family: arial; font-size: 24px;\">Mailpit is <b>awesome</b>!</p><p><img src=\"cid:mailpit-logo\" /></p></div>".to_string(),
        headers: Some([("X-IP".to_string(), "1.2.3.4".to_string())].into_iter().collect()),
        reply_to: Some(vec![AddressObject {
            address: "secretary@example.com".to_string(),
            name: Some("Secretary".to_string()),
        }]),
        subject: "Mailpit message via the HTTP API".to_string(),
        tags: vec!["Tag 1".to_string(), "Tag 2".to_string()],
        text: "Mailpit is awesome!".to_string(),
        to: vec![AddressObject {
            address: "jane@example.com".to_string(),
            name: Some("Jane Doe".to_string()),
        }],
    };
    let response = client.post_send_message(request).await.unwrap();

    let expected_response: SendMessageResponse = serde_json::from_str(expected_response).unwrap();
    assert_eq!(&expected_response, &response);

    mock.assert();
}
