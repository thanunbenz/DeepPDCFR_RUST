use axum::{
    body::Body,
    http::{Request, StatusCode},
};
use deeppdcfr_mock_server::create_app;
use serde_json::json;
use tower::util::ServiceExt; // for `oneshot` and `ready`

#[tokio::test]
async fn test_health_endpoint() {
    let app = create_app();

    let response = app
        .oneshot(
            Request::builder()
                .uri("/health")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);

    let body = axum::body::to_bytes(response.into_body(), usize::MAX)
        .await
        .unwrap();
    let body: serde_json::Value = serde_json::from_slice(&body).unwrap();

    assert_eq!(body["status"], "ok");
    assert_eq!(body["model_loaded"], true);
    assert_eq!(body["version"], "0.1.0");
}

#[tokio::test]
async fn test_solve_endpoint() {
    let app = create_app();

    let request_body = json!({
        "player": "OOP",
        "board": "Ah Kd Qc",
        "effective_stack": 100,
        "starting_pot": 20
    });

    let response = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/v1/solve")
                .header("content-type", "application/json")
                .body(Body::from(serde_json::to_string(&request_body).unwrap()))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);

    let body = axum::body::to_bytes(response.into_body(), usize::MAX)
        .await
        .unwrap();
    let body: serde_json::Value = serde_json::from_slice(&body).unwrap();

    // Verify response structure
    assert_eq!(body["player"], "OOP");
    assert_eq!(body["board"], "Ah Kd Qc");
    assert_eq!(body["pot"], 20);
    assert_eq!(body["effective_stack"], 100);
    assert_eq!(body["num_combos"], 46);

    // Verify actions array
    let actions = body["actions"].as_array().unwrap();
    assert_eq!(actions.len(), 4);
    assert_eq!(actions[0]["name"], "Check");
    assert_eq!(actions[1]["name"], "Bet 33%");
    assert_eq!(actions[2]["name"], "Bet 67%");
    assert_eq!(actions[3]["name"], "All-in");

    // Verify combos array
    let combos = body["combos"].as_array().unwrap();
    assert_eq!(combos.len(), 46);

    // Verify first combo structure
    let first_combo = &combos[0];
    assert_eq!(first_combo["hand"], "AcAd");
    assert_eq!(first_combo["hand_id"], 1320);
    let strategy = first_combo["strategy"].as_array().unwrap();
    assert_eq!(strategy.len(), 4);

    // Verify strategies sum to 1.0
    let sum: f64 = strategy.iter().map(|v| v.as_f64().unwrap()).sum();
    assert!((sum - 1.0).abs() < 0.001);
}

#[tokio::test]
async fn test_cors_headers() {
    let app = create_app();

    let response = app
        .oneshot(
            Request::builder()
                .method("OPTIONS")
                .uri("/health")
                .header("origin", "http://example.com")
                .header("access-control-request-method", "GET")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    // CORS should allow the request
    assert!(response.status().is_success() || response.status() == StatusCode::NO_CONTENT);

    let headers = response.headers();
    assert!(headers.contains_key("access-control-allow-origin"));
}

#[tokio::test]
async fn test_swagger_ui_accessible() {
    let app = create_app();

    let response = app
        .oneshot(
            Request::builder()
                .uri("/docs/")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    // Swagger UI should be accessible (returns HTML or redirects)
    assert!(
        response.status().is_success() || response.status().is_redirection(),
        "Swagger UI should be accessible"
    );
}
