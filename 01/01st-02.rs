use std::io;
use std::iter::Peekable;

fn get_number<I: Iterator<Item = char>>(p_it: Peekable<I>) -> Option<u32>
where
    I: Clone,
{
    let numbers = [
        ("one", 1),
        ("two", 2),
        ("three", 3),
        ("four", 4),
        ("five", 5),
        ("six", 6),
        ("seven", 7),
        ("eight", 8),
        ("nine", 9),
    ];
    for (number, i) in numbers.map(|(s, i)| (s.chars(), i)) {
        let mut p_it_copy = p_it.clone();
        let l = number.size_hint().0 + 1;
        for (pos, n) in number.enumerate() {
            match p_it_copy.peek() {
                Some(&c) => {
                    if c == n {
                        p_it_copy.next();
                        if l == pos {
                            return Some(i);
                        }
                    } else if c.is_digit(10) && pos == 0 {
                        return Some(c.to_digit(10).unwrap());
                    } else {
                        break;
                    }
                }
                None => {
                    break;
                }
            };
        }
    }
    //return
    None
}

fn main() {
    let lines = io::stdin().lines();
    let mut sum_calibration = 0;
    for line in lines {
        let line = line.unwrap();
        print!("{}\t",line);
        let mut line_it = line.chars().into_iter().peekable();

        let mut _numbers: Vec<_> = Vec::<u32>::new();
        while let Some(c) = line_it.peek(){
            print!("{}", c);
            if let Some(d) = get_number(line_it.clone().peekable()) {
                print!(">{}<",d);
                _numbers.push(d);
            }
            line_it.next();
        }

        println!("{:?}", _numbers);
        if _numbers.len() > 0 {
            let calibration_value = _numbers[0] * 10 + _numbers[_numbers.len() - 1];
            sum_calibration += calibration_value;
            println!("{:?} -> {}",_numbers, _numbers[0]*10 + _numbers[_numbers.len()-1]);
        }
    }
    println!("Sum of calibration values: {}", sum_calibration)
}
