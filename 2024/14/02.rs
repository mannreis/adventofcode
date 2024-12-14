use std::collections::HashMap;
use std::collections::hash_map::Entry;


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

    fn move_robot(&mut self, cache: &mut HashMap<Pos,usize>){
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

        // Decrement cache
        if let Entry::Occupied(mut o) = cache.entry(self.pos) {
            *o.get_mut()-=1;
            if *o.get() == 0 {
                o.remove();
            }
        }
        
        self.pos = ((x as usize),(y as usize));
        // Decrement cache
        cache.entry(self.pos).and_modify(|e| *e+=1).or_insert(1);
    }
    #[inline]
    pub fn boundaries() -> (usize,usize) {
        (101,103)
    }
}

fn print_map(cache: &HashMap<Pos,usize> ) -> bool{
    let (c,r) = Robot::boundaries();
    let mut s = String::new();
    for i in 0..r{
        for j in 0..c {
            //let count = robots.iter().filter(|r|r.pos==(j,i)).count();
            let count = cache.get(&(j,i)).unwrap_or(&0);
            s.push(if *count == 0usize {'.'}else{char::from_digit((count%10) as u32, 10).unwrap()});
        }
        s+= "\n";
    }
    //println!("{}",s);
    s.contains("1111111111111111111111111111111")
}
fn run(robots: &mut Vec<Robot>) -> usize {
    let mut cache = HashMap::<Pos,usize>::new();

    print_map(&cache);
    let mut i = 1;
    loop {
        robots.iter_mut().for_each(|r| r.move_robot(&mut cache));
        //println!("After {} seconds:",i);
        if print_map(&cache) {
            return i;
        }
        i+=1
    }
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
    let res = run(&mut robots);
    println!("Res: {:?}",res);
}
