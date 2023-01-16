use serde_json::{json, Value};

#[test]
fn simple_test() {
    println!("Hello, world!");
    let v = json!("a string");
    //let result = serde_json::from_str::<serde_json::Value>(r#" {"param": "1"} "#).unwrap();
    let result = serde_json::from_str::<serde_json::Value>(r#" 1  "#).unwrap();
    println!("result: {:?}", result);
}
