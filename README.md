# NTUST Query Course Library

## Example

[All examples](./examples/)

### Simple

```rust
use q::Q;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let c = Q::new();

    let details = c
        .query("1122", "AT2005701", "en")
        .await
        .expect("failed to query course description");

    println!("{:#?}", details);

    Ok(())
}
```
