//use std::collections::HashMap
use std::sync::Arc;

fn print_map(m: &Vec<i64>) {
    println!("{:?}",m.iter().map(|&id|
        if id == 0 {'.'} 
        else {
            char::from_digit(
                ((id-1)%10) as u32
                ,10
            ).unwrap()
        }).collect::<String>());
}

fn checksum(m: &Vec<i64>) -> i64{
    m.iter().enumerate().fold(0,|xs,(i,id)|{
        xs + i as i64 * (if *id > 0 {*id - 1} else{0})
    })
}

fn realign(m: &mut Vec<i64>){
    let mut e = m.len()-1;
    let mut l;
    while e > 0 {
        //while s < m.len()-1 && m[s]!=0{
        //    s+=1;
        //}
        while 0 < e && m[e]==0{
            e-=1;
        }

        l = 0;
        while 0 < e-l && l <= m.len() && m[e] == m[e-l] {
            l+=1;
        }
        if let Some(f_pos) = m[0..e].windows(l).position(|w| w.iter().all(|&id|id==0)) {
            //println!("Pattern {}*{:?} fits on {}",l,(m[e]-1)%10, s+f_pos);
            //print_map(m);
            //println!("{:?}"," ".repeat(s)+&"s"+&" ".repeat(e-s)+&"e");
            for i in 0..l {
                m.swap(f_pos+i,e);
                e-=1;
            }

        }else{
           // println!("Pattern {}x{:?} don't fit",l,(m[e]-1)%10);
            //print_map(m);
            //println!("{:?}"," ".repeat(s)+&"s"+&" ".repeat(e-s)+&"e");
            // no space...
            e -= l;
        }

    }
}

fn main() {
    let res = std::io::stdin().lines().fold(0,|r,l|{
        //println!("{:?}",l);
        let d = l.expect("Input Error").chars().map(|c|c.to_digit(10).unwrap() as usize).collect::<Arc<[_]>>();
        let mut disk_map = Vec::<i64>::with_capacity(d.iter().sum::<usize>());
        d.iter().step_by(2).enumerate().for_each(|(id,f)| {
            let s = if  id*2+1 < d.len(){ d[id*2+1] } else { 0 };
            //println!("File {:?} blocks: {:?}, free: {:?}", id, f,s);
            //print!("{}",id.to_string().repeat(*f)+&".".repeat(s));
            disk_map.append(&mut vec![1+ id as i64;*f]);
            disk_map.append(&mut vec![0;s]);
        });
        //println!("\n>{:?}",disk_map);
        //print_map(&disk_map);
        realign(&mut disk_map);
        //print_map(&disk_map);
        //println!("\n>>>>>{:?}",disk_map);
        r + checksum(&disk_map)
    });
    println!("Result: {:?}", res);
}
