#![allow(unused_parens)]
#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_snake_case)]

// Because enumerations can contain any struct type AND they can act like C enums with integer designators, I wanted to see what would happen if you combine both
#[repr(i32)]
enum MyEnum
{
	UnitValue,
	IntValue = 4, //designator!
	TupleValue (i32, i32),
	StructValue {x: i32, y: i32},
}

fn main()
{
	let varUnit = MyEnum::UnitValue;
	let varInt = MyEnum::IntValue;
	let varTuple = MyEnum::TupleValue(42, 69);
	let varStruct = MyEnum::StructValue{x: 8, y: 7};
	match varTuple
	{
		MyEnum::TupleValue(a, b) => println!("TupleValue: {}, {}", a, b),
		// MyEnum::IntValue => println!("IntValue: {}", varInt as i32), //still can't cast MyEnum as i32 because not all values have a designator
		_ => todo!(),
	}
	// println!("UnitValue: {}", varUnit as i32);
	// println!("IntValue: {}", varInt as i32);
	// println!("TupleValue: {}", varTuple as i32);
	// println!("StructValue: {}", varStruct as i32);
}
