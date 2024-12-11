fn blink(s:& Vec<i64>, i:usize) -> usize{
    let mut res = s.clone();
    for _ in 0..i {
        let mut n = Vec::<i64>::new();
        for v in res {
            let d = v.checked_ilog10().unwrap_or(0) + 1; 
            if v == 0{
                //println!("\t{:?}",1);
                n.push(1);
            }else if d%2==0{
                //println!("\t{:?} | {:?}", v/(d as i64), v%(10_i64.pow(d as u32)));
                n.push(v/(10_i64.pow((d/2) as u32))); //left side
                n.push(v%(10_i64.pow((d/2) as u32))); //right side
            }else{
                //println!("\t{:?}*2024", v);
                n.push(v*2024);
            }
        }
        //println!(">{:?}",n);
        res = n.clone();
    }
    res.len()
}

fn main() {
    //let ins = "3279 998884 1832781 517 8 18864 28 0";
    let s = std::io::stdin().lines().map(|l|{
        l.expect("IO err").split(' ').map(|n|
            n.parse::<i64>().unwrap()
        ).collect::<Vec<_>>()
    }).collect::<Vec<Vec<i64>>>();
    let iters = 25; 
    let res = s.iter().fold(0,|acc,l| acc + blink(l,iters));
    println!("Result: {:?} --[x{}]--> {:?}",s , iters, res);
}
