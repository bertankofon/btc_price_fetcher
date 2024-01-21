use tokio::sync::mpsc;
use tokio_tungstenite::{connect_async, tungstenite::protocol::Message};
use futures_util::StreamExt;
use serde_json::Value;
use std::time::{Duration, Instant};
use ed25519_dalek::{SigningKey, VerifyingKey, Signer}; // Updated imports


pub async fn client_task(id: u8, tx: mpsc::Sender<(f64, Vec<u8>, VerifyingKey)>, signing_key: SigningKey) {
    // Define the WebSocket URL
    let url = "wss://stream.binance.com:9443/ws/btcusdt@trade";

    // Connect to the WebSocket server
    let (mut ws_stream, _) = connect_async(url)
        .await
        .expect("Failed to connect to WebSocket");

    println!("Client {} connected to the WebSocket", id);

    // Initialize a vector to store prices
    let mut prices = Vec::new();
    let start_time = Instant::now();
    let duration = Duration::from_secs(10); // Clients will listen for 10 seconds

    // Read messages from the WebSocket for a fixed duration
    while Instant::now().duration_since(start_time) < duration {
        if let Some(message) = ws_stream.next().await {
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

    // Calculate the average price and sign it
    if !prices.is_empty() {
        let average_price: f64 = prices.iter().sum::<f64>() / prices.len() as f64;
        let message = average_price.to_be_bytes(); // Convert the f64 to bytes for signing
        let signature = signing_key.sign(&message); // Updated signing code

        // Get the verifying key from the signing key
        let verifying_key: VerifyingKey = signing_key.verifying_key();

        // Send the average price along with the signature and verifying key
        tx.send((average_price, signature.to_bytes().to_vec(), verifying_key)).await.expect("Failed to send average price and signature");
        println!("Client {} sent average price and signature to aggregator", id);
    } else {
        eprintln!("Client {} did not receive any prices", id);
    }
}