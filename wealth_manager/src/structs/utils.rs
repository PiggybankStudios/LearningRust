
// This function is used from stock.rs through the declaration of this file in mod.rs
pub fn constructor_shout(stock_name: &str) -> ()
{
	println!("the constructor for {} is firing!", stock_name);
}