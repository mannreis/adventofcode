use std::io;


struct Seed{
    seed_number: u32,
    updated: bool,
}

fn main(){
    let mut lines = io::stdin().lines();
    
    let mut seeds = lines.next().unwrap().unwrap();
    let (_,seeds) = seeds.split_once(": ").unwrap();
    let mut seeds : Vec<u64> = seeds.split_whitespace().map(|s|s.parse::<u64>().unwrap()).collect();
    println!("{:?}", seeds);
    
    let mut mapped_mask : u64= 0;
    for line in lines {
        let line = line.unwrap();
        if line.len() == 0 {
            mapped_mask = 0;
            continue;
        }
        println!("{}",line);
        if line.ends_with(" map:") {
            let(mapname, _) = line.split_once(" ").unwrap();
        }else{
            let map : Vec<_> = line.split_whitespace().map(|s|s.parse::<u64>().unwrap()).collect();
            assert!(map.len() == 3);

            seeds.iter_mut().enumerate().for_each(|(i,s): (usize,&mut u64)| {
                if (mapped_mask & 1<< i) == 0 && map[1] <= *s && *s <= map[1]+map[2] {
                    println!("\t{} is mapped to {}",*s, map[0]+(*s-map[1]));
                    *s = map[0]+(*s-map[1]);
                    mapped_mask|=1<<i;
                }
            });
        }
        println!("{:?}",seeds);
    }
    let mut min = seeds[0];
    for loc in seeds {
        if min >= loc {
            min = loc;
        }
    }
    println!("Result: {}", min);
}
