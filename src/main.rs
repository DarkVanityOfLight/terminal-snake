use std::time::Duration;
use std::thread::sleep;
use std::iter;

pub struct Snake{
    head_position: [i32; 2],
    body: Vec<[i32; 2]>,
}

pub struct Field{
    size: [usize; 2],
    player: Snake,
    current_food: Food,
}

pub struct Food{
    position: [i32; 2],

}

impl Field{

    fn new(size: [usize; 2], player: Snake, starting_food: Food) -> Field{
        Field{
            size: size,
            player: player,
            current_food: starting_food,
        }
    }

    fn draw(& self){
        let repeated: String = iter::repeat("#").take(self.size[1] + 2).collect();
        println!("{}", repeated);
        for row in 0..self.size[0]{
            print!("#");
            for column in 0..self.size[1]{
                print!(" ")
            }
            println!("#");
        }
        println!("{}", repeated);
        println!("{}", self.size[0]);
        println!("{}", self.size[1]);

    }
}

fn main() {
}
