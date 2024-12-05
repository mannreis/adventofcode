use std::io;
use std::cmp::{min,max};

fn main() {
	let lines = io::stdin().lines();
	let mut universe : Vec<Vec<char>> = Vec::<Vec<char>>::new();
	let mut galaxies : Vec<(usize,usize)> = Vec::<(usize,usize)>::new();
	let mut x_scale	= Vec::<usize>::new();
	let mut y_scale	= Vec::<usize>::new();
	let factor = 1000000;
	let mut i = 0;	
	for l in lines {
		let l = l.unwrap();
		if !l.contains('#') {
			//	universe.push(l.chars().into_iter().collect());
			println!("Add row: {}", i);
			y_scale.push(i);
		}			
		universe.push(l.chars().into_iter().collect());
		i+=1;
	}
	
	i=0;
	while i < universe[0].len() {
		if universe.iter().all(|v|v[i]=='.') {
			println!("Add columun: {}",i);
			x_scale.push(i);
		}
		i+=1;
	}
	print!(" ");
	for i in 0..universe[0].len() {
		print!("{}",if x_scale.contains(&i) {'v'}else{' '});
	}
	println!();	
	for (y,r) in universe.iter().enumerate(){
		let mut i = r.iter();
		let mut x = 0;
		while let Some(g) = i.position(|&c|c=='#'){
			//println!("Galaxie at ({},{})",y,x+g);
			galaxies.push((x+g,y));
			x += g+1;
		}
		println!("{}{}",if y_scale.contains(&y){'>'}else{' '},r.iter().collect::<String>());
	}
	println!("Universe dimensions: {}x{}",universe.len(),universe[0].len());
	println!("Number of galaxies: {}", galaxies.len());
	let mut result = 0;
	
	for i in 0..galaxies.len() {
		for j in i+1..galaxies.len() {
			let (a,b) = (galaxies[i],galaxies[j]);
			let n_x = x_scale.iter().filter(|&x| *x < max(a.0,b.0) && * x > min (a.0,b.0)).count();
			let n_y = y_scale.iter().filter(|&y| *y < max(a.1,b.1) && * y > min (a.1,b.1)).count();
			
			let d = (a.0 as i64 - b.0 as i64).abs() + n_x as i64*(factor-1) + (a.1 as i64 -b.1 as i64).abs() + n_y  as i64*(factor-1);
			//println!("{:?}:{:?} => {}",a,b,d);	
			result += d as i64;
		}
	}
	println!("Result: {}",result);
}
