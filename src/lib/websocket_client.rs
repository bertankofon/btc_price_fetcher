use tokio_tungstenite::{connect_async, tungstenite::protocol::Message};
use futures_util::StreamExt;
use serde_json::Value;
use std::fs::File;
use std::io::Write;
use std::time::{Duration, Instant};

pub async fn cache_mode(times: u64) {
    let url = "wss://stream.binance.com:9443/ws/btcusdt@trade";

    let (mut socket, _) = connect_async(url)
        .await
        .expect("Failed to connect to WebSocket");

    let mut prices = Vec::new();
    let start_time = Instant::now();

    while Instant::now().duration_since(start_time) < Duration::from_secs(times) {
        if let Some(message) = socket.next().await {
            if let Ok(Message::Text(text)) = message {
                if let Ok(data) = serde_json::from_str::<Value>(&text) {
                    if let Some(price_str) = data["p"].as_str() {
                        if let Ok(price) = price_str.parse::<f64>() {
                            prices.push(price);
                        }
                    }
                }
            }
        }
    }

    let average_price: f64 = prices.iter().sum::<f64>() / prices.len() as f64;
    println!("Cache complete. The average USD price of BTC is: {:.2}", average_price);

    let mut file = File::create("btc_prices.txt").expect("Failed to create file");
    writeln!(file, "Average price: {:.2}", average_price).expect("Failed to write to file");
    for price in &prices {
        writeln!(file, "{:.2}", price).expect("Failed to write to file");
    }
}
