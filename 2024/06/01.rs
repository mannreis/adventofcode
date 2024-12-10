//use std:: collections::HashMap;
use std::sync::Arc;
use std::io;
use std::fmt;

struct Guard {
    pos: (usize, usize),
    map: Vec<Vec<char>>,
    counter: u32,
}

impl fmt::Display for Guard {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f,"{}", self.map.iter().map(|l|l.iter().collect::<String>()+"\n").collect::<String>())
    }
}


impl Guard {
    fn new(map: Arc<[Arc<[char]>]>) -> Self {
        
        let mut pos = (0usize,0usize);
        map.iter().enumerate().find(|(i,r)|{
            r.iter().enumerate().find(|(j,c)| {
                match c {
                    '^'|'v'|'>'|'<' => { pos = (*i,*j); true},
                    _ => false,
                }
            }).is_some()
        });

        Guard{pos: pos, map:map.iter().map(|r| r.to_vec()).collect::<Vec<Vec<char>>>(), counter:1}
    }

    fn move_guard(&mut self) -> bool{
        let mut dir = self.map[self.pos.0][self.pos.1];
        match dir {
            'X'|'^'|'>'|'v'|'<' => {
                let (mut x,mut y) = (self.pos.0,self.pos.1);
                let mut counter = 0;
                let mut ndir;
                while counter < 4{
                    let npos = match dir {
                        '^' => {ndir='>';(x.checked_sub(1),Some(y))},
                        'v' => {ndir='<';(x.checked_add(1),Some(y))},
                        '<' => {ndir='^';(Some(x),y.checked_sub(1))},
                        '>' => {ndir='v';(Some(x),y.checked_add(1))},
                        _ =>  {return false;}
                    };
                    if npos.0.is_some() && npos.1.is_some() && npos.0.unwrap() < self.map.len() && npos.1.unwrap() < self.map[npos.0.unwrap()].len() {
                        let (r,c) = (npos.0.unwrap(),npos.1.unwrap());
                        if self.map[r][c] != '#' {
                            (x,y) = (r,c);
                            break;
                        }
                    }else{
                        return false;
                    }
                    dir = ndir;
                    counter+=1;
                }
                if self.map[x][y] != 'X' {
                    self.counter+=1;
                }
                self.map[x][y] = dir;
                self.map[self.pos.0][self.pos.1] = 'X';
                self.pos = (x,y);
                return true;
                
            },
            _ => false
        }
    }
}



fn main() {
    let map = io::stdin().lines().map(|l| {
        l.unwrap().chars().collect::<Arc<[char]>>()
    }).collect::<Arc<[Arc<[char]>]>>();
    
    let mut g = Guard::new(map);
    while g.move_guard() {
        //println!("{}counter: {}",g,g.counter);
    }
    println!("Result: {}",g.counter)
    
}
