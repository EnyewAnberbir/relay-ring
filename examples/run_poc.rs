//! Run a `.rlrg` journal through the production workflow path.
//! Usage: cargo run --example run_poc -- path/to/input.bin

use relayring::journal_workflow;
use std::env;
use std::fs;
use std::process::ExitCode;

fn main() -> ExitCode {
    let path = match env::args().nth(1) {
        Some(p) => p,
        None => {
            eprintln!("usage: run_poc <file.bin>");
            return ExitCode::from(2);
        }
    };
    let data = match fs::read(&path) {
        Ok(d) => d,
        Err(e) => {
            eprintln!("read {path}: {e}");
            return ExitCode::from(2);
        }
    };
    match journal_workflow::process_journal_bytes(&data) {
        Ok(stats) => {
            eprintln!(
                "ok records={} appends={} seals={} payload={}",
                stats.records, stats.append_marks, stats.seal_rounds, stats.payload_bytes
            );
            ExitCode::SUCCESS
        }
        Err(e) => {
            eprintln!("err: {e}");
            ExitCode::from(1)
        }
    }
}
