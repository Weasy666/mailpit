use httpmock::{
    Method::{GET, PUT},
    MockServer,
};
use mailpit_client::{
    MailpitClient,
    models::{ChaosTrigger, ChaosTriggersConfiguration, ChaosTriggersResponse},
};
use pretty_assertions::{assert_eq, assert_str_eq};

#[tokio::test]
async fn get_chaos_triggers_success() {
    let expected_response = r#"{
      "Authentication": {
        "ErrorCode": 451,
        "Probability": 5
      },
      "Recipient": {
        "ErrorCode": 451,
        "Probability": 5
      },
      "Sender": {
        "ErrorCode": 451,
        "Probability": 5
      }
    }"#;

    let server = MockServer::start_async().await;
    let mock = server
        .mock_async(|when, then| {
            when.method(GET).path("/api/v1/chaos");
            then.status(200)
                .header("content-type", "application/json")
                .body(expected_response);
        })
        .await;

    let client = MailpitClient::new(&server.base_url()).unwrap();
    let response = client.get_chaos_triggers().await.unwrap();

    let expected_response: ChaosTriggersResponse = serde_json::from_str(expected_response).unwrap();
    assert_eq!(&expected_response, &response);

    mock.assert();
}

#[tokio::test]
async fn put_set_chaos_triggers_success() {
    let expected_request = r#"{"Authentication":{"ErrorCode":451,"Probability":5},"Recipient":{"ErrorCode":451,"Probability":5},"Sender":{"ErrorCode":451,"Probability":5}}"#;

    let server = MockServer::start_async().await;
    let mock = server
        .mock_async(|when, then| {
            when.method(PUT)
                .path("/api/v1/chaos")
                .body(expected_request);
            then.status(200)
                .header("content-type", "application/json")
                .body(expected_request);
        })
        .await;

    let client = MailpitClient::new(&server.base_url()).unwrap();
    let config = ChaosTriggersConfiguration {
        authentication: ChaosTrigger {
            error_code: 451,
            probability: 5,
        },
        recipient: ChaosTrigger {
            error_code: 451,
            probability: 5,
        },
        sender: ChaosTrigger {
            error_code: 451,
            probability: 5,
        },
    };
    let response = client.put_set_chaos_triggers(Some(config)).await.unwrap();

    let expected_response: ChaosTriggersResponse = serde_json::from_str(expected_request).unwrap();
    assert_eq!(&expected_response, &response);

    mock.assert();
}

#[tokio::test]
async fn get_render_message_html_part_success() {
    let expected_response = r#"<div style="text-align:center"><p style="font-family: arial; font-size: 24px;">Mailpit is <b>awesome</b>!</p><p><img src="cid:mailpit-logo"/></p></div>"#;

    let server = MockServer::start_async().await;
    let mock = server
        .mock_async(|when, then| {
            when.method(GET)
                .path("/view/database-id.html")
                .query_param("embed", 0.to_string());
            then.status(200)
                .header("content-type", "application/json")
                .body(expected_response);
        })
        .await;

    let client = MailpitClient::new(&server.base_url()).unwrap();
    let response = client
        .get_render_message_html_part("database-id", Some(false))
        .await
        .unwrap();

    assert_str_eq!(expected_response, response);

    mock.assert();
}

#[tokio::test]
async fn get_render_message_text_part_success() {
    let expected_response = r#"Mailpit is awesome!"#;

    let server = MockServer::start_async().await;
    let mock = server
        .mock_async(|when, then| {
            when.method(GET).path("/view/database-id.txt");
            then.status(200)
                .header("content-type", "application/json")
                .body(expected_response);
        })
        .await;

    let client = MailpitClient::new(&server.base_url()).unwrap();
    let response = client
        .get_render_message_text_part("database-id")
        .await
        .unwrap();

    assert_str_eq!(expected_response, response);

    mock.assert();
}
