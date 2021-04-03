use std::time::Duration;
use std::thread::sleep;

pub struct Snake{
    head_position: [i32; 2],
    body: Vec<[i32; 2]>,
}

pub struct Field{
    size: [i32; 2],
    player: Snake,
    current_food: Food,
}

pub struct Food{
    position: [i32, 2],

}

fn main() {
}
