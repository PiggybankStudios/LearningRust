
trait CanTransfer
{
	fn transfer_stock(&self) -> ();
	fn print(&self) -> () { println!("A transfer is happening!"); }
}

struct Stock
{
	name: String,
	#[allow(dead_code)]
	open_price: f32,
	stop_loss: f32,
	take_profit: f32,
	current_price: f32,
}

impl Stock
{
	#[allow(dead_code)]
	fn new(stock_name: &str, price: f32) -> Stock
	{
		return Stock
		{
			name: String::from(stock_name),
			open_price: price,
			stop_loss: 0.0,
			take_profit: 0.0,
			current_price: price,
		};
	}
	
	fn with_stop_loss(mut self, value: f32) -> Stock { self.stop_loss = value; return self; }
	fn with_take_profit(mut self, value: f32) -> Stock { self.take_profit = value; return self; }
	
	fn update_price(&mut self, value: f32)
	{
		self.current_price = value;
	}
}

impl CanTransfer for Stock
{
	fn transfer_stock(&self)
	{
		println!("The stock {} is being transferred for ${}", self.name, self.current_price);
	}
}

fn process_transfer(stock: impl CanTransfer)
{
	stock.print();
	stock.transfer_stock();
}

#[allow(unused_variables)]
#[allow(unused_mut)]
fn main()
{
	let mut stock1 = Stock::new("MonolithAI", 95.0).with_stop_loss(55.0);
	let mut stock2 = Stock::new("RIMES", 150.4).with_stop_loss(55.0);
	let mut stock3 = Stock::new("BUMPER (former known as ASF)", 120.0).with_take_profit(100.0).with_stop_loss(50.0);
	stock1.update_price(128.4);
	println!("Here is the stock: {}", stock1.current_price);
	process_transfer(stock1);
}