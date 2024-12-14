use std::sync::Arc;

fn solve(sys: &Vec<Arc<[f64]>>)->usize{
     let a = (sys[2][0]*sys[1][1] - sys[1][0]*sys[2][1]) / (sys[0][0]*sys[1][1] - sys[1][0]*sys[0][1]);
     let b = (sys[2][1] - a*sys[0][1])/sys[1][1];
     if a.fract() == 0.0 && b.fract() == 0.0 {
        return (3.0*a+1.0*b) as usize;
     }
     return 0;
}

fn main() {
    let mut inp = std::io::stdin().lines();
    let mut eqs = Vec::<Arc<[f64]>>::new();
    let mut res = 0;
    while let Some(Ok(line)) = inp.next(){
        if line.is_empty() {
            let _r= solve(&eqs);
            res += _r;
            //println!("Compute solution for: {:?} -> {:?}", eqs, _r);
            eqs.clear();
            continue;
        }
       
        eqs.push(line.split_terminator(&['X','Y','+','=',',',' '][..]).filter(|x| {
            !x.is_empty() && x.chars().all(|d|d.is_digit(10))
        }).map(|n| n.parse::<f64>().unwrap()).collect::<Arc<[f64]>>());
    }
    if eqs.len() > 0 {
        let _r= solve(&eqs);
        res += _r;
        //println!("Compute solution for: {:?} -> {:?}", eqs, _r);
        eqs.clear();
    }
    println!("Result: {:?}",res);
}
