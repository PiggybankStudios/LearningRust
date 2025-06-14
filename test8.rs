
// #[allow(dead_code)]

macro_rules! capitalize
{
	($a: expr) =>
	{
		let mut v: Vec<char> = $a.chars().collect();
		v[0] = v[0].to_uppercase().nth(0).unwrap();
		$a = v.into_iter().collect();
	}
}

#[allow(unused_variables)]
#[allow(unused_mut)]
fn main()
{
	let mut x = String::from("test");
	capitalize!(x);
	println!("{}", x);
}