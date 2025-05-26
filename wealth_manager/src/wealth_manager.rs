
use serde_json::{json, Value as jsValue};
/*
Documentation of serde_json::Value can be found at:
 https://docs.rs/serde_json/1.0.64/serde_json/enum.Value.html
pub enum Value {
    Null,
    Bool(bool),
    Number(Number),
    String(String),
    Array(Vec<Value>),
    Object(Map<String, Value>),
}
*/

fn main()
{
    let stock: jsValue = json!({
    	"name": "MonolithAI",
    	"price": 43.7,
    	"history": [19.4, 26.9, 32.5],
    });
    println!("first price: {}", stock["history"][0]);
    println!("json: {}", stock.to_string());
}
