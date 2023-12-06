use std::io;

fn main(){
    let mut times = String::new();
    io::stdin().read_line(&mut times);
    
    let mut records = String::new();
    io::stdin().read_line(&mut records);
    
    let (_,times) = times.split_once(":").unwrap();
    let mut time : u64 = 0_u64;
    for t in times.chars().into_iter().filter(|e|e.is_digit(10)) {
        time = time*10+t.to_digit(10).unwrap() as u64;
    }
    println!("{}",time);
    
    let (_,records) = records.split_once(":").unwrap();
    let mut record : u64 = 0_u64;
    for t in records.chars().into_iter().filter(|e|e.is_digit(10)) {
        record = record*10+t.to_digit(10).unwrap() as u64;
    }
    println!("{}",record);

    print!("({},{}[{}])",time,record,record/time);

    //the search should start with the same holding time
    //that provides the record distance but for now let's "brute force"
    let mut speed : u64 = 0_u64;
    let mut possibilities : u64 = 0_u64;
    while speed <= time {
        let d:u64 = (time - speed) * speed;
        if d > record {
            possibilities += 1;
        }
        //println!("\t [{}]->{}", speed,d);
        speed += 1;
    }
    println!("Number of winning possibilities: {}",possibilities);
    println!("Result: {}",possibilities);
}
