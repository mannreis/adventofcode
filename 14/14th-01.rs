use std::io;
use std::fmt;

struct Platform {
	rep : Vec<Vec<char>>,
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
	
	fn tilt(&mut self) {
		for r in 0..self.rep.len() {
			for c in 0..self.rep[r].len(){
				if self.rep[r][c] == 'O' {
					// find where to place rock
					for new_row in (0..r).rev() {
						if self.rep[new_row][c] == '.' {
							self.rep[new_row+1][c] = '.';
							self.rep[new_row][c]= 'O';
						}else {
							break;
						}
					}
				}
			}
		}	
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
	
	println!("{}",platform);	
	platform.tilt();
	println!("{}",platform);	
	println!("Result: {}",platform.load());
}
