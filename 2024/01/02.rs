use std::io;
use std::collections::HashMap;

fn main() {
    let mut hmap : HashMap<i32,i32> =  HashMap::new();
    let mut lists : Vec<Vec<_>> = vec![vec![];2];
    io::stdin().lines().for_each(|line|{
        let l = line.unwrap();
        l.split_whitespace().map(|n:&str|{str::parse::<i32>(n).unwrap()}).enumerate().for_each(|(i,n)|{
            lists[i].push(n);
        });
        hmap.entry(*lists[1].last().unwrap()).and_modify(|counter| *counter += 1).or_insert(1);
    });
    println!("{:?}", hmap);

    let mut res = 0;
    for j in 0..lists[0].len(){
        let lv: i32 = lists[0][j];
        res += lv * hmap.get(&lv).unwrap_or(&0i32);
        //println!("{:?} exists {:?} times => {:?} ", lv, hmap.get(&lv).unwrap_or(&0i32), res);
    }
    println!("Result: {:?}", res);
} 