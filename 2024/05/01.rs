use std:: collections::HashMap;
use std::sync::Arc;
use std::io;

fn main() {
    let mut rules: HashMap<i32,Vec<i32>>=HashMap::new();
    let mut ilines = io::stdin().lines();
    while let Some(Ok(l)) = ilines.next() {   
        if l.is_empty(){
            break;
        }
        let numbers = l.split("|").map(|n|n.parse::<i32>().unwrap()).collect::<Arc<[_]>>();
        rules.entry(numbers[0]).and_modify(|r| r.push(numbers[1])).or_insert(vec![numbers[1]]);
        //println!("{:?}->{:?}",numbers,rules);
    }
    
    let mut counter = 0;
    while let Some(Ok(l)) = ilines.next() {   
        let updates = l.split(",").map(|n|n.parse::<i32>().unwrap()).collect::<Arc<[_]>>();
        //println!("{:?}",updates);
        let valid = updates.iter().enumerate().all(|(i,n)| {
            updates.iter().skip(i+1).all(|m|{
                match rules.get(n) {
                    Some(v) => v.contains(m),
                    _ => false
                }
            })
        });
        if valid {
            counter += updates[updates.len()/2 as usize];
        }
    }
    println!("Result: {:?}",counter);
}
