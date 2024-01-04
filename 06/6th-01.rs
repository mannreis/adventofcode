use std::io;

fn main(){
    let mut times = String::new();
    io::stdin().read_line(&mut times);
    
    let mut records = String::new();
    io::stdin().read_line(&mut records);
    
    let (_,times) = times.split_once(":").unwrap();
    let times : Vec<_> = times.split_whitespace().map(|e| str::parse::<u32>(e).unwrap()).collect();
    println!("{:?}",times);

    let (_,records) = records.split_once(":").unwrap();
    let records : Vec<_> = records.split_whitespace().map(|e| str::parse::<u32>(e).unwrap()).collect();
    println!("{:?}",records);

    let mut result = 1;
    for (t, record_d) in times.iter().zip(records.iter()){
        print!("({},{}[{}])",t,record_d,record_d/t);

        //the search should start with the same holding time
        //that provides the record distance but for now let's "brute force"
        let mut speed = 0;
        let mut possibilities = 0;
        while speed <= *t {
            let d = (t - speed) * speed;
            //if possibilities > 0 && d < *record_d {
            //    println!("\tCovered distance decreased ({}) for race ({},{}) with initial speed of {}",d, t,record_d,speed);
            //    break;
            if d > *record_d {
                possibilities += 1;
            }
            //println!("\t [{}]->{}", speed,d);
            speed += 1;
        }
        println!("Number of winning possibilities: {}",possibilities);
        result *= possibilities;
    }
    println!("Result: {}",result);
}
