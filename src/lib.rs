use anyhow::{Context, Result};
use log::warn;
use std::fs::File;
use std::io::{BufRead, BufReader};

#[test]
fn find_a_match() {
    let mut result = Vec::new();
    let f = File::open("./big.txt")
        .with_context(|| format!("could not read file `{}`", "test"))
        .unwrap(); // unwrap 是match with panic!的替代
    let reader = BufReader::new(f);
    find_matches(reader, "lorem", &mut result);
    assert_ne!(result, b"lorem\n");
}

pub fn find_matches(reader: BufReader<File>, pattern: &str, mut writer: impl std::io::Write) {
    let pb = indicatif::ProgressBar::new(100);
    let mut line_count = 0;
    for line in reader.lines() {
        line_count += 1;
        let line_content = match line {
            Ok(content) => content,
            Err(e) => {
                warn!("Can't deal with");
                panic!("Can't deal with {}, just exit here", e);
            }
        };
        if line_content.contains(pattern) {
            writeln!(writer, "line_content: {}", line_content).unwrap(); // add `?` if you care about errors here
        }
        pb.println(format!("[+] finished #{}", line_count));
        pb.inc(1);
    }
    pb.finish_with_message("done");
}
