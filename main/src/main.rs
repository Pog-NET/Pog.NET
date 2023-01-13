#[derive(Debug, Clone,PartialEq)]
pub enum Types{
    I64Type = 0,
    StringType
}
#[derive(Debug, Clone, PartialEq)]
struct StackTypes{ 
    pub selected: Types,
    pub i64type: Option<i64>,
    pub strtype: Option<String>,
}
impl StackTypes {
    pub fn new(selected: Types, i64type: Option<i64>, strtype: Option<String>) -> Self { Self { selected, i64type, strtype } }
    pub fn newi64(value: i64) -> StackTypes{
        return StackTypes::new(Types::I64Type, Some(value), None)
    }
    pub fn newstr(value: String) -> StackTypes{
        return StackTypes::new(Types::StringType, None,Some(value))
    }
}
use std::{env::{args}, str::Split, collections::{HashMap}, vec, process::exit};
#[derive(Debug,Clone)]
struct Registers{
    r0: Option<StackTypes>,
    r1: Option<StackTypes>,
    r2: Option<StackTypes>,
    r3: Option<StackTypes>,
    r4: Option<StackTypes>,
    r5: Option<StackTypes>,
    r6: Option<StackTypes>,
    r7: Option<StackTypes>,
    r8: Option<StackTypes>,
    r9: Option<StackTypes>,
}
fn get_labels(prog_split: Split<&str>) -> HashMap<String,i16>{
    let mut labels: HashMap<String,i16> = HashMap::new();
    let mut line: i16 = 0;
    for i in prog_split{
        if i.starts_with(":"){
            labels.insert(i.to_string(), line);
        }
        line+=1;
    }
    labels
}
fn set_register(name: String, value: StackTypes, mut registers: Registers) -> Registers{
    if name == "r0"{
        registers.r0 = Some(value);
    }
    else if name == "r1"{
        registers.r1 = Some(value);
    }
    else if name == "r2"{
        registers.r2 = Some(value);
    }
    else if name == "r3"{
        registers.r3 = Some(value);
    }
    else if name == "r4"{
        registers.r4 = Some(value);
    }
    else if name == "r5"{
        registers.r5 = Some(value);
    }
    else if name == "r6"{
        registers.r6 = Some(value);
    }
    else if name == "r7"{
        registers.r7 = Some(value);
    }
    else if name == "r8"{
        registers.r8 = Some(value);
    }
    else if name == "r9"{
        registers.r9 = Some(value);
    }
    else{
        panic!("No register named: {}",name);
    }
    return registers
}
fn get_register(name: String, registers: Registers) -> Option<StackTypes>{
    if name == "r0"{
        return registers.r0
    }
    else if name == "r1"{
        return registers.r1
    }
    else if name == "r2"{
        return registers.r2
    }
    else if name == "r3"{
        return registers.r3
    }
    else if name == "r4"{
        return registers.r4
    }
    else if name == "r5"{
        return registers.r5
    }
    else if name == "r6"{
        return registers.r6
    }
    else if name == "r7"{
        return registers.r7
    }
    else if name == "r8"{
        return registers.r8
    }
    else if name == "r9"{
        return registers.r9
    }
    else{
        panic!("No register named: {}",name)
    }
}

