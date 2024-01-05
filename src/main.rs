use anyhow::{Context, Result};
use clap::Parser;
use log::info;
use std::fs::File;
use std::io::{self, BufReader};

fn answer() -> i32 {
    42
}

#[test]
fn check_answer_validity() {
    assert_eq!(answer(), 42);
}


#[derive(Parser)]
struct Cli {
    pattern: String,
    path: std::path::PathBuf,
}

#[derive(Debug)]
struct CustomError(String);

// implement a function to calcuate fib

fn main() -> Result<()> {
    env_logger::init();
    info!("starting up");
    // 如果有可能返回多个值，则需要将函数签名的返回值设置为Result，同时，函数结尾为Ok(())
    let args = Cli::parse();
    println!("pattern: {:?}, path: {:?}", args.pattern, args.path);
    let f = File::open(args.path).with_context(|| format!("could not read file `{}`", "test"))?; // unwrap 是match with panic!的替代
    let stdout = io::stdout(); // get the global stdout entity
    let handle = io::BufWriter::new(stdout); // optional: wrap that handle in a buffer
                                             // BufReader默认的大小是8KB,但由于是按行读取，所以不会溢出
    let reader = BufReader::new(f);
    grrs::find_matches(reader, &args.pattern, handle);
    Ok(())
}
