mod arguments;
mod scan;

use std::env;
use std::process;
use std::sync::mpsc::channel;
use std::thread;
use arguments::Arguments;
use scan::scan;

const MAX_PORT: u16 = 65535;

fn main() {
    let args: Vec<String> = env::args().collect();
    let program = args[0].clone();

    let arguments = Arguments::new(&args).unwrap_or_else(
        |err| {
            if err.contains("help") {
                process::exit(0);
            } else {
                eprintln!("{} problem parsing arguments {}", program, err);
                process::exit(0);
            }
        }
    );

    let num_of_threads = arguments.threads;
    let addr = arguments.ip_addr;

    let (tx, rx) = channel();

    for i in 0..num_of_threads {
        let tx = tx.clone();

        thread::spawn(move || {
            scan(tx, i, MAX_PORT, addr, num_of_threads);
        });
    }

    let mut out: Vec<u16> = vec![];

    drop(tx);

    for p in rx {
        out.push(p);
    }

    println!("");

    out.sort();

    for v in out {
        println!("{} is open", v);
    }
}
