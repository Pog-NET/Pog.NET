const VERBOSE: bool = false;
use enable_ansi_support::enable_ansi_support;
use rand::random;
use std::{
    env::args,
    fs::{create_dir, read_to_string, write},
    process::{exit, Command},
};
fn err(msg: &str) {
    println!("\x1b[31m\x1b[1mFatal Error.\nError message: \x1b[0m{}", msg);
    exit(1);
}
fn ok(msg: &str) {
    println!("\x1b[32m\x1b[1mOK: \x1b[0m{}", msg);
}
fn info(msg: &str) {
    if VERBOSE {
        println!("\x1b[33m\x1b[1mINFO: \x1b[0m{}", msg);
    }
}
#[cfg(windows)]
fn preprocess(file: &str, out: &str) {
    let mut preprocessed: String = "".to_string();
    let contents = read_to_string(file).unwrap();
    let mut line = 1;
    for i in contents.split("\n") {
        let p = i.split(" ");
        if p.clone().nth(0).unwrap().starts_with("%import") {
            if p.clone().collect::<Vec<_>>().len() != 2 {
                err(format!(
                    "Expected second argument to %import on line {} in file {}",
                    line, file
                )
                .as_str());
            } else if !std::path::Path::new(p.clone().nth(1).unwrap()).exists() {
                err(format!(
                    "File {} does not exist on line {} in file {}.",
                    p.clone().nth(1).unwrap(),
                    line,
                    file
                )
                .as_str());
            } else if std::path::Path::new(p.clone().nth(1).unwrap()).is_dir() {
                err(format!(
                    "{} is a directory on line {} in file {}.",
                    p.clone().nth(1).unwrap(),
                    line,
                    file
                )
                .as_str())
            }
            preprocess(p.clone().nth(1).unwrap(), p.clone().nth(1).unwrap());
            info(format!("Preprocessed: {}", p.clone().nth(1).unwrap()).as_str());
        } else {
            preprocessed += &(i.to_string() + "\n");
        }
        line += 1;
    }
    if out != "main" {
        write(
            ("build_artifacts/".to_string() + &random::<u16>().to_string()) + ".pnet",
            preprocessed.clone(),
        )
        .unwrap();
    }
    if out == "main" {
        write("build_artifacts/main.pnet", preprocessed).unwrap();
        let mut last_step: String = "".to_string();
        let paths = std::fs::read_dir("./build_artifacts").unwrap();
        for path in paths {
            if path.as_ref().unwrap().path().display().to_string() != "./build_artifacts/main.pnet"
            {
                last_step +=
                    &(read_to_string(path.unwrap().path().display().to_string()).unwrap() + "\n")
            }
        }
        write("build_artifacts/final", last_step).unwrap();
    }
}
#[cfg(not(windows))]
fn preprocess(file: &str, out: &str) {
    let mut preprocessed: String = "".to_string();
    let contents = read_to_string(file).unwrap();
    let mut line = 1;
    for i in contents.split("\n") {
        let p = i.split(" ");
        if p.clone().nth(0).unwrap().starts_with("%import") {
            if p.clone().collect::<Vec<_>>().len() != 2 {
                err(format!(
                    "Expected second argument to %import on line {} in file {}",
                    line, file
                )
                .as_str());
            } else if !std::path::Path::new(p.clone().nth(1).unwrap()).exists() {
                err(format!(
                    "File {} does not exist on line {} in file {}.",
                    p.clone().nth(1).unwrap(),
                    line,
                    file
                )
                .as_str());
            } else if std::path::Path::new(p.clone().nth(1).unwrap()).is_dir() {
                err(format!(
                    "{} is a directory on line {} in file {}.",
                    p.clone().nth(1).unwrap(),
                    line,
                    file
                )
                .as_str())
            }
            preprocess(p.clone().nth(1).unwrap(), p.clone().nth(1).unwrap());
            info(format!("Preprocessed: {}", p.clone().nth(1).unwrap()).as_str());
        } else {
            preprocessed += &(i.to_string() + "\n");
        }
        line += 1;
    }
    if out != "main" {
        write(
            ("build_artifacts/".to_string() + &random::<u16>().to_string()) + ".pnet",
            preprocessed.clone(),
        )
        .unwrap();
    }
    if out == "main" {
        write("build_artifacts/main.pnet", preprocessed).unwrap();
        let mut last_step: String = "".to_string();
        let paths = std::fs::read_dir("./build_artifacts").unwrap();
        for path in paths {
            if path.as_ref().unwrap().path().display().to_string() != "./build_artifacts/main.pnet"
            {
                last_step +=
                    &(read_to_string(path.unwrap().path().display().to_string()).unwrap() + "\n")
            }
        }
        last_step +=
            &(read_to_string("./build_artifacts/main.pnet").unwrap() + "\n");
        write("build_artifacts/final", last_step).unwrap();
    }
}
#[cfg(not(windows))]
fn build(){
    match Command::new("rm")
        .args(["-rf", "build_artifacts"])
        .spawn()
        .unwrap()
        .wait()
    {
        Err(_) => err("rm command failed"),
        Ok(_) => (),
    }
    create_dir("build_artifacts").unwrap();
    if args().nth(1).unwrap() == "run" {
        info("Preprocessing.");
        preprocess(args().nth(2).unwrap().as_str(), "main");
        info("Running");
        Command::new("pdn_exec")
            .args(["build_artifacts/final"])
            .spawn()
            .unwrap()
            .wait()
            .unwrap();
    } else if args().nth(1).unwrap() == "deploy" {
        info("Preprocessing.");
        preprocess(args().nth(2).unwrap().as_str(), "main");
        ok("Preprocessed");
        info("Deploying");
        Command::new("pdn_deploy")
            .args([
                "build_artifacts/final",
                args()
                    .nth(2)
                    .unwrap()
                    .as_str()
                    .replace(".pnet", ".out")
                    .as_str(),
            ])
            .output()
            .unwrap();
        ok("Deployed");
    }
}
#[cfg(windows)]
fn build(){
    if let Ok(_) = fs::remove_dir_all("build_artifacts") {
    } else {};
    create_dir("build_artifacts").unwrap();
    if args().nth(1).unwrap() == "run" {
        info("Preprocessing.");
        preprocess(args().nth(2).unwrap().as_str(), "main");
        info("Running");
        Command::new("pdn_exec")
            .args(["build_artifacts/final"])
            .spawn()
            .unwrap()
            .wait()
            .unwrap();
    } else if args().nth(1).unwrap() == "deploy" {
        info("Preprocessing.");
        preprocess(args().nth(2).unwrap().as_str(), "main");
        ok("Preprocessed");
        info("Deploying");
        Command::new("pdn_deploy")
            .args([
                "build_artifacts/final",
                args()
                    .nth(2)
                    .unwrap()
                    .as_str()
                    .replace(".pnet", ".out")
                    .as_str(),
            ])
            .output()
            .unwrap();
        ok("Deployed");
    }
}
fn main() {
    enable_ansi_support().unwrap();
    if args().len() != 3 {
        err(format!("Usage: {} <run/deploy> <file>", args().nth(0).unwrap()).as_str())
    } else if args().nth(1).unwrap() != "run" && args().nth(1).unwrap() != "deploy" {
        err(format!("Usage: {} <run/deploy> <file>", args().nth(0).unwrap()).as_str())
    } else if !std::path::Path::new(&args().nth(2).unwrap()).exists() {
        err(format!("File {} does not exist.", args().nth(2).unwrap()).as_str())
    } else if std::path::Path::new(&args().nth(2).unwrap()).is_dir() {
        err(format!("{} is a directory.", args().nth(2).unwrap()).as_str())
    }
    build();
}
