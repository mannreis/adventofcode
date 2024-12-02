use std::io;

fn main() {
    let mut lists : Vec<Vec<_>> = vec![vec![];2];
    io::stdin().lines().for_each(|line|{
        let l = line.unwrap();
        l.split_whitespace().map(|n:&str|{str::parse::<i32>(n).unwrap()}).enumerate().for_each(|(i,n)|{
            lists[i].push(n);
        })
    });

    for list in lists.iter_mut(){
        list.sort();
    }

    let mut res = 0;
    for j in 0..lists[0].len(){
        let mut distance = lists[0][j];
        for i in 1..lists.len(){
            distance = (distance - lists[i][j]).abs();
        }
        res += distance;
    }
    println!("Result: {:?}", res);
} 