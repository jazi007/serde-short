# Serde short derive

This crate provides a derive macro to derive Serde's `Serialize` and
`Deserialize` traits for C enum represented as short enum
`u8`, `u16`, `u32` or in case first value is < 0 `i8`, `i16` or `i32`

99% of the code is a copy/paste from [serde-repr](https://crates.io/crates/serde_repr) all credits goes to this crate

```toml
[dependencies]
serde = "1.0"
serde_short = "0.1"
```

```rust
use serde_short::{Serialize_short, Deserialize_short};

#[derive(Serialize_short, Deserialize_short, PartialEq, Debug)]
#[repr(u8)]
enum SmallPrime {
    Two = 2,
    Three = 3,
    Five = 5,
    Seven = 7,
}

fn main() -> serde_json::Result<()> {
    let j = serde_json::to_string(&SmallPrime::Seven)?;
    assert_eq!(j, "7");

    let p: SmallPrime = serde_json::from_str("2")?;
    assert_eq!(p, SmallPrime::Two);

    Ok(())
}
```

#### Credits

- [serde-repr](https://crates.io/crates/serde_repr)
