use std::io;

fn main() {
    let lines = io::stdin().lines();
    let mut sum_calibration = 0;
    for line in lines {
//        print!("{} -> ",line.as_ref().unwrap());
        let _numbers : Vec<_> = line
            .unwrap()
            .chars()
            .filter(|&e| e.is_digit(10))
            .map(|c|c.to_digit(10).unwrap()).collect();
        if _numbers.len() > 0 {
            let calibration_value = _numbers[0]*10 + _numbers[_numbers.len()-1];
            sum_calibration += calibration_value;
            //println!("{:?} -> {}",_numbers, _numbers[0]*10 + _numbers[_numbers.len()-1]);
        }
    }
    println!("Sum of calibration values: {}", sum_calibration)
}
