use std::time::Duration;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Use a custom reqwest client builder
    let mut reqwest_client_builder = q::default_reqwest_builder();

    // Set a timeout of 10 seconds
    reqwest_client_builder = reqwest_client_builder.timeout(Duration::from_secs(10));

    // Build the reqwest client
    let reqwest_client = reqwest_client_builder
        .build()
        .expect("failed to build reqwest client");

    // Create a new client with the custom reqwest client
    let c = q::ClientBuilder::new()
        .reqwest_client(reqwest_client)
        .build();

    // Query course details
    let details = c
        .query("1131", "TCG046301", q::Language::Zh)
        .await
        .expect("failed to query course description");

    println!("{:#?}", details);

    Ok(())
}
