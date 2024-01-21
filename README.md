A new branch where the clients send the signed messages to the aggregator. And the aggregator validates the signatures and then computes the average of averages.

-- There is an error at src/bin/main.rs:53:47
"no function or associated item named `generate` found for struct `SigningKey` in the current scope"

There is no implementation error according to the documentation in: https://docs.rs/ed25519-dalek/latest/ed25519_dalek cannot resolve the error for now. 


