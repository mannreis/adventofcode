use std::io;

#[derive(PartialEq,Debug)]
enum Direction {N,S,E,W}

fn get_possibilities(pipes:&Vec<Vec<char>>, x:usize, y:usize, dir:&Direction, possibilities: &mut Vec<(usize,usize,Direction)>, any : bool) -> usize{
    let n_poss = possibilities.len();
    // explore North
    if (any||*dir == Direction::N) && y.checked_sub(1) != None && "S|7F".contains(pipes[y-1][x]){
		let dir = match pipes[y-1][x]{
			'F' => Direction::E,
			'7'	=> Direction::W,
			 _	=> Direction::N,
		};
        possibilities.push((x,y-1,dir));
    }
    // explore South
    if (any||*dir == Direction::S) && y+1 < pipes.len() && "S|LJ".contains(pipes[y+1][x]){
		let dir = match pipes[y+1][x]{
			'L' => Direction::E,
			'J'	=> Direction::W,
			 _	=> Direction::S,
		};
        possibilities.push((x,y+1,dir));
    }
    //explore East
     if (any||*dir == Direction::E) && x+1 < pipes[y].len() && "S-J7".contains(pipes[y][x+1]) {
		let dir = match pipes[y][x+1]{
			'J' => Direction::N,
			'7'	=> Direction::S,
			 _	=> Direction::E,
		};
        possibilities.push((x+1,y,dir));
    }
    //explore West
    if (any || *dir == Direction::W) && x.checked_sub(1) != None && "S-LF".contains(pipes[y][x-1]) {
		let dir = match pipes[y][x-1]{
			'L' => Direction::N,
			'F'	=> Direction::S,
			 _	=> Direction::W,
		};
        possibilities.push((x-1,y,dir));
    }
	possibilities.len() - n_poss
}

fn explore(pipes:&mut Vec<Vec<char>>,possibilities:&mut Vec<(usize,usize,Direction)>, vertices:&mut Vec<(usize,usize)>) -> usize{
	let mut v = false;
	let mut counter = 1;
	let mut c : char = 'x';
	//println!("[{}]",counter);
	//for i in 0..counter{print!("\t");}
	while !possibilities.is_empty(){
		let (x,y,dir) = possibilities.pop().unwrap();
        //print!("({},{} > {})",x,y,match dir{Direction::N=>'N',Direction::S=>'S',Direction::W=>'W',_=>'E',});
    	//for v in pipes.iter() {
      	//	println!("{:?}",v);
		//}
		if "F7JL".contains(pipes[y][x]) {
			vertices.push((x,y));
			//println!("Adding vertex {},{}",x,y);
			v = true;
		}else if pipes[y][x] == 'S' {
			break;
		}
		//println!();
		c = pipes[y][x]; 
        pipes[y][x] = 'X';
		counter += 1;
		if get_possibilities(pipes,x,y,&dir,possibilities,false) == 0 {
			counter -= 1;
			if v {vertices.pop();v=false;}
			pipes[y][x] = 'x'; //c;	
		}
    }
	return counter;
	//println!();
    //for v in pipes.iter() {
    //  	println!("{:?}",v);
	//}
	//if new_possibilities.len() == 0 {
	//	return counter;
	//}else{
	//	return explore(pipes, &mut new_possibilities, counter+1, vertices);
    //}
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
	let mut initial_possibilities = Vec::<(usize,usize,Direction)>::new();
	let dir = Direction::N;
	//print!("{:?} -> ",(x,y));
	get_possibilities(&mut pipes,x,y,&dir, &mut initial_possibilities,true);
	let mut vertices: Vec<(usize,usize)> = Vec::<(usize,usize)>::new();
	let b = explore(&mut pipes,&mut initial_possibilities,&mut vertices);
    for v in pipes.iter() {
    	println!("{:?}",v);
	}
	
	println!("Vertices: {:?}",vertices);
	//vertices.push((x,y));
	let first = vertices.iter();
	let second = first.clone().skip(1);
	let mut area = 0;
	for (f,s) in first.zip(second){
		area += (f.0 as i64 * s.1 as i64 - f.1 as i64 * s.0 as i64);
	}
	area += vertices[vertices.len()-1].0 as i64 * vertices[0].1 as i64 - vertices[vertices.len()-1].1 as i64 * vertices[0].0 as i64;
	area = area/2;
	println!("Area: {}\tInterior points: {}", area,b);

	let result = area - b as i64 /2 + 1;
	println!("Result:{}", result);
	
}
