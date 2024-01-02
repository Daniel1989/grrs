use clap::Parser;
use std::fs::File;
use std::io::{self, BufReader, BufRead, Write};
use anyhow::{Context, Result};
use indicatif;
use log::{info, warn};

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
    let mut handle = io::BufWriter::new(stdout); // optional: wrap that handle in a buffer
    let pb = indicatif::ProgressBar::new(100);
    // BufReader默认的大小是8KB,但由于是按行读取，所以不会溢出
    let reader = BufReader::new(f);

    let mut line_count = 0;
    for line in reader.lines() {
        line_count += 1;
        let line_content = match line {
            Ok(content) => {
                content
            },
            Err(e) => {
                warn!("Can't deal with");
                panic!("Can't deal with {}, just exit here", e); 
            }
        };
        if line_content.contains(&args.pattern) {
            writeln!(handle, "line_content: {}", line_content)?; // add `?` if you care about errors here
        }
        pb.println(format!("[+] finished #{}", line_count));
        pb.inc(1);
    }
    pb.finish_with_message("done");
    Ok(())
}

