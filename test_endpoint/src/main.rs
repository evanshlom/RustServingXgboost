use reqwest;
use serde::Serialize;
use std::time::Instant;
use futures::future::join_all;

#[derive(Serialize, Clone)]
struct GasPredictRequest {
    hour: i32,
    day_of_week: i32,
    prev_gas_1: f32,
    prev_gas_2: f32,
    prev_gas_3: f32,
    high_bids_count: i32,
    avg_bid_price: f32,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = reqwest::Client::new();
    let url = "http://localhost:3000/predict";
    
    let request = GasPredictRequest {
        hour: 14,
        day_of_week: 2,
        prev_gas_1: 45.5,
        prev_gas_2: 42.3,
        prev_gas_3: 40.1,
        high_bids_count: 25,
        avg_bid_price: 47.2,
    };

    // Single request
    println!("Single request test:");
    let resp = client.post(url).json(&request).send().await?;
    println!("Response: {}", resp.text().await?);

    // 1000 concurrent requests
    println!("\n1000 concurrent requests test:");
    let start = Instant::now();
    
    let futures: Vec<_> = (0..1000)
        .map(|_| {
            let client = client.clone();
            let request = request.clone();
            let url = url.to_string();
            async move {
                client.post(&url).json(&request).send().await
            }
        })
        .collect();

    let results = join_all(futures).await;
    let success_count = results.iter().filter(|r| r.is_ok()).count();
    
    let duration = start.elapsed();
    println!("Completed {} requests in {:?}", success_count, duration);
    println!("Requests per second: {:.2}", 1000.0 / duration.as_secs_f64());
    
    Ok(())
}