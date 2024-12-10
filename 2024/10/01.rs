use std::sync::Arc;

type Point = (isize, isize);

fn contains(map:&Arc<[Arc<[u32]>]>, p:Point) -> bool{
    0 <= p.0 && (p.0 as usize) < map.len() &&
    0 <= p.1 && (p.1 as usize) < map[p.0 as usize].len()
}

fn explore(map:&Arc<[Arc<[u32]>]>, start:(usize,usize)) -> u64{
    let mut counter = 0;
    let p = start;
    let mut queue = Vec::<(usize,usize)>::new();
    queue.push(p);
    let mut m = map.iter().map(|r| 
        r.iter().map(|&v|v).collect::<Vec<_>>()
    ).collect::<Vec<Vec<_>>>();
    
   
    while let Some(n) = queue.pop() {
        for d in [(0,1),(1,0),(0,-1),(-1,0)]{
            let r = n.0 as isize +d.0;
            let c = n.1 as isize +d.1;
            
            if contains(map, (r,c)){
                let (x,y) = (r as usize, c as usize);
                
                if m[x][y] == m[n.0][n.1] + 1 {
                    if m[x][y] == 9 {
                        counter += 1;
                        m[x][y] += 100;
                    }else{
                        //println!(" on position {:?} adding {:?}",n, (x,y));
                        queue.push((x,y));
                    }
                }
            }
        }
    }
    counter
}

fn main() {
    let mut h = Vec::<Point>::new();
    let map = std::io::stdin().lines().enumerate().map(|(i,l)|{
        l.expect("IO error").chars().enumerate().map(|(j,c)|{
            if c == '0'{
                h.push((i as isize ,j as isize));
            }
            c.to_digit(10).unwrap()
        }).collect::<_>()
    }).collect::<Arc<[Arc<[u32]>]>>();
    
    let mut res = 0;
    h.iter().for_each(|s|{
        
        let r = explore(&map,(s.0 as usize,s.1 as usize));
        //println!("{:?} -> {:?}",s, r);
        res+=r;
        
    });
    
    println!("Result: {:?}",res);
}
