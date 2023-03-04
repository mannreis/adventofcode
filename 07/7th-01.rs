use std::io;
use std::collections::HashMap;
use std::cmp::Ordering;
use std::fmt;

#[derive(PartialEq, PartialOrd, Eq, Hash, Copy, Clone, Debug)]
enum CardType{A,K,Q,J,T,n9,n8,n7,n6,n5,n4,n3,n2}
const Cards: [(char, u8); 13] = [('A',12),('K',11),('Q',10),('J',9),('T',8),('9',7),('8',6),('7',5),('6',4),('5',3),('4',2),('3',1),('2',0)];
#[derive(PartialEq, PartialOrd, Eq, Hash, Copy, Clone, Debug)]
enum HandType {
    FiveOAK,
    FourOAK,
    FullHouse,
    ThreeOAK,
    TwoPairs,
    OnePair,
    HighCard,
}

trait Type { fn find_type(&self) -> HandType; }
impl Type for Hand {
    fn find_type(&self) -> HandType {
        let mut map : HashMap<char,u8> = HashMap::with_capacity(5);
        for (c,_) in &self.cards {
            if let Some(n) = map.get_mut(&c) {
                *n += 1;
            }else{
                map.insert(*c,1);
            }
        }
        let (mut first, mut second) : (u8,u8) = (0,0);
        for v in map.values() {
            if *v > first {
                second = first;
                first = *v;
            }else if *v > second {
                second = *v;
            }
        }
        match first {
            5 => HandType::FiveOAK,
            4 => HandType::FourOAK,
            3 => {if second == 2 {HandType::FullHouse}else{HandType::ThreeOAK}},
            2 => {if second == 2 {HandType::TwoPairs}else{HandType::OnePair}},
            _ => HandType::HighCard,
        }
    }
}

struct Hand {
    cards: Vec<(char,u8)>,
    bid: u32,
}

impl fmt::Debug for Hand {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[{} {}]", self.cards.iter().map(|(c,_)|format!("{}", c)).collect::<Vec<_>>().join(""), self.bid)
    }
}

impl Hand {
    fn new(cards: &str, bid: u32) -> Self {
        assert!(cards.chars().all(|c|Cards.iter().any(|(C,_)| C == &c)));
        Self { 
            cards: cards.chars().map(|c|Cards.iter().find(|C| C.0==c).unwrap().clone()).collect(),
            bid: bid
        }
    }
}

impl PartialEq for Hand {
    fn eq(&self, other: &Self) -> bool {
        if self.find_type() == other.find_type() {
            for (s,o) in self.cards.iter().zip(other.cards.iter()){ 
                if s.1 < o.1 && s.1 > o.1 { 
                    return false; 
                } 
            } 
            return true;
        }
        return false;
    }
}
impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        let stype = self.find_type(); 
        let otype = other.find_type(); 
 
        if stype > otype { 
            return Some(Ordering::Less);
        }else if stype == otype { 
            for (s,o) in self.cards.iter().zip(other.cards.iter()){ 
                if s.1 < o.1 { 
                    return Some(Ordering::Less);
                }else if s.1 > o.1 { 
                    return Some(Ordering::Greater); 
                } 
            } 
            return Some(Ordering::Equal); 
        }else { 
            return Some(Ordering::Greater);
        }
        None
    }
}

fn main() {

    let mut hands : Vec<_> = io::stdin().lines().map(|line|{
        let line = line.unwrap();
        let (hand,bid) = line.split_once(' ').unwrap();
        let h = Hand::new(hand,bid.parse::<u32>().unwrap());
        //println!("{:?}",h.find_type());
        h

    }).collect();
    //println!("{:?}",hands);
    hands.sort_by(|a,b|a.partial_cmp(b).unwrap());
    //println!("{:?}",hands);

    let mut result = 0;
    for (rank,hand) in hands.iter().enumerate(){
        println!("Hand: {:?} Rank: {} -> {}", hand, (rank+1), (rank as u32 +1 )* hand.bid);
        result += hand.bid * (rank as u32 + 1) ;
    }
    println!("Result: {}", result);
}
