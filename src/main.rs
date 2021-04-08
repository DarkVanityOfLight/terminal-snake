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
        for x in 0..self.size[0]{
            print!("#");
            for y in 0..self.size[1]{
                let current_position = [x, y];
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
        unsafe {
            print!("{}\r\n", (*self.player).head_position[0]);
            print!("{}\r\n", (*self.player).head_position[1]);
        }

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
            self.head_position[0] -= 1;
        }else if direction == "DOWN"{
            self.head_position[0] += 1;
        }else if direction == "LEFT"{
            self.head_position[1] -= 1;
        }else if direction == "RIGHT"{
            self.head_position[1] += 1;
        }

    }
}

fn valid_action(action: &str, last_action: &str) -> bool{
    if action == "UP"{
        return last_action != "DOWN"
    }else if action == "DOWN"{
        return last_action != "UP"
    }else if action == "LEFT"{
        return last_action != "RIGHT"
    }else if action == "RIGHT"{
        return last_action != "LEFT"
    }else{
        return false
    }
}

fn input_handler(player: *mut Snake, mut last_action: &mut String){

    let k = io::stdin().bytes().next().unwrap();
        
    let pressed_key = k.unwrap() as char;
    let mut action;

    // Check what we direction have to go
    if pressed_key == 'w'{
        action = "UP";
    }else if pressed_key == 's'{
        action = "DOWN";
    }else if pressed_key == 'a'{
        action = "LEFT";
    }else if pressed_key == 'd'{
        action = "RIGHT";
    }else{
        action = last_action;
    }


    if !valid_action(&action, &last_action){
        action = last_action;
    }

    unsafe {
        (*player).mov(&action);
    }
    *last_action = action.to_string();

}

fn main() {
}
