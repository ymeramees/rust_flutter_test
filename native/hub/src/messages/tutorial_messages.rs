use rinf::{DartSignal, RustSignal};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, DartSignal)]
pub struct MyPreciousData {
    pub input_numbers: Vec<i32>,
    pub input_string: String,
}

#[derive(Serialize, RustSignal)]
pub struct MyAmazingNumber {
    pub current_number: i32,
}
  