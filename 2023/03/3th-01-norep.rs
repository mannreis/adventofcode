use std::cmp;
use std::io;

fn main() {
    let lines = io::stdin()
        .lines()
        .collect::<Result<Vec<String>, io::Error>>()
        .unwrap();
    let mut sum = 0;
    for (l, line) in lines.iter().enumerate() {
        println!("{} [{}]", line,l);
        let mut number = 0;
        for (i, c) in line.chars().into_iter().enumerate() {
            if c.is_digit(10)  {
                number = number * 10 + c.to_digit(10).unwrap();
            } 
            if !c.is_digit(10) || i == line.len()-1_usize{
                if number != 0 {
                    print!("Found number {}", number);
                    let mut ndigits = 0;
                    while number / 10_u32.pow(ndigits) != 0 {
                        ndigits += 1;
                    }

                    println!(
                        ", it has {} digits, ranging from [{} to {}]",
                        ndigits,
                        i - ndigits as usize,
                        i
                    );
                    // check for symbols
                    // current line
                    let dbegin = i as isize - ndigits as isize - if c.is_digit(10) && i == line.len()-1_usize {0}else{1};
                    if i > ndigits as usize
                        && line
                            .chars()
                            .into_iter()
                            .nth(dbegin as usize)
                            .unwrap()
                            != '.'
                    {
                        println!(
                            "\t On the same line there's a symbol {} right before the number ({})",
                            line.chars()
                                .into_iter()
                                .nth(dbegin as usize)
                                .unwrap(),
                            i - ndigits as usize - 1
                        );
                        println!("Updating sum: {}+{}={}",sum,number, sum+number);
                        sum += number;
                        number=0;
                        continue;
                    }else{println!("\tNo symbol on the beggining of the number");}
                    if c != '.' && !c.is_digit(10) {
                        println!(
                            "\t On the same line there's a symbol {} right after the number ({})",
                            c, i
                        );
                        println!("Updating sum: {}+{}={}",sum,number, sum+number);
                        sum += number;
                        number=0;
                        continue;
                    }

                    // check previous and next lines
                    let l: isize = l as isize;
                    let dbegin = i as isize - ndigits as isize - if c.is_digit(10) && i == line.len()-1_usize {0}else{1};
                    let dend = i; // + if i == line.len()-1_usize {1}else{0};
                    println!("digit {} begins at {} and stops at {}",number, dbegin,dend);
                    for lindex in [l - 1, l + 1] {
                        if lindex > 0 && lindex < lines.len() as isize {
                            if lines[lindex as usize].char_indices().any(|(j, c)| {
                                if !c.is_digit(10) && c != '.' {
                                    if dbegin <= j as isize && j <= dend {
                                        println!(
                                            "\t Found symbol {} at line {}, index {}. Number has {} digits, and we're checking for symbols between {} to {}",
                                            c, lindex, j, ndigits, dbegin, dend
                                        );
                                        return true;
                                    }
                                }
                                return false;
                            }){
                                println!("Updating sum: {}+{}={}",sum,number, sum+number);
                                sum+=number;
                                number=0;
                                break;
                            }
                        }
                    }
                    number = 0;
                    // move on to the next number
                }
            }
        }
    }

    println!("Result: {}", sum);
}
