use std::net::{IpAddr, TcpStream};
use std::io::{self, Write};
use std::sync::mpsc::Sender;

pub fn scan(tx: Sender<u16>, starting_port: u16, max_port: u16, addr: IpAddr, num_of_threads: u16) {
    let mut port: u16 = starting_port + 1;

    loop {
        match TcpStream::connect((addr, port)) {
            Ok(_) => {
                print!(".");
                io::stdout().flush().unwrap();
                tx.send(port).unwrap()
            }
            Err(_) => {}
        }

        if (max_port - port) <= num_of_threads {
            break;
        }

        port += num_of_threads;
    }
}