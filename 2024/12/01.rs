use std::sync::Arc;
use std::collections::HashSet;

type Point = (usize,usize);

fn explore_region(p: Point, m: & Arc<[Arc<[char]>]>) -> (usize,HashSet::<Point>){
    let mut ret = 0;
    let mut q = Vec::<Point>::from([p]);
    let (nr,nc) = (m.len(), m[0].len());
    let mut visited = HashSet::<Point>::new();
    while let Some((x,y)) = q.pop() {
        if !visited.insert((x,y)) {
            continue
        }
        
        //Up
        if x.checked_sub(1).is_some_and(|e|e < nc) && m[x][y] == m[x-1][y] {
            if !visited.contains(&(x-1,y)){
                q.push((x-1,y));
            }
        }else {
            ret+=1;
        }

        //Left
        if y.checked_sub(1).is_some_and(|e| e < nr) && m[x][y] == m[x][y-1] {
            if !visited.contains(&(x,y-1)) {
                q.push((x,y-1));
            }
        }else {
            ret+=1;
        }
        //Down
        if x.checked_add(1).is_some_and(|e|e<nc) && m[x][y] == m[x+1][y] {
            if !visited.contains(&(x+1,y)) {
                q.push((x+1,y));
            }
        }else {
            ret+=1;
        }
        //Right
        if y.checked_add(1).is_some_and(|e|e < nr) && m[x][y] == m[x][y+1] {
            if !visited.contains(&(x,y+1)) {
                q.push((x,y+1));
            }
        }else {
            ret+=1;
        }
    } 
    (ret * visited.len(),visited)
}


fn main() {
    let m = std::io::stdin().lines().map(|l|{
        l.expect("IO err").chars().collect::<Arc<[_]>>()
    }).collect::<Arc<[Arc<[char]>]>>();
    let mut res = 0;
    let mut explored = HashSet::<Point>::new();
    for (i,r) in m.iter().enumerate() {
        for (j,_) in r.iter().enumerate(){
            if !explored.contains(&(i,j)) {
                let (_r,set) = explore_region((i,j),&m);
                explored.extend(set);
                res+=_r;
            }
        }
    }
    println!("Result: {:?}",res);
}
