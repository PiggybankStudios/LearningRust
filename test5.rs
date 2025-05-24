
fn get_highest<'a, 'b>(first_number: &'a i8, second_number: &'b i8) -> &'a i8
{
	if first_number > second_number { return first_number; }
	else { return &0; } //NOTE: How does this get the 'a lifetime?
}

fn main()
{
	let foo: i8 = 42;
	let outcome: &i8;
	{
		let bar: i8 = 7;
		outcome = get_highest(&foo, &bar);
	}
	println!("{}", outcome);
}