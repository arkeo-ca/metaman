use metaman::configuration::get_configuration;
use sqlx::{Connection, PgConnection};
use std::net::TcpListener;

fn spawn_app() -> String {
    let listener = TcpListener::bind("127.0.0.1:0").expect("Failed to bind random port");
    let port = listener.local_addr().unwrap().port();
    let server = metaman::startup::run(listener).expect("Failed to bind address");

    let _ = tokio::spawn(server);

    format!("http://127.0.0.1:{}", port)
}

#[tokio::test]
async fn health_check_works() {
    let app_address = spawn_app();
    let client = reqwest::Client::new();

    let response = client
        .get(format!("{}/health_check", &app_address))
        .send()
        .await
        .expect("Failed to execute request.");

    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}

#[tokio::test]
async fn create_marking_returns_a_201_for_valid_form_data() {
    let app_address = spawn_app();
    let configuration = get_configuration().expect("Failed to read configuration");
    let connection_string = configuration.database.connection_string();
    let mut connection = PgConnection::connect(&connection_string)
        .await
        .expect("Failed to connect to Postgres.");
    let client = reqwest::Client::new();

    let body = "{\"name\": \"tlp_red\", \"definition_type\": \"tlp\", \"definition\": \"TLP Red\"}";
    let response = client
        .post(&format!("{}/markings", &app_address))
        .header("Content-Type", "application/json")
        .body(body)
        .send()
        .await
        .expect("Failed to execute request.");

    assert_eq!(201, response.status().as_u16());

    let saved = sqlx::query!("SELECT name, definition_type, definition FROM markings",)
        .fetch_one(&mut connection)
        .await
        .expect("Failed to fetch saved marking.");

    assert_eq!(saved.name, "tlp_red");
    assert_eq!(saved.definition_type, "tlp");
    assert_eq!(saved.definition, "TLP Red");
}

#[tokio::test]
async fn create_marking_returns_a_400_when_data_is_missing() {
    let app_address = spawn_app();
    let client = reqwest::Client::new();

    let test_cases = vec![
        (
            "{\"name\": \"tlp_red\", \"definition_type\": \"tlp\"}",
            "missing the definition",
        ),
        (
            "{\"name\": \"tlp_red\", \"definition\": \"TLP Red\"}",
            "missing the definition type",
        ),
        (
            "{\"definition_type\": \"tlp\", \"definition\": \"TLP Red\"}",
            "missing the name",
        ),
        (
            "{\"name\": \"tlp_red\"}",
            "missing the definition type and the definition",
        ),
        (
            "{\"definition_type\": \"tlp\",}",
            "missing the name and the definition",
        ),
        (
            "{\"definition\": \"TLP Red\"}",
            "missing the name and the definition type",
        ),
        ("{}", "missing the name, definition, and definition type"),
    ];

    for (invalid_body, error_message) in test_cases {
        let response = client
            .post(&format!("{}/markings", &app_address))
            .header("Content-Type", "application/json")
            .body(invalid_body)
            .send()
            .await
            .expect("Failed to execute request.");

        assert_eq!(
            400,
            response.status().as_u16(),
            "The API did not fail with 400 Bad Request when the payload was {}",
            error_message
        );
    }
}
