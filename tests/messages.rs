use httpmock::{
    Method::{DELETE, GET, PUT},
    MockServer,
};
use mailpit_client::{MailpitClient, models::MessagesSummary};
use pretty_assertions::assert_eq;

#[tokio::test]
async fn get_list_messages_success() {
    let expected_response = r#"{
      "messages": [
        {
          "Attachments": 0,
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
          "Created": "1970-01-01T00:00:00.000Z",
          "From": {
            "Address": "string",
            "Name": "string"
          },
          "ID": "string",
          "MessageID": "string",
          "Read": false,
          "ReplyTo": [
            {
              "Address": "string",
              "Name": "string"
            }
          ],
          "Size": 0,
          "Snippet": "string",
          "Subject": "string",
          "Tags": [
            "string"
          ],
          "To": [
            {
              "Address": "string",
              "Name": "string"
            }
          ],
          "Username": "string"
        }
      ],
      "messages_count": 0,
      "messages_unread": 0,
      "start": 0,
      "tags": [
        "string"
      ],
      "total": 0,
      "unread": 0
    }"#;

    let server = MockServer::start_async().await;
    let mock = server
        .mock_async(|when, then| {
            when.method(GET).path("/api/v1/messages");
            then.status(200)
                .header("content-type", "application/json")
                .body(expected_response);
        })
        .await;

    let client = MailpitClient::new(&server.base_url()).unwrap();
    let response = client.get_list_messages(None, None).await.unwrap();

    let expected_response: MessagesSummary = serde_json::from_str(expected_response).unwrap();
    assert_eq!(&expected_response, &response);

    mock.assert();
}

#[tokio::test]
async fn put_set_read_status_success() {
    let expected_request = r#"{"IDs":["4oRBnPtCXgAqZniRhzLNmS","hXayS6wnCgNnt6aFTvmOF6"],"Read":true,"Search":"tag:backups"}"#;

    let server = MockServer::start_async().await;
    let mock = server
        .mock_async(|when, then| {
            when.method(PUT)
                .path("/api/v1/messages")
                .body(expected_request);
            then.status(200)
                .header("content-type", "application/json")
                .body("ok");
        })
        .await;

    let client = MailpitClient::new(&server.base_url()).unwrap();
    let response = client
        .put_set_read_status(
            Some(true),
            Some(&["4oRBnPtCXgAqZniRhzLNmS", "hXayS6wnCgNnt6aFTvmOF6"]),
            Some("tag:backups"),
            None,
        )
        .await
        .unwrap();

    assert!(response);

    mock.assert();
}

#[tokio::test]
async fn delete_messages_success() {
    let expected_request = r#"{"IDs":["4oRBnPtCXgAqZniRhzLNmS","hXayS6wnCgNnt6aFTvmOF6"]}"#;

    let server = MockServer::start_async().await;
    let mock = server
        .mock_async(|when, then| {
            when.method(DELETE)
                .path("/api/v1/messages")
                .body(expected_request);
            then.status(200)
                .header("content-type", "application/json")
                .body("ok");
        })
        .await;

    let client = MailpitClient::new(&server.base_url()).unwrap();
    let response = client
        .delete_messages(&["4oRBnPtCXgAqZniRhzLNmS", "hXayS6wnCgNnt6aFTvmOF6"])
        .await
        .unwrap();

    assert!(response);

    mock.assert();
}

#[tokio::test]
async fn get_search_messages_success() {
    let expected_query = "foo";
    let expected_start = 0;
    let expected_limit = 25;
    let expected_tz = chrono_tz::Europe::Berlin;
    let expected_response = r#"{
      "messages": [
        {
          "Attachments": 0,
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
          "Created": "1970-01-01T00:00:00.000Z",
          "From": {
            "Address": "string",
            "Name": "string"
          },
          "ID": "string",
          "MessageID": "string",
          "Read": false,
          "ReplyTo": [
            {
              "Address": "string",
              "Name": "string"
            }
          ],
          "Size": 0,
          "Snippet": "string",
          "Subject": "string",
          "Tags": [
            "string"
          ],
          "To": [
            {
              "Address": "string",
              "Name": "string"
            }
          ],
          "Username": "string"
        }
      ],
      "messages_count": 0,
      "messages_unread": 0,
      "start": 0,
      "tags": [
        "string"
      ],
      "total": 0,
      "unread": 0
    }"#;

    let server = MockServer::start_async().await;
    let mock = server
        .mock_async(|when, then| {
            when.method(GET)
                .path("/api/v1/search")
                .query_param("query", expected_query)
                .query_param("start", expected_start.to_string())
                .query_param("limit", expected_limit.to_string())
                .query_param("tz", expected_tz.to_string());
            then.status(200)
                .header("content-type", "application/json")
                .body(expected_response);
        })
        .await;

    let client = MailpitClient::new(&server.base_url()).unwrap();
    let response = client
        .get_search_messages(
            expected_query,
            Some(expected_start),
            Some(expected_limit),
            Some(expected_tz),
        )
        .await;

    let expected_response: MessagesSummary = serde_json::from_str(expected_response).unwrap();
    assert_eq!(&expected_response, &response.unwrap());

    mock.assert();
}

#[tokio::test]
async fn delete_messages_by_search_success() {
    let expected_query = "foo";
    let expected_tz = chrono_tz::Europe::Berlin;

    let server = MockServer::start_async().await;
    let mock = server
        .mock_async(|when, then| {
            when.method(DELETE)
                .path("/api/v1/search")
                .query_param("query", expected_query)
                .query_param("tz", expected_tz.to_string());
            then.status(200)
                .header("content-type", "application/json")
                .body("ok");
        })
        .await;

    let client = MailpitClient::new(&server.base_url()).unwrap();
    let response = client
        .delete_messages_by_search(expected_query, Some(expected_tz))
        .await
        .unwrap();

    assert!(response);

    mock.assert();
}
