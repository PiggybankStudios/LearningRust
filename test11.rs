#![allow(unused_parens)]
#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_snake_case)]

// When the templated type is only in the return, then the type must be inferred by the context, if it can't we must employ "turbofish" syntax
fn template_return<T>(number: u8) -> T where T: From<u8>
{
	return T::from(number);
}

fn main()
{
	// Turbofish syntax is the ::<i32> part
	let num1 = "104".parse::<i32>().unwrap();
	let num2 = template_return::<i32>(42);
	println!("num1={:?} num2={:?}", num1, num2);
}