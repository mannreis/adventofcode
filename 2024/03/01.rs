use std::io;

fn main() {
	let res : i32 = io::stdin().lines().map(|l|{
		let line = l.unwrap(); //.chars().collect::<Vec<char>>();
		line.match_indices("mul(").fold(0,|acc,(i,op)|{
			let s = i+op.len();
			let ll = if s + 12 > line.len() {line.len()}else{s+12};
			if let Some(m) = &line[s..s+4].find(","){
				let _a = &line[s..s+m].parse::<i32>();
				if !_a.is_ok() || s+m >= ll{
					return acc;
				}
				if let Some(e) = &line[s+m+1..ll].find(")"){
					let _b = &line[s+m+1..s+m+1+e].parse::<i32>();
					if !_b.is_ok(){
						return acc;
					}
					if let (Ok(a),Ok(b))= (_a,_b){
						return  acc + a*b;
					}
					return acc;
				}
			}
			return acc 
		})
	}).sum();
    println!("Result: {:?}", res);
} 
