use serde_json::{json, Value};
use std::collections::HashMap;
use std::fmt::Error;

fn main() {
    let result = json!({
        "beep": "boop",
        "fleep": "floop",
        "foo": "bar"
    });
    println!("Original:");
    println!("    {:?}", result);
    println!("    {}", type_of(&result));

    let result = initialise_simple_hash(result).unwrap();

    println!("Converted:");
    println!("    {:?}", result);
    println!("    {}", type_of(&result));
}

// only takes json of the form { string: string, ... }
// Could be extended but this loses the flexibility of serde_json.... cannot have mixed value
// types in core Rust types.
fn initialise_simple_hash(json: Value) -> Result<HashMap<String, String>, Error> {
    let mut out: HashMap<String, String> = HashMap::new();
    if json.is_object() {
        let json = json.as_object().ok_or("JSON was not an Object");
        let json = {
            let mut temp: HashMap<String, String> = HashMap::new();
            for (key, value) in json.unwrap() {
                temp.insert(
                    key.into(),
                    value
                        .as_str()
                        .ok_or("Value is not a String")
                        .unwrap()
                        .into(),
                );
            }
            temp
        };
        for (key, value) in json {
            out.insert(key, value);
        }
    }
    Ok(out)
}

fn type_of<T>(_: &T) -> String {
    format!("{}", std::any::type_name::<T>())
}
