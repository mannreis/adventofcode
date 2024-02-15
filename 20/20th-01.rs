use std::io;
use std::cell::RefCell;
use std::collections::HashMap;
use std::collections::VecDeque;
use std::fmt;
use std::env;


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
			println!("{}>{:?}",name,outputs);
		}else if name.starts_with('%') {
			modtype = ModuleType::FlipFlop;
			println!("Flip-flop");
			println!("{}>{:?}",name,outputs);
			name_it.next();
		}else if name.starts_with('&') {
			modtype = ModuleType::Conjunction;
			println!("Conjuction");
			println!("{}>{:?}",name,outputs);
			name_it.next();
		}else {
			panic!("wtf");
		}
		let input_name = name_it.clone().as_str().to_string();
		for out in outputs.iter() {
			println!("\tInserting or updating {} to set as input {}",out.clone(),input_name);
			self.modules.entry(out.clone()).and_modify(|m|{m.get_mut().inputs.push(input_name.clone())}).or_insert(RefCell::new(Module::new(out.clone(),vec![input_name.clone()],Vec::<_>::new())));
		}
		
//		self.modules.entry(input_name.clone()).and_modify(|m|{m.get_mut().modtype=modtype; m.get_mut().outputs = outputs.clone()}).or_insert(RefCell::new(Module::new(input_name.clone(),Vec::<_>::new(),outputs.clone())));
		let mut module = Module::new(input_name.clone(),Vec::<_>::new(),outputs.clone());
		module.modtype = modtype.clone();
		self.modules.entry(input_name.clone()).and_modify(|m|{m.get_mut().modtype=modtype; m.get_mut().outputs = outputs.clone()}).or_insert(RefCell::new(module));
		println!("\t{:?}",self);
	}
	
	fn press_button(& self) -> (u32,u32) {
		let mut counter = (0,0);
		let mut next_stage = VecDeque::<(bool,String)>::from([(false,String::from("broadcaster"))]);

		while let Some((signal, name)) = next_stage.pop_front() {
			if signal {counter.1 += 1} else {counter.0 += 1}
			let mut module = self.modules.get(&name).unwrap().borrow_mut();
//			println!("Popped: {:?}>{:?} from {:?}\t",module.modtype,(signal,name.clone()),next_stage);
			match module.modtype {
				ModuleType::FlipFlop => {
					if !signal {
							module.state = ! module.state;
							for o in module.outputs.iter() {
								println!("{} -{}-> {}", module.name, if module.state{"high"}else{"low"}, o.clone());	
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
						println!("{} -{}-> {}", module.name, if !all{"high"}else{"low"}, o.clone());	
						next_stage.push_back((!all, o.clone()));
					}
				},
				ModuleType::Broadcaster=>{
					println!("button -low-> broadcaster");
					for o in module.outputs.iter() {
						println!("{} -{}-> {}", module.name, if module.state{"high"}else{"low"}, o.clone());	
						next_stage.push_back((signal, o.clone()));
					}
				},
			}
			//println!();
//			if counter.0+counter.1 > 10 { break; }
		}
		counter
	}
}

fn main() {
    let mut cycles = 1000;
	let args: Vec<String> = env::args().collect();
	
	if args.len() == 2 {
		cycles = args[1].parse().unwrap();	
	}

	let mut conf = Configuration::new();
	for line in io::stdin().lines().map(|l|l.unwrap()){
		println!("{}",line);
		let (name, outputs) = line.split_once(">").unwrap();
		let outputs = outputs.split(",").map(|s|s.trim().to_string()).collect::<Vec<String>>();
		let (name,_) = name.split_once(" ").unwrap();
		conf.add(name.to_string(),outputs);
	}	
	println!("{:?}",conf);
	let mut counter = (0,0);
	for tic in 1..=cycles {
		println!("[{:>4}]Pressing the button!",tic);
		let result = conf.press_button();
		counter.0 += result.0;
		counter.1 += result.1;
		println!("Result from iteration {}: {:?}",tic,counter);
	}
	println!("Result: {:?} -> {}",counter, counter.0*counter.1);
}
