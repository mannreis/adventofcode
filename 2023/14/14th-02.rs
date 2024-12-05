use std::io;
use std::fmt;
use std::io::Write;
use std::collections::HashMap;
use std::collections::hash_map::Entry;
use std::{thread, time};

#[derive(PartialEq)]
enum Direction {
	North,
	South,
	East,
	West,
}

#[derive(Hash,Eq, PartialEq,Clone)]
struct Platform {
	rep : Vec<Vec<char>>,
}

impl fmt::Debug for Platform {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f,"{}", self.rep.iter().map(|l|l.iter().collect::<String>()+"\n").collect::<String>())
    }
}
impl fmt::Display for Platform {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {	
		write!(f,"{}", self.rep.iter().map(|l|l.iter().collect::<String>()+"\n").collect::<String>())
	}
}
impl Platform {
	fn new(platform: Vec<Vec<char>>) -> Platform {
		Platform {rep : platform}
	}
	
	fn cycle(&mut self) {
		for d in [Direction::North,Direction::West,Direction::South,Direction::East] {
			self.tilt(d);
		}
	}	
	fn tilt(&mut self, dir: Direction) {
		for r in 0..self.rep.len()  {
			for c in 0..self.rep[0].len() {
				let (r,c,b,e) = match dir {
					Direction::North=>(r,c,0,r),
					Direction::South=>(self.rep.len()-1-r,c,self.rep.len()-r,self.rep.len()),
					Direction::West	=>(r,c,0,c),
					Direction::East =>(r,self.rep[r].len()-1-c,self.rep[r].len()-c,self.rep[r].len()),
				};
				let mut prev = (r,c);
				if self.rep[r][c] == 'O' {
					//println!("\ttilting ({},{}) {}..{}",r,c, b,e);
					// find where to place rock
					for j in b..e {
						let new = match dir { 
							Direction::North => (e - j - 1,c),
							Direction::West => (r,e-j-1),
							Direction::South => (j,c),
							Direction::East => (r,j),
						};
						//println!("\t\t {:?} -> {:?} ? {}",prev,new, self.rep[new.0][new.1] == '.' );
						if self.rep[new.0][new.1] == '.' {
							self.rep[prev.0][prev.1] = '.';
							self.rep[new.0][new.1]= 'O';
							prev = new;
						}else {
							break;
						}
					}
				}
			}
		}	
		//println!("{}",self);
	}
	
	fn load(self) -> usize{
		let mut res = 0;
		for (i,l) in self.rep.iter().enumerate() {
			res += (l.len() - i) * l.iter().filter(|&c| *c=='O').count();
		}
		res
	}
	
}



fn main(){
	let mut platform : Platform = Platform::new(io::stdin().lines().map(|l|l.unwrap().chars().collect()).collect());
	let mut cache : Vec<Platform> = Vec::<Platform>::new();
	println!("{}",platform);	
	let mut i = 1;
	let iters=1000000000;
	while i < iters {
		println!("Cycle {}, platform:\n{}",i,platform);
		if let Some(next) = cache.iter().position(|j| *j == platform) {
			let loop_size = cache.len() - next;
			println!("Found loop! Cycle {} goes back to {} (size= {}). We can fit {} cycles in this loop, and there's only {} left",i,next,loop_size, (iters - i)/loop_size, (iters - i)%loop_size);
			platform = cache[next+(iters - i + 1)%loop_size].clone();
			break;
		}else{
			cache.push(platform.clone());
		}
		platform.cycle();
		i+=1;
	}
	println!("\nResult: {}",platform.load());
}
