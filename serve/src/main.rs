use axum::{
    extract::State,
    http::StatusCode,
    response::Json,
    routing::post,
    Router,
};
use ort::session::{builder::GraphOptimizationLevel, Session};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tower_http::cors::CorsLayer;
use tracing_subscriber;

#[derive(Deserialize)]
struct GasPredictRequest {
    hour: i32,
    day_of_week: i32,
    prev_gas_1: f32,
    prev_gas_2: f32,
    prev_gas_3: f32,
    high_bids_count: i32,
    avg_bid_price: f32,
}

#[derive(Serialize)]
struct GasPredictResponse {
    predicted_gas_price: f32,
}

struct AppState {
    model: Session,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt::init();

    let model = Session::builder()?
        .with_optimization_level(GraphOptimizationLevel::All)?
        .with_intra_threads(4)?
        .commit_from_file("/model/model.onnx")?;

    let state = Arc::new(AppState { model });

    let app = Router::new()
        .route("/predict", post(predict))
        .layer(CorsLayer::permissive())
        .with_state(state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await?;
    tracing::info!("Server running on http://0.0.0.0:3000");
    
    axum::serve(listener, app).await?;
    Ok(())
}

async fn predict(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<GasPredictRequest>,
) -> Result<Json<GasPredictResponse>, StatusCode> {
    let features = vec![
        vec![
            payload.hour as f32,
            payload.day_of_week as f32,
            payload.prev_gas_1,
            payload.prev_gas_2,
            payload.prev_gas_3,
            payload.high_bids_count as f32,
            payload.avg_bid_price,
        ]
    ];
    
    let outputs = state.model
        .run(ort::inputs!{"float_input" => features}?)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let output = &outputs[0];
    let predictions = output
        .try_extract_tensor::<f32>()
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        .view()
        .as_slice()
        .ok_or(StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(GasPredictResponse {
        predicted_gas_price: predictions[0],
    }))
}