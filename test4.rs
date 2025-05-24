
fn function_requiring_copied_int(input: i32)
{
	println!("Direct value: \"{}\"", input);
}
fn function_requiring_borrowed_int(input: &i32)
{
	println!("Borrowed value: \"{}\"", input);
}

fn function_requiring_moved_str(input: String)
{
	println!("Direct value: \"{}\"", input);
}
fn function_requiring_borrowed_str(input: &String)
{
	println!("Borrowed value: \"{}\"", input);
}

fn main()
{
	let int1: i32 = 42;
	let int2: i32 = 128;
	function_requiring_copied_int(int1);
	function_requiring_borrowed_int(&int2);
	println!("int1: \"{}\"", int1); //This does not cause an error because i32 implements the Copy trait!
	println!("int2: \"{}\"", int2);
	
	let string1 = String::from("This is a test!");
	let string2 = String::from("This is a drill!");
	function_requiring_moved_str(string1);
	function_requiring_borrowed_str(&string2);
	// println!("string1: \"{}\"", string1); //This causes a use after move error during compile!
	println!("string2: \"{}\"", string2);
	
	let hello = String::from("Hello");
	// String does not implement the Copy trait, so using it in this expression would result in a Move if we don't call .to_owned()
	let hello_world = hello.to_owned() + " World!";
	// let hello_world = hello.clone() + " World!"; //https://www.reddit.com/r/rust/comments/l5uih4/what_is_the_difference_between_clone_and_to_owned/
	println!("\"{}\"", hello); //this line will break without the .to_owned() above!
	println!("\"{}\"", hello_world);
}