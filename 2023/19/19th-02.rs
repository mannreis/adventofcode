use std::io;
use std::collections::HashMap;
use std::fmt;

#[derive(PartialEq,Clone)]
struct Rule {
	rate: char,
	operation:	char,
	value: isize,
	valid: bool,

}

impl Rule {
	fn invert(self:&mut Self){
		match self.operation {
			'>' => {self.operation = '<';self.value+=1;},
			'<' => {self.operation = '>';self.value-=1;},
			 _	=> {self.operation = '!'},
		}
	}
	fn get_range(self: Self,r : std::ops::RangeInclusive<isize>) -> std::ops::RangeInclusive<isize>{
		return match self.operation {
			'>' => std::ops::RangeInclusive::new(self.value+1,*r.end()),
			'<' => std::ops::RangeInclusive::new(*r.start(),self.value-1),
			 _	=> std::ops::RangeInclusive::new(self.value,self.value),
		} 
	}
}
impl fmt::Display for Rule {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		if self.valid {
			write!(f,"{}",true)
		}else{
			write!(f,"{}{}{}", self.rate,self.operation,self.value)
		}
	}
}

impl fmt::Debug for Rule {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		if self.valid {
			write!(f,"{}",true)
		}else{
			write!(f,"{}{}{}", self.rate,self.operation,self.value)
		}
	}
}

impl Default for Rule {
	fn default() -> Self {
       	Self { rate:'.',operation:'.',value: 0, valid:true }
   	}
}

#[derive(Debug)]
struct Workflows {
	flows: HashMap<String,Vec<(Rule,String)>>,
}
impl Workflows {
	fn new() -> Self{
		Workflows{ flows: HashMap::<String,Vec<_>>::new()}
	}
	fn parse_rule(st:&str) -> Rule{
        let op = st.find(|c| r#"<>="#.contains(c)).unwrap();
	    let (k,v) = st.split_at(op);
	 	let (op,v) = v.split_at(1);
	    let v = v.parse::<isize>().unwrap();
		Rule { rate:k.chars().nth(0).unwrap(), operation:op.chars().nth(0).unwrap(),value:v,valid:false}
	}
	fn add(self:&mut Self, name:String, rule:Rule, result:String) {
		self.flows.entry(name.clone()).and_modify(|e|e.push((rule.clone(),result.clone()))).or_insert(vec![(rule,result)]);
	}
	fn possibilities(self : &mut Self) -> u128{
		let mut conds = Vec::<_>::new();
		let mut result = 0;
		self.explore("in",1,&mut conds,&mut result);
		result
	}
	fn explore(self : &Self, flow:&str,mut stage: isize, conds:&mut Vec<Rule>,result: &mut u128) {
		if flow == "A" || flow == "R" {
			if flow == "A" {
				let ( mut x, mut m, mut a, mut s) = (1..=4000,1..=4000,1..=4000,1..=4000);
				for r in conds.iter(){
					if r.valid {continue};
					match r.rate {
						'x' => x = r.clone().get_range(x),
						'm' => m = r.clone().get_range(m),
						'a' => a = r.clone().get_range(a),
						 _	=> s = r.clone().get_range(s), 
					}
				}
				let mut poss = 1_u128;
				for r in [x.clone(),m.clone(),a.clone(),s.clone()].iter() {
					poss*=(*r.end() as u128)-(*r.start() as u128 - 1_u128);
				}
				*result+=poss;
//				println!(" ====++++====> {:?}:{:?}",conds,(x,m,a,s));
			}else {
//				println!(" xxxxxxxxxxxx> {:?}",conds);
			}
			return;
		}
		let mut i = conds.len();
		for (condition,action) in self.flows.get(flow).unwrap() {
//			print!("{}",format!("{: <20}",format!("{}|{}->{}|",flow,condition,action)));
			if condition.valid {
				conds.push(Rule::default());
			}
			if condition.valid == false {
				conds.push(condition.clone());
			}
			self.explore(&action,stage+1,conds,result);
			if let Some(mut rule) = conds.pop() {
				rule.invert();
				conds.push(rule.clone());
			}
			for _ in 0..stage-1{
//				print!("{}",format!("{: <20}",""));
			}
		}
//		println!();
		for i in 0..(conds.len() - i) {
			conds.pop();
		}
		
	}
}


fn main() {
	let mut workflows = Workflows::new();
	let mut result = 0;	
	for line in io::stdin().lines() {
		let mut line = line.unwrap();
		if line.is_empty() {
			println!("Result: {:?}",workflows.possibilities());
			continue;
		}
		let mut rules = line.split_off(line.chars().position(|c|c=='{').unwrap());
		let wf = line;
		rules.retain(|c|c!='{'&& c != '}');
//		println!("workflow {} has rules: {}", wf ,rules);
		for rule in rules.split(','){
			if let Some(j) = rule.find(':') {
				let (rule,action) = rule.split_at(j);
				let rule=Workflows::parse_rule(rule);
//				println!("\t{} -> {}",rule,action[1..].to_string());
				workflows.add(wf.clone(),rule,action[1..].to_string());
			}else{ // final
//				println!("\ttrue -> {}",rule);
				workflows.add(wf.clone(),Rule::default(),rule.to_string());
			}
		}
	}
}