#[derive(Debug,PartialEq, Clone)]
enum ArgTypes{
    Register = 0,
    Int = 1,
    Label = 2,
    String = 3
}
#[derive(Debug, Clone)]
struct Arg{
    selected: ArgTypes,
    register: Option<String>,
    label: Option<String>,
    string: Option<String>,
    int: Option<i64>
}
fn parse_args(args: Split<&str>, stack: &mut Vec<StackTypes>) -> Vec<Arg>{
    let mut types: Vec<Arg> = vec![];
    let mut t : bool = false;
    let mut strpush: bool = false;
    let mut strbuf: String = "".to_string();
    for i in args{
        if !t{t=true;continue;}
        if strpush{
            if i == "\""{
                types.push(Arg { selected: ArgTypes::String, register: None, int: None, label: None, string: Some(strbuf[0..strbuf.len()-1].to_string())});
                strpush = false;
            }
            else{
                strbuf = strbuf.to_string() + i + " ";
            }
            continue;
        }
        if i.starts_with("#"){
            types.push(Arg { selected: ArgTypes::Int, register: None, int: Some(i.replace("#","").parse().unwrap()), label: None, string: None})
        }
        else if i.starts_with("$"){
            types.push(Arg { selected: ArgTypes::Register, register: Some(i.replace("$","")), int: None, label: None, string: None})
        }
        else if i.starts_with(":"){
            types.push(Arg { selected: ArgTypes::Label, register: None, int: None, label: Some(i.to_string()), string: None})
        }
        else if i == "!"{
            let t = stack.pop().unwrap();
            if t.selected == Types::I64Type{
                types.push(Arg { selected: ArgTypes::Int, register: None, int: Some(t.i64type.unwrap()), label: None, string: None})
            }
            else if t.selected == Types::StringType{
                types.push(Arg { selected: ArgTypes::String, register: None, int: None, label: None, string: Some(t.strtype.unwrap())})
            }
        }
        else if i == "!!"{
            let t = stack.pop().unwrap();
            let f = stack.pop().unwrap();
            if f.selected == Types::I64Type{
                types.push(Arg { selected: ArgTypes::Int, register: None, int: Some(f.i64type.unwrap()), label: None, string: None})
            }
            else if f.selected == Types::StringType{
                types.push(Arg { selected: ArgTypes::String, register: None, int: None, label: None, string: Some(f.strtype.unwrap())})
            }
            stack.push(t);
        }
        else if i == "\""{
            strpush = true
        }
        else{
            panic!("Unknown identifier: {}",i)
        }
    }
    types
}
fn count_args(args: Split<&str>) -> i32{
    let mut count = 0;
    let mut strpush: bool = false;
    for i in args{
        if strpush{
            if i == "\""{
                count += 1;
                strpush = false;
            }
            continue;
        }
        else if i == "\""{
            strpush = true
        }
        else{
            count += 1
        }
    }
    count
}
fn execute(mut stack: Vec<StackTypes>, mut registers: Registers, program: String){
    let prog_split: Split<&str> = program.split("\n");
    let labels: HashMap<String, i16> = get_labels(prog_split.clone());
    if !labels.contains_key(":main"){
        panic!("No main in program")
    }
    let mut prog_len: i32 = 0;
    for _ in prog_split.clone(){
        prog_len+=1;
    }
    let mut line_num: i32 = *labels.get(":main").unwrap() as i32;
    let mut return_stack: Vec<i32> = vec![];
    while line_num < prog_len-1{
        line_num += 1;
        let line: String = prog_split.clone().nth(line_num as usize).unwrap().replace("\t","").replace("    ", "");
        let mut line_split: Split<&str> = line.split(" ");
        let args = parse_args(line_split.clone(),&mut stack);
        let op: &str = line_split.nth(0).unwrap();
        let arg_count: i32 = count_args(line_split.clone());
        if labels.contains_key(op){
            continue;
        }
        if op == "push"{
            if arg_count != 1{
                panic!("Not enough args at line {}",line_num+1)
            }
            if args.get(0).unwrap().selected == ArgTypes::Int{
                stack.push(StackTypes::newi64(args.get(0).unwrap().int.unwrap()));
            }
            else if args.get(0).unwrap().selected == ArgTypes::String{
                stack.push(StackTypes::newstr(args.get(0).unwrap().string.clone().unwrap()));
            }
            else if args.get(0).unwrap().selected == ArgTypes::Register{
                stack.push(get_register(args.get(0).unwrap().register.clone().unwrap(), registers.clone()).unwrap())
            }
            else{
                panic!("Invalid type for push: {:#?} at line {}", args.get(0).unwrap().selected, line_num+1)
            }
        }
        else if op == "add"{
            let a: StackTypes = stack.pop().expect(format!("Stack underflow at line {}",line_num+1).as_str());
            let b: StackTypes = stack.pop().expect(format!("Stack underflow at line {}",line_num+1).as_str());
            if a.selected != Types::I64Type{
                panic!("Invalid type for add: {:#?} at line {}", a.selected, line_num+1)
            }
            if b.selected != Types::I64Type{
                panic!("Invalid type for add: {:#?} at line {}", a.selected, line_num+1)
            }
            stack.push(StackTypes::newi64(a.i64type.unwrap()+b.i64type.unwrap()));
        }
        else if op == "sub"{
            let a: StackTypes = stack.pop().expect(format!("Stack underflow at line {}",line_num+1).as_str());
            let b: StackTypes = stack.pop().expect(format!("Stack underflow at line {}",line_num+1).as_str());
            if a.selected != Types::I64Type{
                panic!("Invalid type for sub: {:#?} at line {}", a.selected, line_num+1)
            }
            if b.selected != Types::I64Type{
                panic!("Invalid type for sub: {:#?} at line {}", a.selected, line_num+1)
            }
            stack.push(StackTypes::newi64(a.i64type.unwrap()-b.i64type.unwrap()));
        }
        else if op == "mul"{
            let a: StackTypes = stack.pop().expect(format!("Stack underflow at line {}",line_num+1).as_str());
            let b: StackTypes = stack.pop().expect(format!("Stack underflow at line {}",line_num+1).as_str());
            if a.selected != Types::I64Type{
                panic!("Invalid type for mul: {:#?} at line {}", a.selected, line_num+1)
            }
            if b.selected != Types::I64Type{
                panic!("Invalid type for mul: {:#?} at line {}", a.selected, line_num+1)
            }
            stack.push(StackTypes::newi64(a.i64type.unwrap()*b.i64type.unwrap()));
        }
        else if op == "jump"{
            if arg_count != 1{
                panic!("Not enough args at line {}",line_num+1)
            }
            if args.get(0).unwrap().selected != ArgTypes::Label{
                panic!("Type: {:#?} unexpected at line {}", args.get(0).unwrap().selected, line_num+1)
            }
            if labels.contains_key(&args.get(0).unwrap().clone().label.unwrap()){
                return_stack.push(line_num);
                line_num = *labels.get(&args.get(0).unwrap().clone().label.unwrap()).unwrap() as i32;
            }
            else{
                panic!("Label: {} doesnt exist at line {}", &args.get(0).unwrap().clone().label.unwrap(), line_num+1)
            }
        }
        else if op == "je"{
            if arg_count != 1{
                panic!("Not enough args at line {}",line_num+1)
            }
            if args.get(0).unwrap().selected != ArgTypes::Label{
                panic!("Type: {:#?} unexpected at line {}", args.get(0).unwrap().selected, line_num+1)
            }
            let a: StackTypes = stack.pop().expect(format!("Stack underflow at line {}",line_num+1).as_str());
            let b: StackTypes = stack.pop().expect(format!("Stack underflow at line {}",line_num+1).as_str());
            if a == b{            
                if labels.contains_key(&args.get(0).unwrap().clone().label.unwrap()){
                    return_stack.push(line_num);
                    line_num = *labels.get(&args.get(0).unwrap().clone().label.unwrap()).unwrap() as i32;
                }
                else{
                    panic!("Label: {} doesnt exist at line {}", &args.get(0).unwrap().clone().label.unwrap(), line_num+1)
                }
            }
        }
        else if op == "jne"{
            if arg_count != 1{
                panic!("Not enough args at line {}",line_num+1)
            }
            if args.get(0).unwrap().selected != ArgTypes::Label{
                panic!("Type: {:#?} unexpected at line {}", args.get(0).unwrap().selected, line_num+1)
            }
            let a: StackTypes = stack.pop().expect(format!("Stack underflow at line {}",line_num+1).as_str());
            let b: StackTypes = stack.pop().expect(format!("Stack underflow at line {}",line_num+1).as_str());
            if a != b{            
                if labels.contains_key(&args.get(0).unwrap().clone().label.unwrap()){
                    return_stack.push(line_num);
                    line_num = *labels.get(&args.get(0).unwrap().clone().label.unwrap()).unwrap() as i32;
                }
                else{
                    panic!("Label: {} doesnt exist at line {}", &args.get(0).unwrap().clone().label.unwrap(), line_num+1)
                }
            }
        }
        else if op == "pop"{
            if arg_count != 1{
                panic!("Not enough args at line {}",line_num+1)
            }
            if args.clone().get(0).unwrap().selected != ArgTypes::Register{
                panic!("Invalid type for pop: {:#?} at line {}", args.clone().get(0).unwrap().selected, line_num+1)
            }
            registers = set_register(args.get(0).unwrap().clone().register.unwrap(), stack.pop().unwrap(), registers)
        }
        else if op == "nout"{
            if arg_count != 1{
                panic!("Not enough args at line {}",line_num+1)
            }
            if args.clone().get(0).unwrap().selected != ArgTypes::Register && args.clone().get(0).unwrap().selected != ArgTypes::Int{
                panic!("Invalid type for out: {:#?} at line {}", args.clone().get(0).unwrap().selected, line_num+1)
            }
            if args.clone().get(0).unwrap().selected == ArgTypes::Register{
                print!("{}",get_register(args.clone().get(0).unwrap().clone().register.unwrap(), registers.clone()).unwrap().i64type.unwrap())
            }
            if args.clone().get(0).unwrap().selected == ArgTypes::Int{
                print!("{}",args.clone().get(0).unwrap().clone().int.unwrap())
            }
        }
        else if op == "cout"{
            if arg_count != 1{
                panic!("Not enough args at line {}",line_num+1)
            }
            if args.clone().get(0).unwrap().selected != ArgTypes::Register && args.clone().get(0).unwrap().selected != ArgTypes::Int{
                panic!("Invalid type for out: {:#?} at line {}", args.clone().get(0).unwrap().selected, line_num+1)
            }
            if args.clone().get(0).unwrap().selected == ArgTypes::Register{
                print!("{}",get_register(args.clone().get(0).unwrap().clone().register.unwrap(), registers.clone()).unwrap().i64type.unwrap() as u8 as char)
            }
            if args.clone().get(0).unwrap().selected == ArgTypes::Int{
                print!("{}",args.clone().get(0).unwrap().clone().int.unwrap() as u8 as char)
            }
        }
        else if op == "sout"{
            if arg_count != 1{
                panic!("Not enough args at line {}",line_num+1)
            }
            if args.clone().get(0).unwrap().selected != ArgTypes::Register && args.clone().get(0).unwrap().selected != ArgTypes::String{
                panic!("Invalid type for sout: {:#?} at line {}", args.clone().get(0).unwrap().selected, line_num+1)
            }
            if args.clone().get(0).unwrap().selected == ArgTypes::Register{
                if get_register(args.clone().get(0).unwrap().clone().register.unwrap(), registers.clone()).unwrap().selected != Types::StringType{
                    panic!("Invalid type for sout: {:#?} at line {}", get_register(args.clone().get(0).unwrap().clone().register.unwrap(), registers.clone()).unwrap().selected, line_num+1)
                }
                print!("{}",get_register(args.clone().get(0).unwrap().clone().register.unwrap(), registers.clone()).unwrap().strtype.unwrap())
            }
            if args.clone().get(0).unwrap().selected == ArgTypes::String{
                print!("{}",args.clone().get(0).unwrap().clone().string.unwrap())
            }
        }
        else if op == "ret"{
            line_num = return_stack.pop().unwrap()
        }
    }
}
fn _usage() -> String{
    println!("Pog.NET\nUsage: pdn <file>");
    exit(1);
}
fn main() {
    let stack: Vec<StackTypes> = vec![];
    let registers =  Registers { r0: None, r1: None, r2: None, r3: None, r4: None, r5: None, r6: None, r7: None, r8: None, r9: None };
    let file: String = match args().nth(1){
        None => _usage(),
        Some(i) => i
    };
    let program : String = std::fs::read_to_string(file).expect("File error.");
    execute(stack, registers, program)
}
fn _deployment(){
    let program: String = "_deployer_replace_me".to_string();
    let stack: Vec<StackTypes> = vec![];
    let registers =  Registers { r0: None, r1: None, r2: None, r3: None, r4: None, r5: None, r6: None, r7: None, r8: None, r9: None };
    execute(stack, registers, program)
}