use scylla::transport::errors::NewSessionError;
use std::env;
use scylla::SessionBuilder;
use tracing::*;

fn main() -> Result<(), NewSessionError> {
    let mut rt = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .expect("Failed to spawn runtime");

    rt.block_on(async {
        tracing_subscriber::fmt::init();
        let uri = env::var("SCYLLA_URI").unwrap_or_else(|_| "127.0.0.1:9042".to_string());

        println!("Connecting...");
        let future = SessionBuilder::new()
            .known_node(uri)
            // .use_keyspace(keyspace.as_ref(), false)
            .build();

        let result = future.await;

        match result {
            Ok(_) => info!("Connected!"),
            Err(e) => panic!("Result returned error `{}` -- {:?}", e, e)
        }

        Ok(())
    })
}
