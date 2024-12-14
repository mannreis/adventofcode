
type Vel = (isize,isize);
type Pos = (usize,usize);

#[derive(Debug)]
struct Robot {
    pos: Pos,
    vel: Vel,

}

impl Robot {
    fn new(p: (usize,usize), v:(isize,isize)) -> Self {
        Robot {pos:p,vel:v} 
    }

    fn move_robot(&mut self){
        let (c,r) = Robot::boundaries();
        let C = c as isize;
        let R = r as isize;
        let (mut x,mut y) = ((self.pos.0 as isize), (self.pos.1 as isize));
        
        x+=self.vel.0;
        if x < 0 { 
            x = (C+x)%C;
        } else if x >= C {
            x = x%C;
        }

        y+=self.vel.1;
        if y < 0 { 
            y = (R+y)%R;
        } else if y >= R {
            y = y%R;
        }

        self.pos = ((x as usize),(y as usize));
    }
    #[inline]
    pub fn boundaries() -> (usize,usize) {
        (101,103)
    }
}

fn print_map(robots: &Vec<Robot>){
    let (c,r) = Robot::boundaries();
    for i in 0..r{
        for j in 0..c {
            let count = robots.iter().filter(|r|r.pos==(j,i)).count();
            print!("{}", if count == 0 {'.'}else{char::from_digit((count%10) as u32, 10).unwrap()});
        }
        println!();
    }
    println!()
}
fn run(robots: &mut Vec<Robot>) -> usize {
    println!("Initial state:");
    print_map(robots);
    for i in 1..=100{
        robots.iter_mut().for_each(|r| r.move_robot());
        if i == 100 {
            println!("After {} seconds:",i);
            print_map(robots);
        }
    }
    
    let (C,R) = Robot::boundaries();
    let mut res = 1;
    for i in [(0..(C/2)),((C/2+1)..C)]{
        for j in [(0..(R/2)),((R/2+1)..R)] {
            //add quadrant
            let _r = robots.iter().filter(|r|i.contains(&r.pos.0)&&j.contains(&r.pos.1)).count();
            println!("Quadrant ({:?}) -> {}",(i.clone(),j.clone()), _r);
            res *= _r;
        }
    }
    res
}

fn main () {
    let mut robots = std::io::stdin().lines().map(|_l|{
        let l = _l.expect("IO err");
        let values = l.split_terminator(['p','=',',',' ','v']).filter(|n| {
            !n.is_empty()
        }).collect::<Vec<_>>();
        assert!(values.len() == 4);
        let pos = (values[0].parse::<usize>().unwrap(), values[1].parse::<usize>().unwrap());
        let vel = (values[2].parse::<isize>().unwrap(), values[3].parse::<isize>().unwrap());
        Robot::new(pos, vel)
        
    }).collect::<Vec<Robot>>();
    println!("{:?}",robots);

    let res = run(&mut robots);
    println!("{:?}",res);
}
