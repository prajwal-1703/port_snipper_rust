use std::env;
use std::io::{self, Write};
use std::net::{IpAddr,TcpStream};
use std::str::FromStr;
use std::process::exit;
use std::sync::mpsc ::{Sender, channel};
use std::thread;

const MAX: u16 = 65535; 
struct Arguments {
    ipaddr: IpAddr,
    threads: u32,
}

impl Arguments {
    fn new(args: &[String]) -> Result<Arguments, &'static str> {
        if args.len() < 2 {
            return Err("Not enough Arguments");
        } else if args.len() > 4 {
            return Err("Too many Arguments");
        }
        let f = args[1].clone();
        if let Ok(ipaddr) = IpAddr::from_str(&f) {
            return Ok(Arguments { ipaddr, threads: 4 });
        } else {
            let flag = args[1].clone();
            if (flag.contains("-h") || flag.contains("-help")) && args.len() == 2 {
                println!("usage: -j to select the number of threads you want
                \r\n      -h or -help to this help message");
                return Err("Help requested");
            } else if flag.contains("-h") || flag.contains("-help") {
                return Err("Too many Arguments");
            } else if flag.contains("-j") {
                if args.len() < 4 {
                    return Err("Not enough arguments for -j flag");
                }
                let threads = match args[2].parse::<u32>() {
                    Ok(s) => s,
                    Err(_) => return Err("Failed to parse the number of threads"),
                };
                let ipaddr = match IpAddr::from_str(&args[3]) {
                    Ok(s) => s,
                    Err(_) => return Err("Not a Valid Ip Address it must be IPv4 or IPv6 format"),
                };
                return Ok(Arguments { threads, ipaddr });
            } else {
                return Err("Invalid Arguments passed");
            }
        }
    }
}


fn scan(tx: Sender<u16>, start_port: u16, addr: IpAddr, num_threads: u16) {
    let mut port: u16 = start_port + 1;
    loop {
        match TcpStream::connect((addr, port)){
            Ok(_) => {
                print!(".");
                io::stdout().flush().unwrap();
                tx.send(port).unwrap();
            }
            Err(_) => {}
        }
        if (MAX - port) <= num_threads {
            break;
        } 
        port += num_threads;
    }
}


fn main() {
    let args: Vec<String> = env::args().collect();
    let program = args[0].clone();
    let arguments = Arguments::new(&args).unwrap_or_else(
        |err| {
            if err.contains("help") {
                exit(0);
            } else {
                eprintln!("{} Problem parsing the arguments: {}", program, err);
                exit(0);
            }
        }
    );
    let num_threads = arguments.threads;
    let addr = arguments.ipaddr;
    let (tx, rx) = channel();
    for i in 0 .. num_threads {
        let tx = tx.clone();
        thread::spawn(move  || {
            scan(tx, i as u16, addr, num_threads as u16);
        });
    }

    let mut out = vec![];
    drop (tx);
    for p in rx {
        out.push(p);// Close the sender to stop the loop when all threads are done
    }
    println!("");
    out.sort();
    for v in out {
        println!("Port {} is open", v);
    }
}