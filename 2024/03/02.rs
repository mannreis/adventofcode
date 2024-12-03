use std::io;
fn evaluate_segment(segment: &str)->i32{
	segment.match_indices("mul(").fold(0,|acc,(i,op)|{
		let s = i+op.len();
		let ll = if s + 12 > segment.len() {segment.len()}else{s+12};
		if let Some(m) = &segment[s..s+4].find(","){
			let _a = &segment[s..s+m].parse::<i32>();
			if !_a.is_ok() || s+m >= ll{
				return acc;
			}
			if let Some(e) = &segment[s+m+1..ll].find(")"){
				let _b = &segment[s+m+1..s+m+1+e].parse::<i32>();
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
}

fn main() {
    let line = io::read_to_string(io::stdin()).unwrap();
	let mut s = 0;
	let mut res = 0;

	loop {
		let e = line[s..].find("don't()").unwrap_or(line.len()-s) + s;
		res += evaluate_segment(&line[s..e]);
		
		let _s = line[e..].find("do()");
		if _s == None{
			break;
		}
		s = e + _s.unwrap();

	}
    println!("Result: {:?}", res);
} 
