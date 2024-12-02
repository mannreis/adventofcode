use std::io;

fn eval(a: i32,b: i32,c: i32, crescent: Option<bool>) -> bool{
	if let Some(order) = crescent {
		return order && (a < b && b < c && c - b <= 3 && c - b >= 1 && b - a <= 3 && b - a >= 1 ) ||
		!order && (a > b && b > c && a - b <= 3 && a - b >= 1 && b - c <= 3 && b - c >= 1);
	}
	a < b && b < c && c - b <= 3 && c - b >= 1 && b - a <= 3 && b - a >= 1 ||
	a > b && b > c && a - b <= 3 && a - b >= 1 && b - c <= 3 && b - c >= 1
}

fn visualize(r:bool,t:i32,n:&Vec<i32>){
	print!("{}", format!("{:<3}", if !r {"\x1b[31mX\x1b[0m  "}else{"\x1b[33mV\x1b[0m  "}));
	n.iter().enumerate().for_each(|(i,v)|{
		let sv = format!("{:<3}",v);
		if t as usize == i {
			print!("\x1b[31m{}\x1b[0m",sv);
		}else{
			print!("{}",sv);
		}
	});
	println!();
}

fn main() {
	let res = io::stdin().lines().map(|line|{
        let l = line.unwrap();
		let mut tolerance :i32= -1;
		let n = l.split_whitespace().map(|n:&str|{str::parse::<i32>(n).unwrap()}).collect::<Vec<_>>();
		let mut a = 0;
		let mut b = 1;
		let mut c = 2;
		let mut order = None;
		while c < n.len(){
			if !eval(n[a],n[b],n[c], None){
				if tolerance >= 0 {
					visualize(false,tolerance,&n);
					return false;
				}
				if c + 1 >= n.len(){
					tolerance = c as i32;
					break;
				}
				if eval(n[a+1],n[b+1],n[c+1],order){
					tolerance = a as i32;
					a+=1;
					b+=1;
					c+=1;
					continue;
				}
				else if eval(n[a],n[b+1],n[c+1],order){
					tolerance = b as i32;
					b+=1;
					c+=1;
					continue;
				}
				else if eval(n[a],n[b],n[c+1],order){
					tolerance = c as i32;
					c+=1;
					continue;
				}
				visualize(false,tolerance,&n);
				return false;
			}
			if order == None {
				order = Some(n[c]-n[a]>0);
			}
			a = b;
			b = c;
			c+=1;
		}
		visualize(true, tolerance, &n);
		return true;
    }).filter(|v|*v).count();

    println!("Result: {:?}", res);
} 
