use std::io;
use std::collections::VecDeque;
#[derive(PartialEq,Debug)]
enum Direction {N,S,E,W}

fn get_possibilities(pipes:&Vec<Vec<char>>, x:usize, y:usize, dir:&Direction, possibilities: &mut VecDeque<(usize,usize,Direction)>, any : bool) -> usize{
    let n_poss = possibilities.len();
    // explore North
    if (any||*dir == Direction::N) && y.checked_sub(1) != None && "|7F".contains(pipes[y-1][x]){
		let dir = match pipes[y-1][x]{
			'F' => Direction::E,
			'7'	=> Direction::W,
			 _	=> Direction::N,
		};
        possibilities.push_back((x,y-1,dir));
    }
    // explore South
    if (any||*dir == Direction::S) && y+1 < pipes.len() && "|LJ".contains(pipes[y+1][x]){
		let dir = match pipes[y+1][x]{
			'L' => Direction::E,
			'J'	=> Direction::W,
			 _	=> Direction::S,
		};
        possibilities.push_back((x,y+1,dir));
    }
    //explore East
     if (any||*dir == Direction::E) && x+1 < pipes[y].len() && "-J7".contains(pipes[y][x+1]) {
		let dir = match pipes[y][x+1]{
			'J' => Direction::N,
			'7'	=> Direction::S,
			 _	=> Direction::E,
		};
        possibilities.push_back((x+1,y,dir));
    }
    //explore West
    if (any || *dir == Direction::W) && x.checked_sub(1) != None && "-LF".contains(pipes[y][x-1]) {
		let dir = match pipes[y][x-1]{
			'L' => Direction::N,
			'F'	=> Direction::S,
			 _	=> Direction::W,
		};
        possibilities.push_back((x-1,y,dir));
    }
	possibilities.len() - n_poss
}

fn explore(pipes:&mut Vec<Vec<char>>,possibilities:&mut VecDeque<(usize,usize,Direction)>,counter:usize) -> usize{
	let mut new_possibilities = VecDeque::<(usize,usize,Direction)>::new();
	//println!("[{}]",counter);
	//for i in 0..counter{print!("\t");}
	while !possibilities.is_empty(){
		let (x,y,dir) = possibilities.pop_front().unwrap();
        //print!("({},{} > {})",x,y,match dir{Direction::N=>'N',Direction::S=>'S',Direction::W=>'W',_=>'E',});
        pipes[y][x] = 'x';
		get_possibilities(pipes,x,y,&dir,&mut new_possibilities,false);
    }
	//println!();
    //for v in pipes.iter() {
    //  	println!("{:?}",v);
	//}
	if new_possibilities.len() == 0 {
		return counter;
	}else{
		return explore(pipes, &mut new_possibilities, counter+1);
    }
}

fn main(){
    let mut pipes : Vec<Vec<char>> = Vec::<_>::new();
    let (mut x, mut y) : (usize,usize) = (0,0);
    for l in io::stdin().lines(){
        let l = l.unwrap();
        if let Some(v) = l.find('S') {
            y = pipes.len();
			x = v;
            println!("Start at [{},{}]",x,y);
        }
        pipes.push(l.chars().collect());
    }
    //for v in pipes.iter() {
    //    println!("{:?}",v);
    //}
	let mut initial_possibilities = VecDeque::<(usize,usize,Direction)>::new();
	let dir = Direction::N;
	//print!("{:?} -> ",(x,y));
	get_possibilities(&mut pipes,x,y,&dir, &mut initial_possibilities,true);
    println!("Result: {}",explore(&mut pipes,&mut initial_possibilities,1));

}
