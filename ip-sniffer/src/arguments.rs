use std::net::IpAddr;
use std::str::FromStr;

#[allow(dead_code)]
pub struct Arguments {
    pub flag: String,
    pub ip_addr: IpAddr,
    pub threads: u16
}

impl Arguments {
    pub fn new(args: &[String]) -> Result<Arguments, &'static str> {
        if args.len() < 2 {
            return Err("Not enough arguments");
        } else if args.len() > 4 {
            return Err("Too many arguments");
        }

        let f = args[1].clone();

        if let Ok(ip_addr) = IpAddr::from_str(&f) {
            return Ok(Arguments { flag: String::from(""), ip_addr, threads: 4 })
        } else {
            let flag = args[1].clone();

            if flag.contains("-h") || flag.contains("-help") && args.len() == 2 {
                println!("\nUsage:\n -j to select how many threads you want\r\n-h or -help to show this help message\n");

                return Err("Help");
            } else if flag.contains("-h") || flag.contains("-help") {
                return Err("Too many arguments")
            } else if flag.contains("-j") {
                let ip_addr = match IpAddr::from_str(&args[3]) {
                    Ok(s) => s,
                    Err(_) => return Err("Not a valid IPADDR, must be v4 or v6")
                };

                let threads = match args[2].parse::<u16>() {
                    Ok(s) => s,
                    Err(_) => return Err("Failed to parse thread number")
                };

                return Ok(Arguments { threads, flag, ip_addr });
            } else {
                return Err("Invalid syntax");
            }
        }
    }
}