use std::io;
use std::collections::HashMap;

fn main(){

    let mut instructions = String::new();
    let _ = io::stdin().read_line(&mut instructions);
    let mut map : HashMap<String,(String,String)> = HashMap::new();
    let mut current :Vec<String> = Vec::<String>::new();
    let mut lcm : Vec<i64> = Vec::<i64>::new();
    for l in io::stdin().lines(){
        let l = l.unwrap();
        if l.is_empty() { continue };
        let (key,value) = l.split_once(" = ").unwrap();
        let mut value = value.to_string();
        value.retain(|c| !r#"( )"#.contains(c));
        let value = value.split_once(",").unwrap();
        //println!("{} {},{}",key,value.0,value.1);
        map.insert(key.to_string(),(value.0.to_string(),value.1.to_string()));
        if key.ends_with("A"){
            current.push(key.to_string());
            lcm.push(0);
        }
    }
    let mut counter : i64 = 0;
    let mut brk = false;
    while !lcm.iter().all(|c| *c != 0_i64) {
        brk = true;
        for i in instructions.trim().chars() {
            //println!("{:?}",current);
            brk = true;
            counter += 1;
            for (c,l) in current.iter_mut().zip(lcm.iter_mut()) {
                let options = &map[c];
                *c = if i == 'R' {options.1.clone()}else{options.0.clone()};
                if c.ends_with("Z") {
                    *l = counter;
                }
                brk = brk && c.ends_with("Z");
                //print!(" {} ",c);
            }

            //println!();
            if brk {
                println!("Done");
                break
            };
        }
        //println!("Going for another round: {}",counter);
    }
    fn gcd(n: i64, d: i64) -> i64{
        if n%d != 0 {
            gcd(d,n%d)
        }else{
            d
        }
    }
    let result  = lcm.iter().copied().reduce(|a,b| {
        a*b/gcd(a,b)
    }).unwrap();
    println!("It took {:?} iterations, respectively, for each starting point to reach it's end.\nSo the least common multiple of these numbers is: {}", lcm, result);
    for n in lcm {
        println!("\t{}/{}={}",result, n,result/n)
    }    
}
