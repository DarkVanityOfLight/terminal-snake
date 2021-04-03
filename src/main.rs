use std::time::Duration;
use std::thread::sleep;

pub struct Snake{
    head_position: [i32; 2],
    body: Vec<[i32; 2]>,
    field: Field,
}

pub struct Field{
    size: [i32; 2]
}


fn main() {
}
