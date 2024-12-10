use std:: collections::HashSet;
use std::sync::Arc;
use std::io;
use std::fmt;

struct Guard {
    pos: (usize, usize),
    map: Vec<Vec<char>>,
    counter: u32,
    record: HashSet<(usize,usize,char)>
}

impl fmt::Display for Guard {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f,"{}", self.map.iter().map(|l|l.iter().collect::<String>()+"\n").collect::<String>())
    }
}


impl Guard {
    fn new(map: &Arc<[Arc<[char]>]>) -> Self {
        
        let mut pos = (0usize,0usize);
        map.iter().enumerate().find(|(i,r)|{
            r.iter().enumerate().find(|(j,c)| {
                match c {
                    '^'|'v'|'>'|'<' => { pos = (*i,*j); true},
                    _ => false,
                }
            }).is_some()
        });

        Guard{
            pos: pos,
            map:map.iter().map(|r| r.to_vec()).collect::<Vec<Vec<char>>>(),
            counter:1,
            record:HashSet::from([(pos.0,pos.1,map[pos.0][pos.1])])
        }
    }

    fn add_obstacle(&mut self, i:usize,j:usize) -> bool{
        if i < self.map.len() && j < self.map[j].len() {
            if self.map[i][j] == '.' {
                self.map[i][j] = '#';
                return true;
            }
            return false;
        }
        return false;
    }
    fn move_guard(&mut self) -> Option<bool>{
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
                        _ =>  {return Some(false);}
                    };
                    if npos.0.is_some() && npos.1.is_some() && npos.0.unwrap() < self.map.len() && npos.1.unwrap() < self.map[npos.0.unwrap()].len() {
                        let (r,c) = (npos.0.unwrap(),npos.1.unwrap());
                        // add positions that make direction turns
                        self.record.insert((self.pos.0,self.pos.1,dir));
                        if self.map[r][c] != '#' {
                            (x,y) = (r,c);
                            break;
                        }
                    }else{
                        return Some(false);
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

                if self.record.contains(&(x,y,dir)) {
                    //println!("Loop detected: {:?}",(x,y,dir));
                    return None;
                }
                
                return Some(true);
                
            },
            _ => Some(false)
        }
    }
}



fn main() {
    let map = io::stdin().lines().map(|l| {
        l.unwrap().chars().collect::<Arc<[char]>>()
    }).collect::<Arc<[Arc<[char]>]>>();
    
    let mut res=0;
    for i in 0..map.len(){
        for j in 0..map[i].len() {
            if map[i][j] == '.' {
                let mut g = Guard::new(&map);
                g.add_obstacle(i,j);
                //println!("{}\n",g);
                loop {
                    let v = g.move_guard();
                    if Some(false) == v || v.is_none() {
                        res+=v.is_none() as i32;
                        break;
                    }
                    //println!("{}\n",g);
                }
            }
        }
    }
    println!("Result: {}",res);
}
