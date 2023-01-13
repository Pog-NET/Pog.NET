use std::{env::args, process::exit, fs};

fn main() {
    if args().count() != 2{
        println!("Usage: ./deployer <program>");
        exit(1);
    }
    let code = fs::read_to_string(args().nth(1).unwrap()).unwrap();
    println!("Initializing file.");
    let _main = std::fs::read_to_string("main/src/main.rs").unwrap();
    let _main = _main.replace("fn main()","fn _main()");
    let _main = _main.replace("fn _deployment(){", "fn main(){");
    let _main = _main.replace("\"_deployer_replace_me\"",format!("{:#?}",code).as_str());
    std::fs::write("./main.rs", _main).unwrap();
    println!("Building.");
    std::process::Command::new("rustc").arg("./main.rs").arg("-o").arg("out").spawn().unwrap().wait().unwrap();
    println!("Cleaning up.");
    std::process::Command::new("rm").arg("./main.rs").spawn().unwrap().wait().unwrap();
    println!("Outputted ./out")
}