use std::io;

fn get_next_from_sequence(seq: Vec<i32>) -> i32 {
    let mut stop = true;

    let mut series : Vec<Vec<i32>> = Vec::<Vec<i32>>::new();
    
    series.push(seq);
    
    loop {
        stop = true;
        let vals = series.last().unwrap().iter();
        let next_vals = vals.clone().skip(1);
        series.push(vals.zip(next_vals).map(|(cur, next)| {stop = stop && (next - cur == 0); next - cur}).collect());
        println!("{:?}",series.last().unwrap());

        if stop {break;}
    }
    
    let mut result = 0;
    print!("Next in sequence: {}",result);
    for v in series.iter().rev() {
        result = v.last().unwrap() + result;
        print!("+{}",result);
    }
    println!();
    result
}

fn main(){
    let lines = io::stdin().lines();
    let mut result = 0;
    for l in lines {
        let l = l.unwrap();
        let seq = l.split(" ").map(|n| n.parse::<i32>().unwrap()).collect::<Vec<i32>>();
        println!("{:?}",seq);
        result += get_next_from_sequence(seq);
    }
    println!("Result: {}",result);
}
