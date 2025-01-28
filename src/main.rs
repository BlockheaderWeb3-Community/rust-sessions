use signed_operation::intro_to_i;
use unsigned_operation::intro_to_u;

pub mod signed_operation;
pub mod string_operations;
pub mod unsigned_operation;
pub mod utils;

fn main() {
    intro_to_u(); // Introduction to unsigned integer operations
    intro_to_i(); // Introduction to signed integer operations
}
