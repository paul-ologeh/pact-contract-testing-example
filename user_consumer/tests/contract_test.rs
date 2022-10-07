use chrono::Utc;
use pact_consumer::prelude::*;
use pact_consumer::*;
use user_consumer::{create_user, get_user};
use user_provider::{UserIn, UserOut};
use uuid::Uuid;

#[tokio::test]
async fn it_creates_a_users_and_returns_a_serialised_user() {
    let user_in = UserIn {
        username: "john1234".to_owned(),
        password: "john1234".to_owned(),
        name: "John".to_owned(),
        surname: "Smith".to_owned(),
        age: 29,
        job_title: Some("Software Developer".to_owned()),
        phone_number: Some("+1975789024".to_owned()),
    };

    let user_out = UserOut {
        id: Uuid::new_v4(),
        username: "John42".to_owned(),
        name: "John".to_owned(),
        surname: "Doe".to_owned(),
        age: 42,
        timestamp: Default::default(),
        job_title: Some("Software Developer".to_owned()),
        phone_number: Some("+48111222333".to_owned()),
    };

    let pact = PactBuilder::new("User Consumer Service", "User Provider Service")
        .interaction("a request to create a user", "", |mut i| async {
            i.request
                .post()
                .path("/user")
                .content_type("application/json")
                .json_body(like!(serde_json::json!(user_in)));
            i.response
                .created()
                .content_type("application/json")
                .json_body(like!(strip_null_fields(serde_json::json!(user_out))));

            i
        })
        .await
        .output_dir("../pact_contracts")
        .start_mock_server(None);

    let base_url = pact.url().to_string();
    let response = create_user(base_url, user_in).await.unwrap();

    assert_eq!(response, user_out);
}

#[tokio::test]
async fn it_can_fetch_a_user() {
    let user_out = UserOut {
        id: Uuid::new_v4(),
        username: "John42".to_owned(),
        name: "John".to_owned(),
        surname: "Doe".to_owned(),
        age: 42,
        timestamp: Utc::now().naive_utc(),
        job_title: Some("Software Developer".to_owned()),
        phone_number: Some("+48111222333".to_owned()),
    };

    let pact = PactBuilder::new("User Consumer Service", "User Provider Service")
        .output_dir("../pact_contracts")
        .interaction("a request to list a user", "", |mut i| async {
            i.request.path(format!("/user/{}", user_out.id.to_string()));
            i.response
                .content_type("application/json")
                .json_body(like!(strip_null_fields(serde_json::json!(&user_out))));

            i
        })
        .await
        .start_mock_server(None);

    let base_url = pact.url().to_string();
    let response = get_user(base_url, user_out.id).await.unwrap();

    assert_eq!(response, user_out);
}
