use clap::{Command, Arg};
use btc_price_fetcher::websocket_client;
use btc_price_fetcher::file_handler;
use btc_price_fetcher::clients;
use btc_price_fetcher::aggregator;
use tokio::sync::mpsc;


#[tokio::main]
async fn main() {
   let matches = Command::new("Simple Client")
       .version("1.0")
       .author("Your Name")
       .about("Fetches and caches BTC prices")
       .arg(
           Arg::new("mode")
               .long("mode")
               .value_name("MODE")
               .help("Sets the mode of operation: cache or read or simulate")  // Updated help
       )
       .arg(
           Arg::new("times")
               .long("times")
               .value_name("TIMES")
               .help("Number of seconds to listen to the WebSocket in cache mode")
       )
       .get_matches();


   let binding = String::from("");
   let mode = matches.get_one::<String>("mode").unwrap_or(&binding);


   match mode.as_str() {
       "cache" => {
           let times = matches.get_one::<String>("times")
                             .unwrap_or(&"10".to_string())
                             .parse::<u64>()
                             .expect("Failed to parse times argument");
           websocket_client::cache_mode(times).await;
       },
       "read" => {
           file_handler::read_mode();
       },
       "simulate" => {
           // Prepare a channel for communication between clients and the aggregator
           let (tx, rx) = mpsc::channel(5); // Channel with a buffer size of 5


           // Spawn client tasks
           let mut handles = Vec::new();
           for i in 0..5 {
               let tx_clone = tx.clone();
               handles.push(tokio::spawn(async move {
                   clients::client_task(i, tx_clone).await;
               }));
           }


           // Spawn the aggregator task
           let aggregator_handle = tokio::spawn(async move {
               aggregator::aggregator_task(rx, 5).await;
           });


           // Wait for all tasks to complete
           let handles = futures_util::future::join_all(handles).await;


           // Now you can handle the results of each handle if necessary
           for handle in handles {
               match handle {
                   Ok(_) => println!("Client task completed successfully"),
                   Err(e) => eprintln!("Client task failed: {:?}", e),
               }
           }


           // Wait for the aggregator task to complete separately
           let _ = aggregator_handle.await.expect("Aggregator task failed");




       },
       _ => eprintln!("Invalid mode. Use --mode=cache, --mode=read, or --mode=simulate."),
   }
}
