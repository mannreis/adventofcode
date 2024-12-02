use std::io;

fn main() {
	let res = io::stdin().lines().map(|line|{
        let l = line.unwrap();
        l.split_whitespace().map(|n:&str|{str::parse::<i32>(n).unwrap()}).collect::<Vec<i32>>().windows(3).all(|w|{
			let a = w[0];
			let b = w[1];
			let c = w[2];
            a < b && b < c && c - b <= 3 && c - b >= 1 && b - a <= 3 && b - a >=1 ||
			a > b && b > c && a - b <= 3 && a - b >= 1 && b - c <= 3 && b - c >= 1
        })
    }).filter(|&v|v == true).count();

    println!("Result: {:?}", res);
} 
