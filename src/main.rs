extern crate termion;
use std::time::Duration;
use std::thread::sleep;
use std::iter;
use std::io::{self, stdout, Read};
use termion::raw::IntoRawMode; 
use termion::raw::RawTerminal;
use std::convert::TryFrom;

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
        let repeated: String = iter::repeat("#").take(self.size[0] + 2).collect();
        print!("{}\r\n", repeated);
        for y in 0..self.size[1]{
            print!("#");
            for x in 0..self.size[0]{
                let current_position = [x, y];
                unsafe {
                    if (*self.player).body.contains(&current_position){
                        print!("+");
                    }else if (*self.player).head_position == current_position{
                        print!("=");
                    }else if self.current_food.position == current_position{
                        print!("*")
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

    fn mov(&mut self, direction: &str, size_x: usize, size_y: usize) -> bool{
        let head_x = i64::try_from(self.head_position[0]).unwrap();
        let head_y = i64::try_from(self.head_position[1]).unwrap();
        if direction == "UP"{
            if head_y != 0{
                self.head_position[1] -= 1;
                return true
            }
        }else if direction == "DOWN"{
            if self.head_position[1] + 1< size_y{
                self.head_position[1] += 1;
                return true
            }
        }else if direction == "LEFT"{
            if head_x != 0{
                self.head_position[0] -= 1;
                return true
            }
        }else if direction == "RIGHT"{
            if self.head_position[0] + 1 < size_x{
                self.head_position[0] += 1;
                return true
            }
        }

        return false

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
    }else if action == "QUIT" || action == "LOOSE"{
        return true
    }else{
        return false
    }
}

fn to_ctrl_byte(c: char) -> u8{
    let byte = c as u8;
    byte & 0b0001_1111

}

fn input_handler(player: *mut Snake, field: &Field, mut last_action: &mut String){

    let k = io::stdin().bytes().next().unwrap();
    let b  = k.unwrap();
    
    let pressed_key = b as char;
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
    }else if b == to_ctrl_byte('e'){
        action = "QUIT";
        *last_action = action.to_string();
        return
    }else{
        action = last_action;
    }


    if !valid_action(&action, &last_action){
        action = last_action;
    }

    let result: bool;
    unsafe {
        result = (*player).mov(&action, field.size[0], field.size[1]);
    }
    if !result{
        action = "LOOSE";
    }
    *last_action = action.to_string();

}

fn check_exit<T: std::io::Write>(last_action: &str, _stdout: &RawTerminal<T>){
    if last_action == "QUIT" || last_action == "LOOSE"{
        exit(_stdout);
    }
}

fn exit<T: std::io::Write>(_stdout: &RawTerminal<T>){
    _stdout.suspend_raw_mode();
    std::process::exit(0);
}

fn main() {
}
