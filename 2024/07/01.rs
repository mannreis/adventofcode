use std::sync::Arc;

fn check(t:i64, v:&[i64]) -> bool{
    let mut ops = 0;
    let mut attempt;
    //println!("max possibilities: {:?}",2_i64.pow((v.len()-1) as u32));
    while ops < 2_i64.pow((v.len() - 1) as u32){
        //print!("\t({:?}){:?}",ops+1,v[0]);
        attempt = v[1..].iter().enumerate().fold(v[0],|acc,(i,a)|{
            if 1 << i & ops == 0 {
                //print!("*{:?}",a);
                return acc*a
            }
            //print!("+{:?}",a);
            return acc+a
        });
        //println!("->{:?}",attempt);
        if attempt == t  {
            return true;
        }
        ops+=1;
    }
    false
}


fn main() {

    let res = std::io::stdin().lines().fold(0,|res,l|{
        let r = l.unwrap().split([':', ' ']).filter(|n| !n.is_empty()).map(|n|{
            n.parse::<i64>().unwrap()
        }).collect::<Arc<[i64]>>();
        
        //println!("{:?}",r);
        res + if check(r[0],&r[1..]) {
            //println!("true");
            r[0]
        } else {
            //println!("false");
            0
        }
    });
    println!("Result: {:?}", res);
}
