use relayring::telemetry::journal_report::{self as analyze, JournalReport};
use relayring::wire::{decode, encode, validate};

#[derive(Debug)]
enum Cmd {
    Inspect { path: String },
    Validate { path: String, strict: bool },
    Report { path: String },
    Encode { input: String, output: String },
    Help,
}

fn usage() -> &'static str {
    concat!(
        "RelayRing — Lock-free append-only telemetry journal for gateways and observability agents.\n",
        "Usage:\n",
        "  relayring inspect <file.rlrg>\n",
        "  relayring validate [--strict] <file.rlrg>\n",
        "  relayring report <file.rlrg>\n",
        "  relayring encode <raw.bin> <out.rlrg>\n",
    )
}

fn parse_args() -> Result<Cmd, String> {
    let mut args = std::env::args().skip(1);
    let head = args.next().ok_or_else(|| usage().to_string())?;
    match head.as_str() {
        "inspect" => Ok(Cmd::Inspect {
            path: args.next().ok_or("missing path")?,
        }),
        "validate" => {
            let mut strict = false;
            let mut path = None;
            for a in args {
                if a == "--strict" {
                    strict = true;
                } else if path.is_none() {
                    path = Some(a);
                }
            }
            Ok(Cmd::Validate {
                path: path.ok_or("missing path")?,
                strict,
            })
        }
        "report" => Ok(Cmd::Report {
            path: args.next().ok_or("missing path")?,
        }),
        "encode" => {
            let input = args.next().ok_or("missing input")?;
            let output = args.next().ok_or("missing output")?;
            Ok(Cmd::Encode { input, output })
        }
        "-h" | "--help" | "help" => Ok(Cmd::Help),
        other if other.ends_with(".rlrg") => Ok(Cmd::Inspect { path: other.to_string() }),
        _ => Err(format!("unknown command {head}\n{}", usage())),
    }
}

fn cmd_inspect(bytes: &[u8]) -> Result<(), String> {
    let frame = decode::decode(bytes)?;
    println!("version={} sections={}", frame.version(), frame.section_count());
    for i in 0..frame.section_count() {
        if let Some(payload) = frame.payload_for(i) {
            println!("  [{i}] payload_len={}", payload.len());
        }
    }
    Ok(())
}

fn cmd_validate(bytes: &[u8], strict: bool) -> Result<(), String> {
    let issues = validate::validate_frame(bytes, strict)?;
    if issues.is_empty() {
        println!("ok: frame passes {} validation", if strict { "strict" } else { "relaxed" });
    } else {
        for msg in &issues {
            eprintln!("violation: {msg}");
        }
        return Err(format!("{} issue(s) found", issues.len()));
    }
    Ok(())
}

fn cmd_report(bytes: &[u8]) -> Result<(), String> {
    let report: JournalReport = analyze::run(bytes)?;
    println!("{report}");
    Ok(())
}

fn cmd_encode(input_path: &str, output_path: &str) -> Result<(), String> {
    let raw = std::fs::read(input_path).map_err(|e| e.to_string())?;
    let frame = decode::decode(&raw).unwrap_or_else(|_| decode::synthetic_from_payload(&raw));
    let out = encode::encode(&frame)?;
    let nbytes = out.len();
    std::fs::write(output_path, out).map_err(|e| e.to_string())?;
    println!("wrote {} bytes to {output_path}", nbytes);
    Ok(())
}

fn main() {
    match parse_args() {
        Ok(Cmd::Help) => println!("{}", usage()),
        Ok(cmd) => {
            let result = match cmd {
                Cmd::Inspect { path } => {
                    let bytes = std::fs::read(&path).expect("read input");
                    cmd_inspect(&bytes)
                }
                Cmd::Validate { path, strict } => {
                    let bytes = std::fs::read(&path).expect("read input");
                    cmd_validate(&bytes, strict)
                }
                Cmd::Report { path } => {
                    let bytes = std::fs::read(&path).expect("read input");
                    cmd_report(&bytes)
                }
                Cmd::Encode { input, output } => cmd_encode(&input, &output),
                Cmd::Help => Ok(()),
            };
            if let Err(e) = result {
                eprintln!("error: {e}");
                std::process::exit(1);
            }
        }
        Err(e) => {
            eprintln!("{e}");
            std::process::exit(1);
        }
    }
}
