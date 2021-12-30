use std::{env, process};

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 3 {
        die("Two args required; BitWarden csv path (input) and LastPass csv path (output)");
    }

    let result = bitwarden2lastpass::convert_file(args[1].as_str(), args[2].as_str());

    if let Err(e) = result {
        die(e.to_string().as_str());
    }
}

fn die(msg: &str) {
    eprintln!("Error: {}", msg);
    process::exit(1);
}
