use j9;

fn main() -> anyhow::Result<()> {
    let json_str = r#"{ "number": 1 }"#;
    let jq = j9::run(".", json_str)?;
    println!("{:?}", jq);
    Ok(())
}
