# q

NTUST Query Course Library

## Example

[All examples](./examples/)

### Simple

```rust
use q::Q;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let c = Q::new();

    let details = c
        .query("1131", "TCG046301", q::Language::Zh)
        .await
        .expect("failed to query course description");

    println!("{:#?}", details);

    Ok(())
}

```
