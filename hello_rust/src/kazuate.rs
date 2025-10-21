use rand::Rng;
use rand::thread_rng;
use rand::prelude::SliceRandom;
use std::io::{self, stdin, Write};
//use std::io::{self, Write};

pub fn run(){
    //let secret = rand::thread_rng().gen_range(1..100);
    //let secrets = rand::seq::SliceRandom(1..100);
    let mut numbers:Vec<_> = (1..10).collect();
    numbers.shuffle(&mut thread_rng());
    println!("{:?}", &numbers[..4]);
    let mut flag = true;

    while flag{
        let mut input = String::new();
        print!("input string: ");
        io::stdout().flush().unwrap();
        stdin().read_line(&mut input).expect("failed to read line");

        println!("you typed {}", input);
        if input.trim()=="quit"{
            flag=false;
        }

        input = input.trim().split_whitespace()
    }
}