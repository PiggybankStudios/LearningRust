
//This exposes the entire stock.rs file contents to the public module interface
//NOTE: This introduces stock:: into the module path for things inside stock.rs,
// so we'll end up with stock::Stock at the end of our type path for the Stock structure
pub mod stock;

// Expose the utils file to other files in this module, not as part of the public interface
mod utils;
