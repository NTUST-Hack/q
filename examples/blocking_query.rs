use q::blocking::Q;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let c = Q::new();

    let details = c
        .query("1122", "AT2005701", q::Language::En)
        .expect("failed to query course description");

    println!("{:#?}", details);

    Ok(())
}
