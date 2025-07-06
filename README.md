# RustServingXgboost
Serve ML model with Rust Axum and test inferences with Rust, after training with Python, using ONNX model format.

# ETH Gas Price Prediction - Rust ML Inference Demo

This demo shows how to train an XGBoost model in Python, export it to ONNX, and serve it with Rust/Axum for high-performance inference.

## Prerequisites

- Docker
- Rust toolchain (`curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh`)
- Just command runner (`cargo install just`)

## Quick Start

```bash
# Run the entire pipeline
just all
```

This will:
1. Build Docker images for training and serving
2. Train an XGBoost model to predict ETH gas prices
3. Export the model to ONNX format
4. Start the Rust inference server
5. Run single request test
6. Pause for you to press Enter
7. Run concurrent (1000 requests) test

## Demo Steps

```bash
# 1. Setup and train the model
just build-train build-serve # 1-3min and 3-7min respectively to build the 2 images the first time
just train

# 2. Start the inference server
just serve

# 3. Test single request performance
just test-single

# 4. Test concurrent request performance (1000 requests)
just test-concurrent
```

## Manual Steps

```bash
# Build images
just build-train
just build-serve

# Train model
just train

# Start server
just serve

# Run tests separately
just test-single     # Single request test
just test-concurrent # 1000 concurrent requests

# Clean up
just clean
```

## API

POST `/predict`

Request:
```json
{
    "hour": 14,
    "day_of_week": 2,
    "prev_gas_1": 45.5,
    "prev_gas_2": 42.3,
    "prev_gas_3": 40.1,
    "high_bids_count": 25,
    "avg_bid_price": 47.2
}
```

Response:
```json
{
    "predicted_gas_price": 48.7
}
```

## Features

- **hour**: Hour of day (0-23)
- **day_of_week**: Day of week (0-6, Sunday=0)
- **prev_gas_1**: Previous block gas price (gwei)
- **prev_gas_2**: Gas price 2 blocks ago (gwei)
- **prev_gas_3**: Gas price 3 blocks ago (gwei)  
- **high_bids_count**: Count of bids >2x previous price
- **avg_bid_price**: Average bid price in pool (gwei)