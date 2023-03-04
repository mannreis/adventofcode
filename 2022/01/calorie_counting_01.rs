use std::io;

fn main() {
    let lines = std::io::stdin().lines();
    
    let mut counter : u64 = 0;
    let mut result : u64 = 0;
    for l in lines {
        let l = l.unwrap();

        if l.is_empty() {
            //reset counter
            counter = 0;
        }else{
            counter+=l.parse::<u64>().unwrap();
            if counter > result { result = counter;}
        }
    }
    println!("Result: {}",result);
}
