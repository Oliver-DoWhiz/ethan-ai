use axum::{
    body::Body,
    http::{Method, Request, StatusCode},
};
use tower::util::ServiceExt;

use ethan_ai::build_app;

#[tokio::test]
async fn landing_route_returns_ok() {
    let app = build_app();

    let response = app
        .oneshot(
            Request::builder()
                .uri("/")
                .method(Method::GET)
                .body(Body::empty())
                .expect("request should build"),
        )
        .await
        .expect("response should succeed");

    assert_eq!(response.status(), StatusCode::OK);
}

#[tokio::test]
async fn studio_route_returns_ok() {
    let app = build_app();

    let response = app
        .oneshot(
            Request::builder()
                .uri("/studio")
                .method(Method::GET)
                .body(Body::empty())
                .expect("request should build"),
        )
        .await
        .expect("response should succeed");

    assert_eq!(response.status(), StatusCode::OK);
}

#[tokio::test]
async fn template_api_returns_catalog() {
    let app = build_app();

    let response = app
        .oneshot(
            Request::builder()
                .uri("/api/templates")
                .method(Method::GET)
                .body(Body::empty())
                .expect("request should build"),
        )
        .await
        .expect("response should succeed");

    assert_eq!(response.status(), StatusCode::OK);
}

#[tokio::test]
async fn plan_api_accepts_valid_payload() {
    let app = build_app();

    let payload = serde_json::json!({
        "listing_title": "Belmont House",
        "city": "San Mateo",
        "neighborhood": "Baywood",
        "price_millions": 3.6,
        "beds": 4,
        "baths": 3.5,
        "sqft": 3180,
        "agent_name": "Olivia Chen",
        "buyer_profile": "design-conscious family",
        "hero_feature": "double-height living room",
        "cta": "Book a private showing with Olivia Chen",
        "brand_voice": "luxury",
        "hook_style": "cinematic",
        "room_sequence": ["arrival", "living room", "kitchen"],
        "assets": {
            "footage_clips": 18,
            "listing_photos": 24,
            "has_floorplan": true,
            "has_voice_notes": true,
            "has_drone": true,
            "has_neighborhood_broll": true
        }
    });

    let response = app
        .oneshot(
            Request::builder()
                .uri("/api/plan")
                .method(Method::POST)
                .header("content-type", "application/json")
                .body(Body::from(payload.to_string()))
                .expect("request should build"),
        )
        .await
        .expect("response should succeed");

    assert_eq!(response.status(), StatusCode::OK);
}
