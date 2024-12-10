use std::sync::Arc;
use std:: collections::HashSet;

type Point = (isize, isize);
type Node = (char, isize,isize);

fn contains(a:& Arc<[Arc<[char]>]>, r:isize,c:isize) -> bool{
    0 <= r && r < a.len() as isize && 0 <= c && c < (a[r as usize].len() as isize)
}

fn main() {
    let mut a = Vec::<Node>::new();
    let m = std::io::stdin().lines().enumerate().map(|(i,l)|{
        l.expect("Input Error").chars().enumerate().map(|(j,c)|{
            if c != '.' {
                a.push((c,i as isize ,j as isize));
            }
            c
        }).collect::<_>()
    }).collect::<Arc<[Arc<[char]>]>>();
    let mut r = HashSet::<Point>::new();
    a.iter().enumerate().for_each(|(i,f)|{
        let mut _n = a.iter().skip(i+1);
        while let Some(s) = _n.next() {
            if f.0 == s.0 {
                let d = (f.1 as isize - s.1 as isize, f.2 as isize - s.2 as isize);
                let mut hd = d;
                let p = (f.1 as isize, f.2 as isize);
                r.insert(p);
                while contains(&m, p.0 + hd.0, p.1 + hd.1) {
                    r.insert((p.0 + hd.0, p.1+hd.1));
                    hd = (hd.0 + d.0, hd.1 + d.1)
                }
                hd = d;
                while contains(&m, p.0 - hd.0, p.1 - hd.1) {
                    r.insert((p.0 - hd.0, p.1 - hd.1));
                    hd = (hd.0 + d.0, hd.1 + d.1)
                }
            }
        }
    });
    
    println!("Result: {:?}",r.len());
}
