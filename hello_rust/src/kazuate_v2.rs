use rand::thread_rng;
use rand::prelude::SliceRandom;
use std::io::{self, stdin, Write};
use std::env::args;
use std::num::ParseIntError;

pub fn run(){
    let ans_numbers = generate_answer();
    let mut debug = args().any(|arg| arg == "debug");
    if debug{
        println!("{:?}", &ans_numbers);
    }

    let mut flag = true;

    println!("input numbers as 1 2 3 4");
    while flag{
        let numbers = match get_user_guess() {
            None => break,
            Some(Err(err)) => {
                println!("parse error: {err}");
                continue;
            }
            Some(Ok(nums)) => {
                println!("{:?}", nums);
                nums
            }
        };

        let byte = count_byte(&ans_numbers, &numbers);
        let eat = count_eat(&ans_numbers, &numbers);
        println!("{} byte, {} eat", byte, eat);

        if eat==4{
            println!("You Win!");
            flag=false;
        }
    }
}

fn generate_answer()->Vec<u32>{
    let mut nums: Vec<u32> = (0..10).collect();
    nums.shuffle(&mut thread_rng());
    nums.truncate(4);
    nums
}

fn get_user_guess()->Option<Result<Vec<u32>, ParseIntError>>{
    let mut input = String::new();
    print!("input numbers: ");
    io::stdout().flush().unwrap();
    stdin().read_line(&mut input).expect("failed to read line");

    println!("you typed {}", input);
    if input.trim()=="quit"{
        return None;
    }

    let tokens: Vec<_> = input.trim().split_whitespace().collect();
    let numbers: Result<Vec<u32>,_> = tokens.into_iter().map(|tok| tok.parse::<u32>()).collect();
    Some(numbers)
}

fn count_byte(ans_numbers: &Vec<u32>, numbers: &Vec<u32>) -> usize{
    ans_numbers
        .iter()
        .enumerate()
        .filter(|(idx, value)| numbers.iter().enumerate().any(|(j,v)| v == *value && j!= *idx))
        .count()
}

fn count_eat(ans_numbers: &Vec<u32>, numbers: &Vec<u32>) -> usize{
    ans_numbers 
        .iter()
        .zip(numbers.iter())
        .filter(|(a,g)| a==g)
        .count()
}