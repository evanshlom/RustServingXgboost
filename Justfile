# Build training image
build-train:
    docker build -t eth-gas-train ./train

# Build serving image
build-serve:
    docker build -t eth-gas-serve ./serve

# Train model
train:
    mkdir -p model
    docker run -v $(pwd)/model:/model eth-gas-train

# Run server
serve:
    docker run -d --name eth-gas-server -p 3000:3000 -v $(pwd)/model:/model eth-gas-serve

# Stop server
stop:
    docker stop eth-gas-server
    docker rm eth-gas-server

# Single inference test
test-single:
    curl -X POST http://localhost:3000/predict \
        -H "Content-Type: application/json" \
        -d '{"hour": 14, "day_of_week": 2, "prev_gas_1": 45.5, "prev_gas_2": 42.3, "prev_gas_3": 40.1, "high_bids_count": 25, "avg_bid_price": 47.2}'

# Build and run Rust test client
build-test-client:
    cd test_endpoint && cargo build --release

test-concurrent-rust: build-test-client
    ./test_endpoint/target/release/test-client

# Full pipeline
all: build-train build-serve train serve build-test-client
    @echo "Waiting for server to start..."
    @sleep 3
    @echo "Running Rust test client..."
    @just test-concurrent-rust

# Clean up
clean: stop
    rm -rf model
    docker rmi eth-gas-train eth-gas-serve