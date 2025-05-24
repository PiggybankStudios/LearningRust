
fn write_line(line: &str)
{
	println!("{}", line);
}

#[allow(unused_variables)]
#[allow(unused_mut)]
fn main()
{
	let result = 1.0f32 + 2.2;
	write_line(&"Hello World!");
	
	// Arrays are fixed size and are stored on the stack
	let mut my_array: [u32; 3] = [1, 42, 3141592];
	println!("array has {} elements:", my_array.len());
	for (n_index, num) in my_array.iter().enumerate()
	{
		println!("\t[{}] {}", n_index, num);
	}
	my_array[1] = 69;
	println!("array has {} elements:", my_array.len());
	for (n_index, num) in my_array.iter().enumerate()
	{
		println!("\t[{}] {}", n_index, num);
	}
	
	//Vectors are variable length and are stored on the heap
	let mut my_vector: Vec<&str> = vec!["one", "two", "three"];
	println!("Vector has {} elements", my_vector.len());
	my_vector.push("Hello!");
	println!("Vector has {} elements", my_vector.len());
	for (s_index, string) in my_vector.iter().enumerate()
	{
		println!("\t[{}] {}", s_index, string);
	}
}