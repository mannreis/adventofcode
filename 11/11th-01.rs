use std::io;

fn main() {
	let lines = io::stdin().lines();
	let mut universe : Vec<Vec<char>> = Vec::<Vec<char>>::new();
	let mut galaxies : Vec<(usize,usize)> = Vec::<(usize,usize)>::new();
	let mut i = 0;	
	for l in lines {
		i+=1;
		let l = l.unwrap();
		if !l.contains('#') {
			universe.push(l.chars().into_iter().collect());
			println!("Add row: {}", i);
		}			
		universe.push(l.chars().into_iter().collect());
	}
	
	i=0;
	let mut j = 0;
	while i + j < universe[0].len() {
		if universe.iter().all(|v|v[i+j]=='.') {
			println!("Add columun: {}",i);
			j+=1;
			for r in &mut universe { r.insert(i+j,'.');}
		}
		i+=1;
	}
	
	for (y,r) in universe.iter().enumerate(){
		let mut i = r.iter();
		let mut x = 0;
		while let Some(g) = i.position(|&c|c=='#'){
			//println!("Galaxie at ({},{})",y,x+g);
			galaxies.push((x+g,y));
			x += g+1;
		}
		println!("{:?}",r.iter().collect::<String>());
	}
	println!("Universe dimensions: {}x{}",universe.len(),universe[0].len());
	println!("Number of galaxies: {}", galaxies.len());
	let mut result = 0;
	
	for i in 0..galaxies.len() {
		for j in i+1..galaxies.len() {
			let (a,b) = (galaxies[i],galaxies[j]);
			//let d = f64::sqrt(((a.0 as i64 -b.0 as i64).pow(2) + (a.1 as i64 -b.1 as i64).pow(2)) as f64).ceil();
			let d = (a.0 as i64 - b.0 as i64).abs() + (a.1 as i64 -b.1 as i64).abs();
			//println!("{:?}:{:?} => {}",a,b,d);	
			result += d as i64;
		}
	}
	println!("Result: {}",result);
}
