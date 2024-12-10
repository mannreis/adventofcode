use std::sync::Arc;

type MyT = u128;

fn check_2(t:MyT, v:&[MyT]) -> bool{
    let mut ops = 0;
    let mut attempt;
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

fn check_3(t:MyT, v:&[MyT]) -> bool{
    let mut ops = 0;
    let mut attempt;
    while ops < 4_i64.pow((v.len() - 1) as u32){
        //print!("\t({:?}){:?}",ops,v[0]);
        attempt = v[1..].iter().enumerate().fold(v[0],|acc:u128,(i,a)|{
            match 0x3 & (ops >> i*2) {
                0x0 => {
                    //print!("*{:?}",a);
                    acc*a
                },
                0x1 => {
                    //print!("+{:?}",a);
                    acc+a
                },
                _ => {
                    //print!("||{:?}",a);
                    format!("{}{}",acc,a).parse::<u128>().unwrap() // * 10_u128.pow((a/10 + 1) as u32) as MyT + a
                },
            }
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
            n.parse::<MyT>().unwrap()
        }).collect::<Arc<[MyT]>>();
        
        //println!("{:?}",r);
        res + if check_2(r[0],&r[1..]) || check_3(r[0],&r[1..]){
            //println!("true");
            r[0]
        } else {
            //println!("false");
            0
        }
    });
    println!("Result: {:?}", res);
}
