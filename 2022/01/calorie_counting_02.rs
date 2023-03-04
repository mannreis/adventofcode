use std::io;

fn main() {
    let lines = std::io::stdin().lines();
    let (mut first, mut second, mut third) : (u64,u64,u64) = (0,0,0); 
    let mut counter : u64 = 0;
    for l in lines {
        let l = l.unwrap();

        if l.is_empty() {
            //reset counter
            if counter >= first { 
                third = second;
                second = first;
                first = counter;
            }else if counter >= second {
                third = second;
                second = counter;
            }else if counter > third {
                third = counter;
            }
            counter = 0;
        }else{
            counter+=l.parse::<u64>().unwrap();
        }
    }

            if counter >= first {
                third = second;
                second = first;
                first = counter;
            }else if counter >= second {
                third = second;
                second = counter;
            }else if counter > third {
                third = counter;
            }
    println!("Result: {} ({},{},{})",first+second+third,first,second,third);
}
