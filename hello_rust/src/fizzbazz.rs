pub fn run(){
    let fizz = "fizz";
    let buzz = "buzz";
    let cnt = 30;
    for i in 0..cnt{
        if i%3==0 &&i%5 == 0{
            println!("{}, {}{}", i, fizz,buzz); 
        }
        else if i%3 == 0 {
            println!("{}, {}", i, fizz);
        }else if i%5 == 0{
            println!("{}, {}", i, buzz); 
        }
    }
}