use std::{env::args, fs, process::exit};
#[cfg(not(windows))]
fn read_main() -> String{
    std::fs::read_to_string("/etc/pdn/main.rs").unwrap()
}
#[cfg(windows)]
fn read_main() -> String{
    std::fs::read_to_string("/Program Files/pdn/main.rs").unwrap()
}
fn main() {
    if args().count() != 3 {
        println!("Pog.NET deployer\nUsage: pdnd <program> <out>");
        exit(1);
    }
    let code = fs::read_to_string(args().nth(1).unwrap()).unwrap();
    println!("Initializing file.");
    let _main = read_main();
    let _main = _main.replace("fn main()", "fn _main()");
    let _main = _main.replace("fn _deployment() {", "fn main() {");
    let _main = _main.replace("\"_deployer_replace_me\"", format!("{:#?}", code).as_str());
    std::fs::write("./main.rs", _main).unwrap();
    println!("Building.");
    std::process::Command::new("rustc")
        .arg("./main.rs")
        .arg("-o")
        .arg(args().nth(2).unwrap())
        .spawn()
        .unwrap()
        .wait()
        .unwrap();
    println!("Cleaning up.");
    fs::remove_file("main.rs").unwrap();
    println!("Outputted \"{}\"", args().nth(2).unwrap())
}
