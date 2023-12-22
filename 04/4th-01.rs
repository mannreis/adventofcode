use std::io;
use std::collections::HashSet;

fn main(){
    let lines = std::io::stdin().lines();
    let mut result = 0;
    for line in lines {
        let line = line.unwrap();

        let mut winning_set = HashSet::<u32>::new();
        let (c_number,card) = line.split_once(':').unwrap();
        let (w_numbers, numbers) = card.split_once("|").unwrap();
        println!("{} {} > {} ",c_number, w_numbers,numbers);

        for w_number in w_numbers.split_whitespace() {
            winning_set.insert(w_number.parse::<u32>().unwrap());
        }
        println!("{:?}",winning_set);

        let mut points = 0;
        for number in numbers.split_whitespace() {
            //increase or double card points
            if winning_set.contains(&number.parse::<u32>().unwrap()){
                points = if points == 0 { 1 }else{points*2};
                println!("\tcard contains winning number: {} ({} points)", number, points);
            }
        }
        result += points;
    }
    println!("Result: {}", result);
}
