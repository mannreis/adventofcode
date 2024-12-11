use std::sync::Arc;
use std::collections::HashMap;

fn reblink(v:i64, i: usize, l: usize, cache: &mut HashMap<(i64,usize),u64>) -> u64{

    if l <= i {
        return 1;
    }
    if let Some(r) = cache.get(&(v,i)) {
        return *r;
    }
    //println!("{:?}",(i,v));
    let d = v.checked_ilog10().unwrap_or(0) + 1; 
    
    let r = match ((v == 0) as u8) << 1 | ((d%2==0) as u8) {
        0x2|0x3 => reblink(1, i+1,l,cache),
        0x1     => reblink(v/(10_i64.pow((d/2) as u32)),i+1,l,cache) + reblink(v%(10_i64.pow((d/2) as u32)),i+1,l,cache),
        _       => reblink(v*2024,i+1,l,cache),
    };
    cache.insert((v,i),r);
    r
}

fn blink(s:& Arc<[i64]>, i:usize) -> u64 {
    let mut cache = HashMap::<(i64,usize),u64>::new();
    s.iter().fold(0,|acc,x| acc + reblink(*x,0,i, &mut cache))
}

fn main() {
    //let ins = "3279 998884 1832781 517 8 18864 28 0";
    let s = std::io::stdin().lines().map(|l|{
        l.expect("IO err").split(' ').map(|n|
            n.parse::<i64>().unwrap()
        ).collect::<Arc<[_]>>()
    }).collect::<Vec<Arc<[i64]>>>();
    let iters = 75;
    let res = s.iter().fold(0,|acc,l| acc + blink(l,iters));
    println!("Result: {:?} --[x{}]--> {:?}",s , iters, res);
}
