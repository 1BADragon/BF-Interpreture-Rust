use std::io::Read;
use std::fs::File;
use std::env;


fn main() {
    let args:Vec<_> = env::args().collect();
    if args.len() < 2 {
        println!("Usage: {} <filename>", args[0]);
        return;
    }

    let mut file = File::open(&args[1]).expect("Filed to open file");

    let mut buf:[u8; 1] = [0];

    while file.read_exact(&mut buf).is_ok() {
        println!("{}", buf[0] as char);
    }

    return;
}
