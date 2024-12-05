use std::io;
use std::collections::HashSet;

fn main(){
    let lines = std::io::stdin().lines();
    
    println!("{}",lines.size_hint().0);
    let mut result = 0;

    //let's hardcode the vector size
    let mut card_factor : [u32; 256] = [0;256];
    for line in lines {
        let line = line.unwrap();

        let mut winning_set = HashSet::<u32>::new();
        let (c_number,card) = line.split_once(':').unwrap();
        //parse card number
        let (_,c_number) = c_number.split_at(c_number.find(|c:char|c.is_digit(10)).unwrap());
        let c_number = c_number.parse::<u32>().unwrap();
        assert!(c_number > 0 && c_number <= 256);
        //update card factor of original
        card_factor[(c_number - 1) as usize] += 1;

        let (w_numbers, numbers) = card.split_once("|").unwrap();
        println!("{} {} > {} ",c_number, w_numbers,numbers);

        for w_number in w_numbers.split_whitespace() {
            winning_set.insert(w_number.parse::<u32>().unwrap());
        }
        println!("{:?}",winning_set);

        let mut matches = 0;
        for number in numbers.split_whitespace() {
            //increase or double card points
            if winning_set.contains(&number.parse::<u32>().unwrap()){
                matches += 1;
                card_factor[(c_number - 1 + matches) as usize]+=1*card_factor[(c_number -1 )as usize];
                println!("\tcard contains winning number: {} ({} points)", number, matches);
            }
        }
        println!("Card {} got factor of {} and {} new cards", c_number, card_factor[(c_number-1) as usize], matches);
        result += card_factor[(c_number-1) as usize];
    }
    println!("Result: {}", result);
}
