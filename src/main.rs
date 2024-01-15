use anyhow::{Context, Result};
use clap::Parser;
use crossbeam_channel::{bounded, select, tick, Receiver};
use ctrlc;
use log::info;
use std::fs::File;
use std::io::{self, BufReader};
use std::{time::Duration};
use serde::{Deserialize, Serialize};
use confy;
use human_panic::setup_panic;
use is_terminal::IsTerminal as _;
use serde_json::json;


#[derive(Debug, Serialize, Deserialize)]
struct MyConfig {
    comfy: bool,
    foo: i64,
}

impl ::std::default::Default for MyConfig {
    fn default() -> Self { Self { comfy: false, foo: 2i64 } }
}

#[derive(Parser)]
struct Cli {
    pattern: String,
    path: std::path::PathBuf,
    #[arg(long = "json")]
    json: bool,
}

#[derive(Debug)]
struct CustomError(String);

// implement a function to calcuate fib

fn ctrl_channel() -> Result<Receiver<()>, ctrlc::Error> {
    let (sender, receiver) = bounded(100);
    ctrlc::set_handler(move || {
        println!();
        println!("user abort process");
        let _ = sender.send(());
        std::process::abort();
    })?;

    Ok(receiver)
}

fn main() -> Result<()> {
    if std::io::stdout().is_terminal() {
        println!("I'm a terminal");
    } else {
        println!("I'm not");
    }
    let cfg: MyConfig = confy::load("my_app", "hello")?;
    println!("{:#?}", cfg);
    println!("{}", cfg.comfy);
    setup_panic!();
    // panic!("custom panic message");
    env_logger::init();
    // let mut signals = Signals::new(&[SIGINT])?;
    // thread::spawn(move || {
    //     for sig in signals.forever() {
    //         println!("");
    //         println!("Received signal {:?}", sig);
    //         std::process::abort()
    //     }
    // });
    // thread::sleep(Duration::from_secs(20));
    let ctrl_c_events = ctrl_channel()?;
    let ticks = tick(Duration::from_secs(1));

    // let mut not_work: bool = true;
    info!("starting up");
    // 如果有可能返回多个值，则需要将函数签名的返回值设置为Result，同时，函数结尾为Ok(())
    let args = Cli::parse();
    if args.json {
        println!(
            "{}",
            json!({
                "type": "message",
                "content": "Hello world",
            })
        );
    } else {
        println!("Hello world");
    }
    println!("pattern: {:?}, path: {:?}", args.pattern, args.path);
    let f = File::open(args.path).with_context(|| format!("could not read file `{}`", "test"))?; // unwrap 是match with panic!的替代
    let stdout = io::stdout(); // get the global stdout entity
    let handle = io::BufWriter::new(stdout); // optional: wrap that handle in a buffer
                                             // BufReader默认的大小是8KB,但由于是按行读取，所以不会溢出
    let reader = BufReader::new(f);
    grrs::find_matches(reader, &args.pattern, handle);

    // match result {
    //     Ok(_) => {
    //         println!("Done!");
    //         std::process::exit(exitcode::OK);
    //     }
    //     Err(e) => {
    //         eprintln!("Error: {}", e);
    //         std::process::exit(exitcode::DATAERR);
    //     }
    // }

    select! {
        recv(ticks) -> _ => {
        }
        recv(ctrl_c_events) -> _ => {
            println!();
            println!("user abort process");
            std::process::abort();
        }
    }

    Ok(())
}
