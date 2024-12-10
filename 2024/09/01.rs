//use std::collections::HashMap
use std::sync::Arc;

fn checksum(m: &Vec<i64>) -> i64{
    m.iter().enumerate().fold(0,|xs,(i,id)|{
        xs + i as i64 * (if *id > 0 {*id - 1} else{0})
    })
}
fn compress(m: &mut Vec<i64>){
    let mut s = 0;
    let mut e = m.len()-1;
    loop {
        while s < m.len() && m[s] != 0 {
           s+=1;
        }
        while 0 < e && m[e]==0{
            e-=1;
        }
        
        if s > e {
            break
        }
        // Ready to swap
        m.swap(s,e);
    }
}

fn main() {
    let res = std::io::stdin().lines().fold(0,|r,l|{
        let d = l.expect("Input Error").chars().map(|c|c.to_digit(10).unwrap() as usize).collect::<Arc<[_]>>();
        let mut disk_map = Vec::<i64>::with_capacity(d.iter().sum::<usize>());
        d.iter().step_by(2).enumerate().for_each(|(id,f)| {
            let s = if  id*2+1 < d.len(){ d[id*2+1] } else { 0 };
            disk_map.append(&mut vec![1+ id as i64;*f]);
            disk_map.append(&mut vec![0;s]);
        });
        
        compress(&mut disk_map);
        checksum(&disk_map)
    });
    println!("Result: {:?}", res);
}
