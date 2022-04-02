use bincode::serialize as to_bincode;
use serde_cbor::to_vec as to_cbor;
use serde_json::to_string as to_json;
use serde_derive::Serialize;

#[derive(Serialize)]
struct City {
    name: String,
    population: usize,
    latitude: f64,
    longitude: f64,
}

fn main() {
    let calibar = City {
        name: "Calibar".into(),
        population: 470_000,
        latitude: 4.95,
        longitude: 8.33
    };

    let as_json = to_json(&calibar).unwrap();
    let as_cbor = to_cbor(&calibar).unwrap();
    let as_binc = to_bincode(&calibar).unwrap();

    println!("json:    {}", &as_json);
    println!("cbor:    {:?}", &as_cbor);
    println!("bincode: {:?}", &as_binc);
    println!("json(utf-8)     {:?}", String::from_utf8_lossy(as_json.as_bytes()));
    println!("cbor(utf-8):    {:?}", String::from_utf8_lossy(&as_cbor));
    println!("bincode(utf-8): {:?}", String::from_utf8_lossy(&as_binc));
}