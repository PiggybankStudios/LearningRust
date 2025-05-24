
#[allow(dead_code)]
#[derive(Copy, Clone)]
struct Vector2<T>
{
	x: T,
	y: T,
}

#[allow(unused_variables)]
#[allow(unused_mut)]
fn main()
{
	let zero = Vector2{x: 0, y: 0}; //NOTE: this is a integer type vector
	let one = Vector2{x: 1.0, y: 1.0}; //NOTE: This is a float type vector
	let string_vec = Vector2{x: "Hello".to_string(), y: "World".to_string()};
}