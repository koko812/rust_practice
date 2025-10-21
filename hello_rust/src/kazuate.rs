//use rand::Rng;
use rand::thread_rng;
use rand::prelude::SliceRandom;
use std::io::{self, stdin, Write};
//use std::io::{self, Write};
use std::env::args;

pub fn run(){
    //let secret = rand::thread_rng().gen_range(1..100);
    //let secrets = rand::seq::SliceRandom(1..100);
    let mut ans_numbers:Vec<_> = (1..10).collect();
    ans_numbers.shuffle(&mut thread_rng());
    ans_numbers.truncate(4);
    //args().skip(1);
    let mut debug = args().any(|arg| arg == "debug");
    if debug{
        println!("{:?}", &ans_numbers[..4]);
    }
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

        //let separated_input = input.trim().split_whitespace();
        let tokens: Vec<_> = input.trim().split_whitespace().collect();
        println!("{:?}", tokens);
        let numbers: Result<Vec<u32>,_> = tokens.into_iter().map(|tok| tok.parse::<u32>()).collect();
        let numbers = match numbers {
            Ok(nums) => nums,
            Err(err) => {
                println!("parse error: {err}");
                continue;
            }
        };
        println!("{:?}", numbers);

        let mut byte = 0;
        let mut eat = 0;

        // byte_check
        for (a_idx, a_val) in ans_numbers.iter().enumerate(){
            for (idx, val) in numbers.iter().enumerate(){
                if a_val==val && a_idx!=idx{
                    println!("{:?}, {:?}", a_val, val);
                    byte+=1;
                    break;
                }
            }
        }

        // eat_check
        for (idx, a_val) in ans_numbers.iter().enumerate(){
            if *a_val==numbers[idx]{
                eat+=1;
            }
        }

        println!("{} byte, {} eat", byte, eat);
        if eat==4{
            println!("You Win!");
            flag=false;
        }
    }
}