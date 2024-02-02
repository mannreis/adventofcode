use std::io;
use std::collections::HashMap;

#[derive(Debug)]
struct Workflows {
	flows: HashMap<String,Vec<(String,String)>>,
}
impl Workflows {
	fn new() -> Self{
	Workflows{ flows: HashMap::<String,Vec<_>>::new()}
	}
	
	fn parse_rule(st:&str,value:&HashMap<&str,isize>) -> Option<bool>{
        let op = st.find(|c| r#"<>="#.contains(c)).unwrap();
	    let (k,v) = st.split_at(op);
		let value = value[k];
	 	let (op,v) = v.split_at(1);
	    let v = v.parse::<isize>().unwrap();
	    return match op{
	        "<" => Some(value < v),
	        ">" => Some(value > v),
	    	"=" => Some(value == v),
	    	_   => None,
        }
	}
	fn add(self:&mut Self, name:String, operation:String, result:String) {
		//print!("\t\tAdding ({},{}) to workflow: {}", operation,result, name);
		self.flows.entry(name.clone()).and_modify(|e|e.push((operation.clone(),result.clone()))).or_insert(vec![(operation,result)]);
		//println!("\t->\t{:?}",self.flows.entry(name));
	}
	fn sort_part(self : &Self, part: &HashMap<&str,isize>) -> bool{
		let mut current = "in".to_string();
		print!(": in ");
		while !current.is_empty(){
			if let Some(rules) = self.flows.get(&current) {
				for (condition,action) in rules {
	//				println!("\t{:?},{:?}",condition,action);
					if (condition == "true" || Some(true) == Workflows::parse_rule(condition,part) ) && (action == "A" || action == "R" ){
						println!("-> {} ",action);
						return if action == "A" {true} else {false};
					}else if condition == "true" {
						print!("-> {} ",action);
						current = action.to_string();
						break;
					}else {
						if Some(true) == Workflows::parse_rule(condition,part) {
							print!("-> {} ",action);
							current = action.to_string();
							break;
						}
					}
				}
			}else {
				println!("No rules for {}",current);
				break;
			}
		}
		false
	}
}


fn main() {
	let mut process = false;
	let mut workflows = Workflows::new();
	let mut result = 0;	
	for line in io::stdin().lines() {
		let mut line = line.unwrap();
		if line.is_empty() {
			process = true; 
			println!("Result: {:?}", workflows);
			continue;
		}
		if process {
			line.retain(|c|!r#"{}"#.contains(c));
			let part = line.split(",").map(|k|{ let (k, v) = k.split_once('=').unwrap(); let v=v.parse::<isize>().unwrap(); (k,v)}).collect::<HashMap<_,isize>>();
			print!("{:?}",part.clone());
			if workflows.sort_part(&part) {
				for v in  part.values() {
					result+=v;
				}
			}
			continue;
		}
		let mut rules = line.split_off(line.chars().position(|c|c=='{').unwrap());
		let wf = line;
		rules.retain(|c|c!='{'&& c != '}');
		//println!("workflow {} has rules: {}", wf ,rules);
		for rule in rules.split(','){
			if let Some(j) = rule.find(':') {
				let (rule,action) = rule.split_at(j);
				//println!("\t{} -> {}",rule,action[1..].to_string());
				workflows.add(wf.clone(),rule.to_string(),action[1..].to_string());
			}else{ // final
				//println!("\ttrue -> {}",rule);
				workflows.add(wf.clone(),"true".to_string(),rule.to_string());
			}
		}
	}
	println!("Result: {}",result);
}
