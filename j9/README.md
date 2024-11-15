# j9

`j9` provides a high-level API to run jq programs from Rust code,
simplifying the execution of jq filters on JSON data.

## Usage

To use j9, add it as a dependency in your Cargo.toml:

```toml
[dependencies]
j9 = "0.1.4"
```

## Example

```rust
use j9;

fn main() -> anyhow::Result<()> {
    let json_str = r#"{ "number": 1 }"#;
    let jq = j9::run(".", json_str)?;
    println!("{:?}", jq);
    Ok(())
}
```

This example runs the simplest jq program (.)
which outputs the input JSON unchanged.
For more complex jq programs,
simply replace "." with your jq filter.
