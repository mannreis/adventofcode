use std::io;
use std::cell::RefCell;
use std::collections::HashMap;
use std::collections::VecDeque;
use std::fmt;


#[derive(Debug,Clone)]
enum ModuleType {
	FlipFlop,
	Conjunction,
	Broadcaster,
}

struct Module {
	state:bool,
	tic: usize,
	name: String,
	modtype: ModuleType,
	inputs: Vec<String>,
	outputs: Vec<String>
}
impl fmt::Debug for Module {
	fn fmt(&self, f: &mut fmt::Formatter<'_>)->fmt::Result{
		write!(f,"{}",self.name)
	}
}
impl Module {
	fn new(name:String, input: Vec<String>, output: Vec<String>) -> Self {
		Module {state:false, tic:0,name:name,modtype:ModuleType::Conjunction, inputs:input, outputs:output}
	}
}

#[derive(Debug)]
struct Configuration {
	modules: HashMap<String,RefCell<Module>>,
}

impl Configuration {
	fn new() -> Self {
		Configuration {modules: HashMap::<String,_>::new()}
	}
	
	fn add(&mut self,name:String,outputs:Vec<String>) {
		let mut modtype = ModuleType::Broadcaster;
		let mut name_it = name.chars();
		if name.starts_with("broadcaster"){
	//		println!("{}>{:?}",name,outputs);
		}else if name.starts_with('%') {
			modtype = ModuleType::FlipFlop;
	//		println!("Flip-flop");
	//		println!("{}>{:?}",name,outputs);
			name_it.next();
		}else if name.starts_with('&') {
			modtype = ModuleType::Conjunction;
	//		println!("Conjuction");
	//		println!("{}>{:?}",name,outputs);
			name_it.next();
		}else {
			panic!("wtf");
		}
		let input_name = name_it.clone().as_str().to_string();
		for out in outputs.iter() {
	//		println!("\tInserting or updating {} to set as input {}",out.clone(),input_name);
			self.modules.entry(out.clone()).and_modify(|m|{m.get_mut().inputs.push(input_name.clone())}).or_insert(RefCell::new(Module::new(out.clone(),vec![input_name.clone()],Vec::<_>::new())));
		}
		
//		self.modules.entry(input_name.clone()).and_modify(|m|{m.get_mut().modtype=modtype; m.get_mut().outputs = outputs.clone()}).or_insert(RefCell::new(Module::new(input_name.clone(),Vec::<_>::new(),outputs.clone())));
		let mut module = Module::new(input_name.clone(),Vec::<_>::new(),outputs.clone());
		module.modtype = modtype.clone();
		self.modules.entry(input_name.clone()).and_modify(|m|{m.get_mut().modtype=modtype; m.get_mut().outputs = outputs.clone()}).or_insert(RefCell::new(module));
	//	println!("\t{:?}",self);
	}
	
	fn press_button(& self) -> Option<Vec<String>>{
		let mut next_stage = VecDeque::<(bool,String)>::from([(false,String::from("broadcaster"))]);
		
		let mut mods = vec!();
		while let Some((signal, name)) = next_stage.pop_front() {
			//if ["tc", "ks","dn","ms"].map(|n|n.to_string()).contains(&name) && !signal {
			if ["gc", "cm","xf","sz"].map(|n|n.to_string()).contains(&name) && !signal {
				mods.push(name.clone());
			}
//			if name == "rx" {
//				println!("RX received: {}",signal);
//				return true
//			}
			let mut module = self.modules.get(&name).unwrap().borrow_mut();
//			println!("Popped: {:?}>{:?} from {:?}\t",module.modtype,(signal,name.clone()),next_stage);
			match module.modtype {
				ModuleType::FlipFlop => {
					if !signal {
							module.state = ! module.state;
							for o in module.outputs.iter() {
								format!("{} -{}-> {}", module.name, if module.state{"high"}else{"low"}, o.clone());	
								next_stage.push_back((module.state, o.clone()));
							}
					}
					module.tic += 1;
				},
				ModuleType::Conjunction=>{
					let mut all = true;
					for _i in module.inputs.iter() {
						let m = self.modules.get(_i).unwrap().borrow();
						all &= m.state;
					}
					module.state = !all;
					for o in module.outputs.iter() {
						//println!("{} -{}-> {}", module.name, if !all{"high"}else{"low"}, o.clone());	
						next_stage.push_back((!all, o.clone()));
					}
				},
				ModuleType::Broadcaster=>{
					//println!("button -low-> broadcaster");
					for o in module.outputs.iter() {
						//println!("{} -{}-> {}", module.name, if module.state{"high"}else{"low"}, o.clone());	
						next_stage.push_back((signal, o.clone()));
					}
				},
			}
		}
		if mods.len() > 0  {
			Some(mods)
		}else{
			None
		}
	}
}

fn main() {
	let mut conf = Configuration::new();
	println!("flowchart TD");
	for line in io::stdin().lines().map(|l|l.unwrap()){
		//println!("{}",line);
		let (name, outputs) = line.split_once(">").unwrap();
		let outputs = outputs.split(",").map(|s|s.trim().to_string()).collect::<Vec<String>>();
		let (name,_) = name.split_once(" ").unwrap();
		//print one line per output
		let mut name_it = name.chars().into_iter();
		name_it.next();
		let name_it = name_it.collect::<String>();
		for o in outputs.iter() {
			println!("\t{} --> {}",if name.starts_with("&") { format!("{}{{{}}}",name_it,name_it) } else { format!("{}",name_it) }, o);
		}
		conf.add(name.to_string(),outputs);
	}	
	let mut tic = 1;
	let (mut gc,mut cm,mut xf,mut sz) = (0,0,0,0);
	while gc&cm&xf&sz == 0 {
		if let Some(v) = conf.press_button() {
//			println!("[{:>4}]Pressing the button!",tic);
			for e in v {
				match &e[..] {
					"gc" => gc = tic as i128,
					"cm" => cm = tic as i128,
					"xf" => xf = tic as i128,
					"sz" => sz = tic as i128,
					_	=> panic!("nop"),
				}
			}
		}
		tic+=1;
		if tic == 10000 {break;}
	}
	fn gcd(mut a: i128,mut b: i128) -> i128 {
		while a!=b {
			if a >b {
				a -= b;
			}else {
				b -= a;
			}
		}
		a
	}
	fn lcm(a: i128, b: i128) -> i128 {
		i128::abs(a*b)/gcd(a,b)
	}

	fn v_lcm(v: Vec<i128>) -> i128 {
		v.iter().skip(1).fold(v[0], |acc, &x| lcm(acc, x)) 
	}
	println!("{},{},{},{} --lcm--> {}", gc, cm, xf, sz,v_lcm([gc,cm,xf,sz].to_vec()));
}
