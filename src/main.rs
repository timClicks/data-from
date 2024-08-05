use std::{env, io};
use std::io::Write;

mod lspci;

fn usage() -> ! {
    let mut args = env::args();
    let arg0 = args.next().unwrap();

    eprintln!("usage: {arg0} lspci");
    eprintln!("usage: lspci -vmm -nn | {arg0} --lspci");
    std::process::exit(1);
}

fn main() {
    let mut stdout =  io::stdout().lock();

    let mut args = env::args();
    let _ = args.next().unwrap();
    let Some(cmd) = args.next() else {
        usage()
    };

    let items = match cmd.as_str() {
        "--lspci" => lspci::lspci(true),
        "lspci" => lspci::lspci(false),
        _ => usage(),
    };

    for item in items {
        let _ = serde_json::to_writer(&mut stdout, &item);
        let _ = writeln!(stdout);
    }
}
