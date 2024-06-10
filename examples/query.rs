use q::Q;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let c = Q::new();

    let details = c
        .query("1131", "TCG046301", q::Language::Zh)
        .await
        .expect("failed to query course description");

    println!("{:#?}", details);

    Ok(())
}
