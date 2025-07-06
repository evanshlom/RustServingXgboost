import numpy as np
import pandas as pd

def generate_eth_gas_data(n_samples=10000):
    """Generate synthetic ETH gas price data with realistic patterns"""
    np.random.seed(42)
    
    # Features
    hours = np.random.randint(0, 24, n_samples)
    days = np.random.randint(0, 7, n_samples)
    
    # Base gas price pattern
    base_price = 30 + np.random.normal(0, 5, n_samples)
    
    # Hour patterns - higher during US/EU business hours
    hour_effect = np.where((hours >= 9) & (hours <= 17), 10, 0)
    hour_effect += np.where((hours >= 14) & (hours <= 22), 8, 0)  # EU overlap
    
    # Day patterns - lower on weekends
    day_effect = np.where(days >= 5, -8, 3)  # Weekend vs weekday
    
    # Previous block gas prices (simulate autocorrelation)
    prev_gas_1 = base_price + hour_effect + day_effect + np.random.normal(0, 3, n_samples)
    prev_gas_2 = prev_gas_1 * 0.9 + np.random.normal(0, 2, n_samples)
    prev_gas_3 = prev_gas_2 * 0.8 + np.random.normal(0, 2, n_samples)
    
    # Pool features
    # High bids count (2x previous price) - correlated with volatility
    volatility = np.abs(prev_gas_1 - prev_gas_2)
    high_bids_count = np.maximum(0, (volatility * 5 + np.random.normal(10, 5, n_samples))).astype(int)
    
    # Average bid price - slightly above previous
    avg_bid_price = prev_gas_1 * 1.05 + np.random.normal(0, 2, n_samples)
    
    # Target: next block gas price
    # Influenced by all features
    target = (
        base_price + 
        hour_effect * 0.8 + 
        day_effect * 0.7 +
        prev_gas_1 * 0.4 +
        prev_gas_2 * 0.2 +
        prev_gas_3 * 0.1 +
        high_bids_count * 0.15 +
        (avg_bid_price - prev_gas_1) * 0.3 +
        np.random.normal(0, 3, n_samples)
    )
    
    # Create DataFrame
    df = pd.DataFrame({
        'hour': hours,
        'day_of_week': days,
        'prev_gas_1': prev_gas_1,
        'prev_gas_2': prev_gas_2,
        'prev_gas_3': prev_gas_3,
        'high_bids_count': high_bids_count,
        'avg_bid_price': avg_bid_price,
        'target_gas_price': target
    })
    
    return df