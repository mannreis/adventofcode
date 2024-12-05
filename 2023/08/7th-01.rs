use std::io;
use std::collections::HashMap;

fn main(){

    let mut instructions = String::new();
    let _ = io::stdin().read_line(&mut instructions);
    let mut map : HashMap<String,(String,String)> = HashMap::new();
    let mut current :String = "AAA".to_string();
    let goal    :String = "ZZZ".to_string();
    for l in io::stdin().lines(){
        let l = l.unwrap();
        if l.is_empty() { continue };
        let (key,value) = l.split_once(" = ").unwrap();
        let mut value = value.to_string();
        value.retain(|c| !r#"( )"#.contains(c));
        let value = value.split_once(",").unwrap();
        //println!("{} {},{}",key,value.0,value.1);
        map.insert(key.to_string(),(value.0.to_string(),value.1.to_string()));
    }
    let mut counter : u64 = 0;
    while current != goal {
        for i in instructions.trim().chars() {
            let options = &map[&current];
            current = if i == 'R' {options.1.clone()}else{options.0.clone()};
            counter += 1;
            if current == goal {
                break;
            }
        }
        //println!("Going for another round: {}",counter);
    }
    println!("Result: {}", counter);
}
