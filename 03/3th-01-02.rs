use std::io;
use std::cmp;

fn get_line_numbers(line: &String) -> Vec<(usize,usize,u32)> {
    let mut numbers : Vec<(usize,usize,u32)>= Vec::<(usize,usize,u32)>::new();
    let mut b = line.len();
    let mut e = 0;
    let mut number = 0;
    for (i,c) in line.chars().into_iter().enumerate(){
        if c.is_digit(10) {
            if b == line.len() {b = i;}
            number = number*10 + c.to_digit(10).unwrap();
        }else if number != 0 {
            e = i-1;
            numbers.push((b,e,number));
            number = 0;
            b = line.len(); //reset begining
        }
    }

    //in case last characer was a digit, we never get to return it's number
    if number != 0 {
            e = line.len();
            numbers.push((b,e,number));
            number = 0;
    }
    numbers
}

fn contained(bn:usize,en:usize,bs:usize,es:usize) -> bool {
    let mut result = (bn >= bs && bn <= es);
    result |= (en <= es && en >= bs);
    result |= (bn <= bs && en >=es);
    result
}

fn get_gear_ratio(lines: &Vec<String>, l:usize,i:usize) -> u32 {
    let mut result = 1;
    let mut gear_count=0;
    print!("Computing gear ratio of: ");
    for l in (cmp::max(l-1,0)..cmp::min(l+2,lines.len())){
        print!("Line {} ",l);
        for (b,e,number) in get_line_numbers(&lines[l]){
            if contained(b,e,cmp::max(i-1,0),cmp::min(i+1,lines[l].len()-1)){
                gear_count +=1;
                result = result * number;
                print!(" ({},{},{}) ", b,e,number);
            }
        }
    }
    println!(" = {}", result);
    result * (gear_count >= 2) as u32
} 
fn main(){
    let lines = io::stdin() 
        .lines() 
        .collect::<Result<Vec<String>, io::Error>>() 
        .unwrap(); 

    //Change approach:
    //  We'll look for symbols, and decide
    //  which operation do to on their neighboring numbers
    let mut result = 0; 
    for (l, line) in lines.iter().enumerate(){
        println!("{}",line);
        for (i, s) in line.match_indices(|c:char| c=='*'){
            println!("{} at line {}, index {}",s,l,i);
            result += get_gear_ratio(&lines,l,i);
        }
    }
    println!("Result: {}", result);
}
