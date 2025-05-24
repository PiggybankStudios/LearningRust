//This file was written with the help of Github Copilot using the LSP-copilot package in SublimeText

/// This program asks the user for their name and checks if it is valid according to certain rules.
#[allow(unused_parens)]
fn check_name(name: &str) -> bool
{
	if name.is_empty() { println!("You didn't enter a name!"); return false; }
	if name.chars().any(|c| matches!(c, '0'..='9')) { println!("Numbers are not allowed in names!"); return false; }
	if name.chars().any(|c| ['!', '@', '#', '$', '%', '^', '&', '*', '(', ')'].contains(&c)) { println!("Symbols are not allowed in names!"); return false; }
	if name.contains(" ") { println!("Spaces are not allowed in names!"); return false; }
	if name.len() > 20 { println!("Name is too long!"); return false; }
	return true;
}

/// This is the main function that runs the program.
fn main()
{
	println!("Hello World!!");
	println!("Please enter your name: ");
	let mut name = String::new();
	std::io::stdin().read_line(&mut name).expect("Failed to read line");
	if check_name(&name)
	{
		println!("Hello {}", name.trim());
	}
}