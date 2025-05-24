
use std::collections::HashMap;

enum StrOrInt
{
	MyStr(&'static str),
	MyInt(i32),
}

fn is_greater_than(threshold: i32, value_opt: Option<&StrOrInt>)
	-> Result<bool, &'static str>
{
	match value_opt
	{
		Some(value) => {
			match value
			{
				StrOrInt::MyInt(int_value) => {
					if int_value >= &threshold { return Ok(true); }
					else { return Ok(false); }
				}
				StrOrInt::MyStr(_) => { return Err("Value was a string not an integer!"); }
			}
		}
		None => { return Err("Value was None!") }
	}
}

#[allow(unused_variables)]
#[allow(unused_mut)]
fn main()
{
	println!("Enum example:");
	
	let mut map: HashMap<&str, StrOrInt> = HashMap::new();
	map.insert("one", StrOrInt::MyStr("1"));
	map.insert("fourty two", StrOrInt::MyInt(42));
	for (key, value) in &map
	{
		match value
		{
			StrOrInt::MyStr(str_value) => {
				println!("[\"{}\"] \"{}\"", key, str_value);
			}
			StrOrInt::MyInt(int_value) => {
				println!("[\"{}\"] {}", key, int_value);
			}
		}
	}
	
	let optional: Option<&StrOrInt> = map.get("test");
	match optional
	{
		Some(_) => { println!("\"test\" exists in the dictionary!"); }
		None => { println!("\"test\" does not exist in the dictionary!"); }
	}
	
	if is_greater_than(10, map.get("test")).expect("And error occurred!")
	{
		println!("Value is large!");
	}
	else
	{
		println!("Value is small!");
	}
}
