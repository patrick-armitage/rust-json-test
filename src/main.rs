use serde::{Deserialize, Serialize};
use serde_json::Result;

#[derive(Serialize, Deserialize)]
struct Person {
    name: String,
    age: u8,
    phones: Vec<String>,
}

fn main() -> Result<()> {
    // Some JSON input data as a &str.
    let data = r#"
        {
            "name": "John Doe",
            "age": 43,
            "phones": [
                "+44 1234567",
                "+44 2345678"
            ]
        }"#;

    let struct_example: Person = serde_json::from_str(data)?;
    let value_example: serde_json::Value = serde_json::from_str(data).unwrap_or(serde_json::Value::Null);

    println!("Struct example of name {} and number {}", struct_example.name, struct_example.phones[0]);
    println!("Value example of name {} and number {}", value_example["name"], value_example["phones"][0]);

    Ok(())
}
