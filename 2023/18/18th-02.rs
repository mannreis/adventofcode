use std::io;

#[derive(Clone,Debug,PartialEq,PartialOrd,Eq,Ord)]
struct Point {
	x:isize,
	y:isize,
}

struct Lagoon {
	layout : Vec<Point>,
}

impl Lagoon {
	fn new(layout : Vec<Point>)->Self{
		Lagoon { layout:layout }
	}
}

fn main () {
	let instructions : Vec<(char,isize)> = io::stdin().lines().map(|l| l.unwrap()).map(|mut line|{
		let mut color = line.split_off(line.len()-9);
		color.retain(|c| !r#"#()"#.contains(c));
		println!("{}",color);
		let dir = match color.split_off(5).chars().nth(0).unwrap() {
			'0'=>'R',
			'1'=>'D',
			'2'=>'L',
			'3'=>'U',
			_=>panic!(),
		};
		let amount = isize::from_str_radix(color.trim(),16).unwrap();
		(dir,amount)
	}).collect();
	
	let mut start = Point{x:1,y:1};
	let mut points: Vec<Point> = Vec::<Point>::new();
	points.push(start.clone());
	let mut b = 0;
	for (d,a) in instructions.iter() {
		 b+=a;
		 match d {
			'R' => start.x+=a,
			'D' => start.y+=a,
			'L' => start.x-=a,
			'U' => start.y-=a,
			_	=> panic!(),
		}
		points.push(start.clone());	
	}	
	println!("{:?}",instructions);
	//assert!(points[0] == points[points.len()-1]);
	//points.sort();
	//points.push(points[0].clone());
	println!("{:?}",points);
	let mut area = 0_isize;
	for (a,b) in points.iter().zip(points[1..].iter()) {
		println!("{:?} and {:?}",a,b);
		//area += (a.x*b.y) as isize - (a.y*b.x) as isize;
		area += (a.y+b.y) as isize *(a.x-b.x)as isize;
	}
	area /= 2;
	let i = area + 1 - b/2; 
	println!("Result {}, interior points {}, boundary points: {}",i+b, i, b);
}
