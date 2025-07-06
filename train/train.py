import numpy as np
import xgboost as xgb
from sklearn.model_selection import train_test_split
from sklearn.metrics import r2_score, mean_absolute_error
from skl2onnx import convert_sklearn
from skl2onnx.common.data_types import FloatTensorType
from data import generate_eth_gas_data

# Generate data
df = generate_eth_gas_data(n_samples=10000)

# Features and target
feature_cols = ['hour', 'day_of_week', 'prev_gas_1', 'prev_gas_2', 
                'prev_gas_3', 'high_bids_count', 'avg_bid_price']
X = df[feature_cols].values
y = df['target_gas_price'].values

# Split data
X_train, X_test, y_train, y_test = train_test_split(X, y, test_size=0.2, random_state=42)

# Train XGBoost model
model = xgb.XGBRegressor(
    n_estimators=100,
    max_depth=6,
    learning_rate=0.1,
    random_state=42
)
model.fit(X_train, y_train)

# Evaluate
y_pred = model.predict(X_test)
r2 = r2_score(y_test, y_pred)
mae = mean_absolute_error(y_test, y_pred)

print(f"Model Performance:")
print(f"R² Score: {r2:.4f}")
print(f"MAE: {mae:.2f} gwei")

# Convert to ONNX
initial_type = [('float_input', FloatTensorType([None, 7]))]
onnx_model = convert_sklearn(model, initial_types=initial_type, target_opset=12)

# Save ONNX model
with open('/model/model.onnx', 'wb') as f:
    f.write(onnx_model.SerializeToString())

print("\nModel saved to ONNX format")
print(f"Features: {feature_cols}")