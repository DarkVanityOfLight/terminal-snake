extern crate termion;
use std::time::Duration;
use std::thread::sleep;
use std::iter;
use std::io::{self, stdout, Read};
use termion::raw::IntoRawMode; 

pub struct Snake{
    head_position: [usize; 2],
    body: Vec<[usize; 2]>,
}

pub struct Field{
    size: [usize; 2],
    player: *const Snake,
    current_food: Food,
}

pub struct Food{
    position: [usize; 2],

}

impl Field{

    fn new(size: [usize; 2], player: *const Snake, starting_food: Food) -> Field{
        Field{
            size: size,
            player: player,
            current_food: starting_food,
        }
    }

    fn draw(& self){
        let repeated: String = iter::repeat("#").take(self.size[1] + 2).collect();
        print!("{}\r\n", repeated);
        for row in 0..self.size[0]{
            print!("#");
            for column in 0..self.size[1]{
                let current_position = [row, column];
                unsafe {
                    if (*self.player).body.contains(&current_position){
                        print!("+");
                    }else if (*self.player).head_position == current_position{
                        print!("=");
                    }
                    else{
                        print!(" ");
                    }
                }
            }
            print!("#\r\n");
        }
        print!("{}\r\n", repeated);
        print!("{}\r\n", self.size[0]);
        print!("{}\r\n", self.size[1]);


    }
}

impl Snake{

    fn new(head_position: [usize; 2]) -> Snake{
        Snake{
            head_position: head_position,
            body: Vec::new(),
        }
    }

    fn mov(&mut self, direction: &str){
       
        if direction == "UP"{
            self.head_position[1] += 1;
        }else if direction == "DOWN"{
            self.head_position[1] -= 1;
        }else if direction == "LEFT"{
            self.head_position[0] -= 1;
        }else if direction == "RIGHT"{
            self.head_position[0] += 1;
        }

    }
}

fn main() {
}
