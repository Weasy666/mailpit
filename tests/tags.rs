use httpmock::{
    Method::{DELETE, GET, PUT},
    MockServer,
};
use mailpit_client::MailpitClient;
use pretty_assertions::assert_eq;

#[tokio::test]
async fn get_all_current_tags_success() {
    let expected_response = r#"[
      "Tag 1",
      "Tag 2"
    ]"#;

    let server = MockServer::start_async().await;
    let mock = server
        .mock_async(|when, then| {
            when.method(GET).path("/api/v1/tags");
            then.status(200)
                .header("content-type", "application/json")
                .body(expected_response);
        })
        .await;

    let client = MailpitClient::new(&server.base_url()).unwrap();
    let response = client.get_all_current_tags().await.unwrap();

    let expected_response: Vec<String> = serde_json::from_str(expected_response).unwrap();
    assert_eq!(&expected_response, &response);

    mock.assert();
}

#[tokio::test]
async fn put_set_message_tags_success() {
    let expected_request =
        r#"{"IDs":["4oRBnPtCXgAqZniRhzLNmS","hXayS6wnCgNnt6aFTvmOF6"],"Tags":["Tag 1","Tag 2"]}"#;

    let server = MockServer::start_async().await;
    let mock = server
        .mock_async(|when, then| {
            when.method(PUT).path("/api/v1/tags").body(expected_request);
            then.status(200)
                .header("content-type", "application/json")
                .body("ok");
        })
        .await;

    let client = MailpitClient::new(&server.base_url()).unwrap();
    let response = client
        .put_set_message_tags(
            &["4oRBnPtCXgAqZniRhzLNmS", "hXayS6wnCgNnt6aFTvmOF6"],
            &["Tag 1", "Tag 2"],
        )
        .await
        .unwrap();

    assert!(response);

    mock.assert();
}

#[tokio::test]
async fn put_rename_tag_success() {
    let expected_request = r#"{"Name":"New name"}"#;

    let server = MockServer::start_async().await;
    let mock = server
        .mock_async(|when, then| {
            when.method(PUT)
                .path("/api/v1/tags/Tag%201")
                .body(expected_request);
            then.status(200)
                .header("content-type", "application/json")
                .body("ok");
        })
        .await;

    let client = MailpitClient::new(&server.base_url()).unwrap();
    let response = client.put_rename_tag("Tag 1", "New name").await.unwrap();

    assert!(response);

    mock.assert();
}

#[tokio::test]
async fn delete_tag_success() {
    let server = MockServer::start_async().await;
    let mock = server
        .mock_async(|when, then| {
            when.method(DELETE).path("/api/v1/tags/Tag%201");
            then.status(200)
                .header("content-type", "application/json")
                .body("ok");
        })
        .await;

    let client = MailpitClient::new(&server.base_url()).unwrap();
    let response = client.delete_tag("Tag 1").await.unwrap();

    assert!(response);

    mock.assert();
}
