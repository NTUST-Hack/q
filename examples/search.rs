use q::Q;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let c = Q::new();

    let mut options = q::SearchOptions::new("1131", q::Language::Zh);

    options.course_no = "cs".to_string();

    let details = c
        .search(&options, true)
        .await
        .expect("failed to search courses");

    println!("{:#?}", details);

    Ok(())
}
