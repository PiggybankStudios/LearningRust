//! This is the documentation comment for the wealth_manager cargo package!

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

mod structs;
use structs::stock::Stock;

/// Adds two numbers together
///
/// # Arguments
/// * a (i32): The first number
/// * b (i32): The second number
///
/// # Returns
/// (i32): The sum of `a` and `b`
///
/// # Example
/// ```rust
/// result: i32 = add_numbers(42, 69); //result = 111
/// ```
pub fn add_numbers(a: i32, b: i32) -> i32
{
	return a + b;
}

fn main()
{
    let stock: jsValue = json!({
    	"name": "MonolithAI",
    	"price": 43.7,
    	"history": [19.4, 26.9, 32.5],
    });
    println!("first price: {}", stock["history"][0]);
    println!("json: {}", stock.to_string());
    
    let stock: Stock = Stock::new("MonolithAI", 36.5);
    println!("here is the stock name: {}", stock.name);
    println!("here is the stock price: {}", stock.current_price);
}
