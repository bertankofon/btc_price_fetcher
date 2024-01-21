Rust project for Supra Oracles that fetches BTC/USDT prices using Binance API

There are 3 modes: Cache, Read, Simulate

cargo run -- --mode=cache --times=X (Caches the price for a given time(seconds) and write it to a new file named btc_price.txt, prints the average price to the terminal, it is 10 seconds in default if no input X is given)

cargo run -- --mode=read (Reads the .txt file and prints it to the terminal)

cargo run -- --mode=simulate (starts 5 client process simultaneously and calculates+prints the average of the average BTC/USDT price)

