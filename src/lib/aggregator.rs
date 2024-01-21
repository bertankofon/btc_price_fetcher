use ed25519_dalek::VerifyingKey;
use tokio::sync::mpsc;
use std::collections::VecDeque;

pub async fn aggregator_task(mut rx: mpsc::Receiver<(f64, Vec<u8>, VerifyingKey)>, num_clients: usize) {
    let mut averages = VecDeque::with_capacity(num_clients);
    
    // Receive the tuples from the clients
    while let Some((average, _signature, _verifying_key)) = rx.recv().await {
        averages.push_back(average);
        
        // If we've received all client averages, compute the overall average
        if averages.len() == num_clients {
            let total_average: f64 = averages.iter().sum::<f64>() / num_clients as f64;
            println!("Overall average USD price of BTC is: {:.2}", total_average);
            break;
        }
    }
    
    // If a client task fails to send its average due to an error.
    if averages.len() != num_clients {
        eprintln!("Did not receive all client averages. Received {}/{}", averages.len(), num_clients);
    }
}
