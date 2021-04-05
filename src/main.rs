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
    position: [i32; 2],

}

impl Field{

    fn new(size: [i32; 2], player: Snake, starting_food: Food) -> Field{
        Field{
            size: size,
            player: player,
            current_food: starting_food,
        }
    }

    fn draw(& self){
        println!("{}", self.size[0]);
        println!("{}", self.size[1]);

    }
}

fn main() {
}
